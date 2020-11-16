use super::bevy_components::{ButtonMaterials, InventoryButton};
use super::bevy_systems::*;
use super::Data;
use super::PlayerInfo;
use super::UiCom;
use bevy::prelude::*;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn bevy_init(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    {
        //TODO make proper register system
        let mut to_send_guard = to_send.lock().unwrap();
        to_send_guard.push(format!("register {}", "test"));
    }
    let id_to_entity: HashMap<(u32, i32), Entity> = HashMap::new();
    let player_info = PlayerInfo::default();
    let ui_com = UiCom::default();

    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .add_resource(protect_data)
        .add_resource(id_to_entity)
        .add_resource(to_send)
        .add_resource(player_info)
        .add_resource(ui_com)
        .add_startup_system(setup.system())
        .add_system(button_system.system())
        .add_system(player_movement_system.system())
        .add_system(map_system.system())
        .add_system(deserialise_player_info_system.system())
        .add_system(camera_system.system())
        .add_system(inventory_button_system.system())
        .add_system(inventory_ui_system.system())
        .add_system(inventory_item_button_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        // ui camera
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect {
                    bottom: Val::Px(10.),
                    ..Default::default()
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .with(InventoryButton {})
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Inventory".to_string(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                ..Default::default()
            });
        });
}
