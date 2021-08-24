#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod animation;
mod bevy_init;
mod network;
mod systems;
use std::sync::{Arc, Mutex};
mod bevy_components;
use wasm_bindgen::prelude::*;

extern crate specs;

pub const TILE_SIZE: f32 = 32.;

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

#[wasm_bindgen]
pub fn run() {
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
    bevy_init::bevy_init(protect_data.clone(), to_send.clone());
}
