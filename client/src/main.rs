use rltk::Rltk;
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;

mod network;
mod rltk_front;
pub use rltk_front::Runstate;

use std::sync::{Arc, Mutex};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

extern crate specs;
use specs::prelude::*;

mod bundle;
mod game;
mod systems;
use crate::bundle::{GameBundle, NetworkBundle};

#[cfg(target_arch = "wasm32")]
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

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
    amethyst_init(protect_data.clone(), to_send.clone()).expect("Fail in amethyst_init");
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

pub struct Ball {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for Ball {
    type Storage = DenseVecStorage<Self>;
}

pub struct InMap {
    pub velocity: [f32; 2],
    pub radius: f32,
}

impl Component for InMap {
    type Storage = DenseVecStorage<Self>;
}

pub struct Uuid {
    pub string: String,
}

impl Component for Uuid {
    type Storage = DenseVecStorage<Self>;
}

pub fn amethyst_init(
    protect_data: Arc<Mutex<Data>>,
    to_send: Arc<Mutex<Vec<String>>>,
) -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let key_bindings_path = app_root.join("resources/input.ron");

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_bundle(GameBundle)?
        .with_bundle(NetworkBundle {
            protect_data,
            to_send,
        })?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?;
    //ici on poura lancer le stat avec la map, penser a faire le buddle aussi, le bundle va initialiser les ressource

    let mut game = Application::new(resources, game::MyGame, game_data)?;
    game.run();

    Ok(())
}
