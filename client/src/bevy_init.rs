use super::bevy_components::{
    BuildButton, ButtonMaterials, CharacAnimation, CraftButton, Direction2D, InteractionButton,
    InventoryButton, MouseLoc, Player, Sens, ServerState, TextInfoUi, Tool,
};
use super::bevy_systems::*;
use super::Data;
use super::PlayerInfo;
use super::UiCom;
use bevy::prelude::*;

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
    let ui_com = UiCom::default();
    let mouse_loc = MouseLoc::default();
    let tool = Tool::default();

    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .add_resource(protect_data)
        .add_resource(id_to_entity)
        .add_resource(to_send)
        .add_resource(player_info)
        .add_resource(ui_com)
        .add_resource(mouse_loc)
        .add_resource(tool)
        .add_startup_system(setup.system())
        .add_system(button_system.system())
        .add_system(keyboard_intput_system.system())
        .add_system(map_system.system())
        .add_system(deserialise_player_info_system.system())
        .add_system(camera_system.system())
        .add_system(inventory_button_system.system())
        .add_system(inventory_ui_system.system())
        .add_system(inventory_item_button_system.system())
        .add_system(interaction_button_system.system())
        .add_system(interaction_ui_system.system())
        .add_system(interaction_item_button_system.system())
        .add_system(build_button_system.system())
        .add_system(build_ui_system.system())
        .add_system(build_item_button_system.system())
        .add_system(craft_button_system.system())
        .add_system(craft_ui_system.system())
        .add_system(craft_item_button_system.system())
        .add_system(animate_sprite_system.system())
        .add_system(movement_decision_system.system())
        .add_system(update_player_system.system())
        .add_system(text_info_ui_system.system())
        .add_system(mouse_press_system.system())
        .add_system(mouse_movement_updating_system.system())
        .add_system(inventory_equip_button_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("sprites/character_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 3, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let player_sprite = SpriteSheetComponents {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_translation(Vec3::new(0., 0., MAX_RENDER_PRIORITY)),
        ..Default::default()
    };
    commands
        // ui camera
        .spawn(Camera2dComponents::default())
        .spawn(UiCameraComponents::default())
        .spawn(player_sprite)
        .with(Player {})
        .with(Timer::from_seconds(0.05, true))
        .with(CharacAnimation { counter: 0 })
        .with(ServerState {
            x: 0,
            y: 0,
            gen: 0,
            id: 0,
        })
        .with(Sens {
            direction: Direction2D::Down,
        })
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
        })
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect {
                    bottom: Val::Px(10.),
                    left: Val::Px(200.),
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
        .with(InteractionButton {})
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Interaction".to_string(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                ..Default::default()
            });
        })
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect {
                    bottom: Val::Px(10.),
                    left: Val::Px(200.),
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
        .with(CraftButton {})
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Craft".to_string(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                ..Default::default()
            });
        })
        .spawn(ButtonComponents {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect {
                    bottom: Val::Px(10.),
                    left: Val::Px(200.),
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
        .with(BuildButton {})
        .with_children(|parent| {
            parent.spawn(TextComponents {
                text: Text {
                    value: "Build".to_string(),
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    style: TextStyle {
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                },
                ..Default::default()
            });
        })
        .spawn(TextComponents {
            text: Text {
                value: "TextInfoUi".to_string(),
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                style: TextStyle {
                    font_size: 20.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            },
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect {
                    bottom: Val::Px(150.),
                    left: Val::Px(10.),
                    ..Default::default()
                },
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .with(TextInfoUi {});
}
