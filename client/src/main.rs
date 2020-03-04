use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod network;

use std::sync::{Arc, Mutex};

use wasm_bindgen::prelude::*;

use web_sys::WebSocket;
extern crate specs;
use specs::prelude::*;
mod gui;

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<dyn FnMut()>, time: u32) -> i32;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Clone)]
pub enum Runstate {
    BaseState,
    Inventory,
    Interaction,
    Build,
}

struct State {
    pub rectangle: Rect,
    pub data: Arc<Mutex<Data>>,
    pub player_info: PlayerInfo,
    pub ws: Option<WebSocket>,
    pub runstate: Runstate,
    pub ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let data_guard = self.data.lock().unwrap();

        match serde_json::from_str(&data_guard.info_string) {
            Ok(info) => self.player_info = info,
            Err(_) => {
                console_log!("unable to deserialize json");
            }
        }

        ctx.cls();

        draw_map(ctx, data_guard.map.clone());

        let ws_clone = self.ws.clone().unwrap();
        self.runstate = player_input(
            data_guard.my_uid.clone(),
            ws_clone,
            ctx,
            &self.runstate,
            &mut self.rectangle,
            &self.player_info,
        );

        for pos in data_guard.characters.iter() {
            ctx.set(
                pos.x,
                pos.y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('X'),
            );
        }
    }
}

pub fn player_input(
    uid: String,
    ws: WebSocket,
    ctx: &mut Rltk,
    runstate: &Runstate,
    rect: &mut Rect,
    player_info: &PlayerInfo,
) -> Runstate {
    let newrunstate = match runstate {
        Runstate::BaseState => player_base_state(uid, ws, ctx, rect),
        Runstate::Interaction => player_interaction(uid, ctx, player_info, ws),
        _ => Runstate::BaseState,
    };
    newrunstate
}

pub fn player_base_state(uid: String, ws: WebSocket, ctx: &mut Rltk, rect: &mut Rect) -> Runstate {
    // Player movement
    let newrunstate;

    match ctx.key {
        None => {
            newrunstate = Runstate::BaseState;
        } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => {
                ws.send_with_str(&format!("{} {}", uid, "left"))
                    .expect("Unable to send the message");
                newrunstate = Runstate::BaseState;
                rect.x -= 1
            }

            VirtualKeyCode::Right => {
                ws.send_with_str(&format!("{} {}", uid, "right"))
                    .expect("Unable to send the message");
                newrunstate = Runstate::BaseState;
                rect.x += 1
            }
            VirtualKeyCode::Up => {
                ws.send_with_str(&format!("{} {}", uid, "up"))
                    .expect("Unable to send the message");
                newrunstate = Runstate::BaseState;
                rect.y -= 1
            }
            VirtualKeyCode::Down => {
                ws.send_with_str(&format!("{} {}", uid, "down"))
                    .expect("Unable to send the message");
                newrunstate = Runstate::BaseState;
                rect.y += 1
            }
            VirtualKeyCode::F => {
                newrunstate = Runstate::Interaction;
            }

            _ => {
                newrunstate = Runstate::BaseState;
            }
        },
    }
    newrunstate
}

pub fn player_interaction(
    uid: String,
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
    ws: WebSocket,
) -> Runstate {
    let mut newrunstate = Runstate::Interaction;

    let result = gui::show_object_interaction_choice(ctx, player_info);
    match result.0 {
        gui::InteractionMenuResult::Cancel => newrunstate = Runstate::BaseState,
        gui::InteractionMenuResult::NoResponse => {}
        gui::InteractionMenuResult::Selected => {
            let interaction_tuple = result.1.unwrap();
            let (x, y, interaction) = interaction_tuple;

            //send the response
            ws.send_with_str(&format!(
                "{} {} {} {} {} {} {}",
                uid,
                "interact",
                x,
                y,
                interaction.interaction_name,
                interaction.index,
                interaction.generation
            ))
            .expect("Unable to send the message");

            newrunstate = Runstate::BaseState;
        }
    }
    newrunstate
}

pub struct Rect {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

pub fn draw_rect(ctx: &mut Rltk, rect: &Rect) {
    for x in rect.x..rect.width + rect.x {
        for y in rect.y..rect.height + rect.y {
            ctx.set(
                x,
                y,
                RGB::named(rltk::WHITE),
                RGB::named(rltk::BLACK),
                rltk::to_cp437('#'),
            );
        }
    }
}

#[derive(Debug, Clone)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

pub struct Data {
    characters: Vec<Point>,
    my_uid: String,
    map: Vec<(Point, Renderable)>,
    info_string: String,
}

fn draw_map(ctx: &mut Rltk, mut map: Vec<(Point, Renderable)>) {
    map.sort_by(|a, b| b.1.render_order.cmp(&a.1.render_order));
    for (pos, render) in map.iter() {
        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
    }
}

fn main() {
    let data = Data {
        characters: vec![],
        my_uid: "".to_string(),
        map: vec![],
        info_string: "".to_string(),
    };
    let protect_data: Arc<Mutex<Data>> = Arc::new(Mutex::new(data));
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("game").build();
    let mut gs = State {
        rectangle: Rect {
            height: 6,
            width: 2,
            x: 5,
            y: 5,
        },
        data: protect_data.clone(),
        ws: None,
        ecs: World::new(),
        player_info: PlayerInfo {
            inventaire: Vec::new(),
            close_interations: Vec::new(),
            my_info: MyInfo {
                pos: Position { x: 0, y: 0 },
            },
        },
        runstate: Runstate::BaseState,
    };
    let ws = network::start_websocket(protect_data.clone());
    gs.ws = Some(ws.unwrap());
    rltk::main_loop(context, gs);
}
