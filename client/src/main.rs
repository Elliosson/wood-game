use rltk::Rltk;
#[macro_use]
extern crate specs_derive;
mod components;
mod main_menu;
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
    assets::HotReloadBundle,
    core::transform::TransformBundle,
    derive::SystemDesc,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    shrev::{EventChannel, ReaderId},
    ui::{RenderUi, UiBundle, UiEvent},
    utils::application_root_dir,
};

use amethyst_imgui::RenderImgui;

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
            world_map: Vec::new(),
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
        .with_barrier()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(key_bindings_path)?,
        )?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderImgui::<StringBindings>::default())
                .with_plugin(RenderUi::default()),
        )?
        .with_bundle(GameBundle)?
        .with_bundle(NetworkBundle {
            protect_data,
            to_send,
        })?
        .with_bundle(HotReloadBundle::default())?
        .with_system_desc(UiEventHandlerSystemDesc::default(), "ui_event_handler", &[])
        .with_bundle(UiBundle::<StringBindings>::new())?;
    //ici on poura lancer le stat avec la map, penser a faire le buddle aussi, le bundle va initialiser les ressource

    let mut game = Application::new(resources, main_menu::MainMenu::default(), game_data)?;
    //let mut game = Application::new(resources, game::MyGame, game_data)?;
    game.run();

    Ok(())
}

/// This shows how to handle UI events. This is the same as in the 'ui' example.
#[derive(SystemDesc)]
#[system_desc(name(UiEventHandlerSystemDesc))]
pub struct UiEventHandlerSystem {
    #[system_desc(event_channel_reader)]
    reader_id: ReaderId<UiEvent>,
}

impl UiEventHandlerSystem {
    pub fn new(reader_id: ReaderId<UiEvent>) -> Self {
        Self { reader_id }
    }
}

impl<'a> System<'a> for UiEventHandlerSystem {
    type SystemData = Write<'a, EventChannel<UiEvent>>;

    fn run(&mut self, events: Self::SystemData) {
        // Reader id was just initialized above if empty
        for ev in events.read(&mut self.reader_id) {
            log::info!("[SYSTEM] You just interacted with an ui element: {:?}", ev);
        }
    }
}
