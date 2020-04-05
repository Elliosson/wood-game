use rltk::Rltk;
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod general_network;
#[cfg(target_arch = "wasm32")]
mod wasm_network;

#[cfg(not(target_arch = "wasm32"))]
mod desktop_network;

mod runstate;

use runstate::Runstate;

use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

extern crate specs;
use specs::prelude::*;
pub mod gui;

mod rltk_main;
pub use rltk_main::*;

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

fn main() {
    //Shared data between the network and the game system
    let data = Data {
        characters: vec![],
        my_uid: "".to_string(),
        map: vec![],
        info_string: "".to_string(),
    };
    let protect_data: Arc<Mutex<Data>> = Arc::new(Mutex::new(data));
    let to_send: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    lauch_network(protect_data.clone(), to_send.clone());
    rltk_init(protect_data.clone(), to_send.clone());
}

pub fn rltk_init(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    let context = Rltk::init_simple8x8(180 as u32, 90 as u32, "Ecosystem simulator", "resources");
    let gs = State {
        rectangle: Rect {
            height: 6,
            width: 2,
            x: 5,
            y: 5,
        },
        data: protect_data.clone(),
        to_send: to_send.clone(),
        ecs: World::new(),
        player_info: PlayerInfo {
            inventaire: Vec::new(),
            close_interations: Vec::new(),
            my_info: MyInfo {
                pos: Position { x: 0, y: 0 },
                hp: 0,
                max_hp: 0,
                player_log: vec![],
            },
            possible_builds: Vec::new(),
            equipement: Vec::new(),
            combat_stats: Default::default(),
        },
        runstate: Runstate::Register,
        pseudo: "".to_string(),
    };

    rltk::main_loop(context, gs);
}

#[cfg(target_arch = "wasm32")]
fn lauch_network(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    wasm_network::start_websocket(protect_data, to_send).expect("Unable to start websocket");
}

#[cfg(not(target_arch = "wasm32"))]
fn lauch_network(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    desktop_network::start_websocket(protect_data, to_send);
}
