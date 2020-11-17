#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod bevy_init;
mod bevy_systems;
mod network;
mod rltk_front;
use std::sync::{Arc, Mutex};
mod bevy_components;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

extern crate specs;

pub const TILE_SIZE: f32 = 16.;

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

#[derive(Debug, Clone, Default)]
pub struct UiCom {
    pub inventory: bool,
    pub inventory_active: bool,
    pub build: bool,
    pub interaction: bool,
    pub interaction_active: bool,
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
    //rltk_front::rltk_init(protect_data.clone(), to_send.clone());
    bevy_init::bevy_init(protect_data.clone(), to_send.clone());
}
