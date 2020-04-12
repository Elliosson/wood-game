use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::Entity,
    input::{is_close_requested, is_key_down},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{UiCreator, UiEvent, UiEventType, UiFinder, UiText},
    window::ScreenDimensions,
    winit::VirtualKeyCode,
};

use std::sync::{Arc, Mutex};

use crate::game::MyGame;

const BUTTON_START: &str = "start";
const BUTTON_EDITABLE: &str = "editable";
const BUTTON_OPTIONS: &str = "options";
const BUTTON_CREDITS: &str = "credits";

#[derive(Default, Debug)]
pub struct MainMenu {
    ui_root: Option<Entity>,
    button_start: Option<Entity>,
    button_load: Option<Entity>,
    button_options: Option<Entity>,
    button_credits: Option<Entity>,
}

impl SimpleState for MainMenu {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // create UI from prefab and save the reference.
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        init_sprites(world, &sprites, &dimensions);

        self.ui_root =
            Some(world.exec(|mut creator: UiCreator<'_>| creator.create("menu.ron", ())));
    }
    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_start.is_none()
            || self.button_load.is_none()
            || self.button_options.is_none()
            || self.button_credits.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_start = ui_finder.find(BUTTON_START);
                self.button_load = ui_finder.find(BUTTON_EDITABLE);
                self.button_options = ui_finder.find(BUTTON_OPTIONS);
                self.button_credits = ui_finder.find(BUTTON_CREDITS);
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let world = data.world;
        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] Switching back to WelcomeScreen!");
                    Trans::Switch(Box::new(MyGame))
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_credits {
                    log::info!("[Trans::Switch] Switching to CreditsScreen!");
                    return Trans::Switch(Box::new(MyGame));
                }
                if Some(target) == self.button_start {
                    log::info!("[Trans::Switch] Switching to Game!");
                    return Trans::Switch(Box::new(MyGame));
                }
                if Some(target) == self.button_load || Some(target) == self.button_options {
                    log::info!("This Buttons functionality is not yet implemented!");
                }
                Trans::None
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::ValueCommit,
                target,
            }) => {
                if Some(target) == self.button_load {
                    let text_uis = world.read_storage::<UiText>();
                    let my_text = text_uis.get(target).unwrap();
                    log::info!("Pseudo is {}", my_text.text);
                    // send speudo info
                    let to_send = world.fetch::<Arc<Mutex<Vec<String>>>>();
                    let mut to_send_guard = to_send.lock().unwrap();
                    to_send_guard.push(format!("register {}", my_text.text));
                    return Trans::Switch(Box::new(MyGame));
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }
    fn on_stop(&mut self, data: StateData<GameData>) {
        // after destroying the current UI, invalidate references as well (makes things cleaner)
        if let Some(root_entity) = self.ui_root {
            data.world
                .delete_entity(root_entity)
                .expect("Failed to remove MainMenu");
        }

        self.ui_root = None;
        self.button_start = None;
        self.button_load = None;
        self.button_options = None;
        self.button_credits = None;
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    // Center the camera in the middle of the screen, and let it cover
    // the entire screen
    let mut transform = Transform::default();
    transform.set_translation_xyz(500., 500., 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/roguelikeSheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/roguelikeSheet.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    (0..300)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn init_sprites(world: &mut World, sprites: &[SpriteRender], _dimensions: &ScreenDimensions) {
    //store the sprites
    world.insert(sprites.to_vec());
}
