use rltk::{Console, GameState, Rltk, VirtualKeyCode, RGB};
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod network;
use futures::executor::block_on;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::futures_0_3::spawn_local;
use wasm_timer::Delay;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
extern "C" {
    fn setInterval(closure: &Closure<FnMut()>, time: u32) -> i32;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct State {
    pub rectangle: Rect,
    pub data: Arc<Mutex<Data>>,
    pub ws: Option<WebSocket>,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let data_guard = self.data.lock().unwrap();

        let ws_clone = self.ws.clone().unwrap();
        player_input(
            data_guard.my_uid.clone(),
            ws_clone,
            ctx,
            &mut self.rectangle,
        );

        ctx.cls();

        draw_map(ctx, data_guard.map.clone());

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

pub fn player_input(uid: String, ws: WebSocket, ctx: &mut Rltk, rect: &mut Rect) {
    // Player movement

    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => {
                ws.send_with_str(&format!("{} {}", uid, "left"));
                rect.x -= 1
            }

            VirtualKeyCode::Right => {
                ws.send_with_str(&format!("{} {}", uid, "right"));
                rect.x += 1
            }
            VirtualKeyCode::Up => {
                ws.send_with_str(&format!("{} {}", uid, "up"));
                rect.y -= 1
            }
            VirtualKeyCode::Down => {
                ws.send_with_str(&format!("{} {}", uid, "down"));
                rect.y += 1
            }

            _ => {}
        },
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
    ball_x: i32,
    ball_y: i32,
    characters: Vec<Point>,
    my_uid: String,
    map: Vec<(Point, Renderable)>,
}

fn draw_map(ctx: &mut Rltk, mut map: Vec<(Point, Renderable)>) {
    map.sort_by(|a, b| b.1.render_order.cmp(&a.1.render_order));
    for (pos, render) in map.iter() {
        ctx.set(
            pos.x,
            pos.y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            render.glyph,
        );
    }
}

fn main() {
    let data = Data {
        ball_x: 10,
        ball_y: 10,
        characters: vec![],
        my_uid: "".to_string(),
        map: vec![],
    };
    let protect_data: Arc<Mutex<Data>> = Arc::new(Mutex::new(data));
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50().with_title("pong").build();
    let mut gs = State {
        rectangle: Rect {
            height: 6,
            width: 2,
            x: 5,
            y: 5,
        },
        data: protect_data.clone(),
        ws: None,
    };
    let ws = network::start_websocket(protect_data.clone());
    gs.ws = Some(ws.unwrap());
    rltk::main_loop(context, gs);
}
