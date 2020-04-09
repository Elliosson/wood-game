pub use super::gui;
pub use super::runstate::{player_input, Runstate};
use rltk::{Console, GameState, Rltk, RGB};

pub use crate::{components::*, Data, Rect};

use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

extern crate specs;
use specs::prelude::*;

#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(target_arch = "wasm32")]
pub fn consol_print(mes: String) {
    console_log!("{}", mes);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn consol_print(mes: String) {
    println!("{}", mes);
}

pub struct State {
    pub rectangle: Rect,
    pub data: Arc<Mutex<Data>>,
    pub to_send: Arc<Mutex<Vec<String>>>,
    pub player_info: PlayerInfo,
    pub runstate: Runstate,
    pub ecs: World,
    pub pseudo: String,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let data_guard = self.data.lock().unwrap();

        match serde_json::from_str(&data_guard.info_string) {
            Ok(info) => self.player_info = info,
            Err(_) => {
                consol_print("unable to deserialize json".to_string());
            }
        }

        ctx.cls();

        draw_map(ctx, data_guard.map.clone(), &self.player_info.my_info.pos);

        gui::draw_ui(ctx, &self.player_info);

        self.runstate = player_input(
            data_guard.my_uid.clone(),
            self.to_send.clone(),
            ctx,
            &self.runstate,
            &mut self.rectangle,
            &self.player_info,
            &mut self.pseudo,
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

fn draw_map(ctx: &mut Rltk, mut map: Vec<(u32, i32, Point, Renderable)>, my_pos: &Position) {
    let center_x = 30;
    let center_y = 30;
    map.sort_by(|a, b| b.3.render_order.cmp(&a.3.render_order));
    for (_id, _gen, pos, render) in map.iter() {
        let x = pos.x - my_pos.x + center_x;
        let y = pos.y - my_pos.y + center_y;
        if gui::inside_windows(x, y) {
            ctx.set(x, y, render.fg, render.bg, render.glyph);
        }
    }
}
