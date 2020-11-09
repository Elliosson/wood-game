use rltk::Rltk;
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;

mod network;
mod rltk_front;
pub use rltk_front::Runstate;
mod bevy_init;
use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

extern crate specs;
use specs::prelude::*;

#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub struct Data {
    pub characters: Vec<Point>,
    pub my_uid: String,
    pub map: Vec<(u32, i32, Point, Renderable)>,
    pub info_string: String,
}

pub struct Rect {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Default)]
pub struct UiCom {
    pub inventory: bool,
    pub build: bool,
    pub interaction: bool,
    pub esc: bool,
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
    network::lauch_network(protect_data.clone(), to_send.clone());
    //rltk_init(protect_data.clone(), to_send.clone());
    bevy_init::bevy_init(protect_data.clone(), to_send.clone());
}

pub fn rltk_init(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    let context = Rltk::init_simple8x8(180 as u32, 90 as u32, "Ecosystem simulator", "resources");
    let gs = rltk_front::State {
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
