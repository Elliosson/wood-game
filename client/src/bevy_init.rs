use crate::bevy_components::SpriteState;
use crate::{BuildRequests, InteractionRequests};

use super::bevy_components::{Direction2D, MouseLoc, Player, Sens, ServerState, Tool};
use super::systems::*;
use super::Data;
use super::PlayerInfo;
use super::UiState;
use crate::FakeInventory;
use crate::FakeInventoryItem;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub const MAX_RENDER_PRIORITY: f32 = 20.0;

pub fn bevy_init(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    {
        //TODO make proper register system
        let mut to_send_guard = to_send.lock().unwrap();
        to_send_guard.push(format!("register {}", "test"));
    }
    let id_to_entity: HashMap<(u32, i32), Entity> = HashMap::new();
    let player_info = PlayerInfo::default();
    let mouse_loc = MouseLoc::default();
    let tool = Tool::default();
    let ui_state = UiState::default();
    let interaction_requests = InteractionRequests::default();
    let build_request = BuildRequests::default();

    let mut fake_inventory = FakeInventory::default();

    fake_inventory.inventory.insert(
        1,
        FakeInventoryItem {
            name: "sword".to_string(),
            count: 1,
        },
    );

    let mut app = App::build();

    app.add_plugins(DefaultPlugins);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.add_plugin(EguiPlugin)
        .insert_resource(protect_data)
        .insert_resource(id_to_entity)
        .insert_resource(to_send)
        .insert_resource(player_info)
        .insert_resource(mouse_loc)
        .insert_resource(tool)
        .insert_resource(ui_state)
        .insert_resource(interaction_requests)
        .insert_resource(build_request)
        .insert_resource(fake_inventory)
        .add_startup_system(setup.system())
        .add_system(keyboard_intput_system.system())
        .add_system(map_system.system())
        .add_system(deserialise_player_info_system.system())
        .add_system(camera_player_move_system.system())
        .add_system(animate_sprite_system.system())
        .add_system(movement_decision_system.system())
        .add_system(update_player_system.system())
        .add_system(mouse_press_system.system())
        .add_system(mouse_movement_updating_system.system())
        .add_system(main_ui_system.system())
        .add_system(inventory_ui_system.system())
        .add_system(inventory_uiv2_system.system())
        .add_system(build_ui_system.system())
        .add_system(interaction_ui_system.system())
        .add_system(interaction_request_system.system())
        .add_system(build_request_system.system())
        .add_system(text_info_ui_system.system())
        .add_system(action_bar_ui_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    let texture_handle = asset_server.load("sprites/character_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player_sprite = SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(0., 0., MAX_RENDER_PRIORITY)),
        ..Default::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        // ui camera
        // .spawn(Camera2dComponents::default())
        // .spawn(UiCameraComponents::default())
        .spawn_bundle(player_sprite)
        .insert(Player {})
        .insert(Timer::from_seconds(0.05, true))
        .insert(SpriteState::default())
        .insert(ServerState::default())
        .insert(Sens {
            direction: Direction2D::Down,
        });
}
