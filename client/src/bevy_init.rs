use super::components::Renderable;
use super::Data;
use super::PlayerInfo;
use super::UiCom;
use super::TILE_SIZE;
use bevy::prelude::*;
use bevy::render::camera::Camera;
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
        .run();
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}

pub struct Player {}

pub struct InventoryButton {}
pub struct InventoryWindow {}
pub struct InventoryItemButton {
    name: String,
    index: u32,
    generation: i32,
}

fn button_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
    mut text_query: Query<&mut Text>,
) {
    for (_button, interaction, mut material, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn inventory_item_button_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(
        &Button,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
        &InventoryItemButton,
    )>,
    net_data: ResMut<Arc<Mutex<Data>>>,
    mut text_query: Query<&mut Text>,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
) {
    let mut to_send_guard = to_send.lock().unwrap();
    let data_guard = net_data.lock().unwrap();

    for (_button, interaction, mut material, children, item) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                to_send_guard.push(format!(
                    "{} {} {} {}",
                    data_guard.my_uid, "consume", item.index, item.generation
                ));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

fn inventory_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    player_info: Res<PlayerInfo>,
    mut ui_com: ResMut<UiCom>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &InventoryWindow)>,
) {
    if ui_com.inventory == true && ui_com.inventory_active == false {
        //spawn the inventory ui

        ui_com.inventory_active = true;
        let base_node = commands
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Px(500.0), Val::Px(500.0)),
                    position: Rect {
                        left: Val::Percent(0.),
                        top: Val::Percent(0.),
                        ..Default::default()
                    },
                    flex_direction: FlexDirection::Column,
                    // align_content: AlignContent::FlexStart,
                    // justify_content: JustifyContent::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    ..Default::default()
                },
                material: materials.add(Color::WHITE.into()),
                ..Default::default()
            })
            .with(InventoryWindow {});

        for item in &player_info.inventaire {
            //create a button
            base_node.with_children(|parent| {
                parent
                    .spawn(ButtonComponents {
                        style: Style {
                            margin: Rect {
                                bottom: Val::Px(10.),
                                ..Default::default()
                            },
                            size: Size::new(Val::Px(70.0), Val::Px(30.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: button_materials.normal.clone(),
                        ..Default::default()
                    })
                    .with(InventoryWindow {})
                    .with(InventoryItemButton {
                        name: item.name.clone(),
                        index: item.index,
                        generation: item.generation,
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(TextComponents {
                                text: Text {
                                    value: item.name.clone(),
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    style: TextStyle {
                                        font_size: 10.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                },
                                ..Default::default()
                            })
                            .with(InventoryWindow {});
                    });
            });
        }
    } else if ui_com.inventory == false && ui_com.inventory_active == true {
        //despawn the invetory ui
        println!("print closing windows");
        ui_com.inventory_active = false;
        let mut to_despawns: Vec<Entity> = Vec::new();
        for (entity, _inventory_windows) in query.iter_mut() {
            println!("find window");
            to_despawns.push(entity);
        }

        for to_despawn in to_despawns.drain(..) {
            println!("despawn");
            commands.despawn(to_despawn);
        }
    }
}

fn inventory_button_system(
    commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut ui_com: ResMut<UiCom>,
    mut interaction_query: Query<(
        &Button,
        &InventoryButton,
        Mutated<Interaction>,
        &mut Handle<ColorMaterial>,
        &Children,
    )>,
    mut text_query: Query<&mut Text>,
) {
    for (_button, _inventory_button, interaction, mut material, children) in
        interaction_query.iter_mut()
    {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                text.value = "Press".to_string();
                *material = button_materials.pressed.clone();
                ui_com.inventory = !ui_com.inventory;
            }
            Interaction::Hovered => {
                text.value = "Hover".to_string();
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                text.value = "Button".to_string();
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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

fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
) {
    let mut to_send_guard = to_send.lock().unwrap();
    let data_guard = net_data.lock().unwrap();

    if keyboard_input.pressed(KeyCode::Left) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "left"));
    }

    if keyboard_input.pressed(KeyCode::Right) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "right"));
    }

    if keyboard_input.pressed(KeyCode::Up) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "down")); //todo se to fix it
    }

    if keyboard_input.pressed(KeyCode::Down) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "up"));
    }

    if keyboard_input.pressed(KeyCode::G) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "pickup"));
    }
}

fn map_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    from_net_data: Res<Arc<Mutex<Data>>>,
    mut id_to_entity: ResMut<HashMap<(u32, i32), Entity>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut transform_query: Query<&mut Transform>,
) {
    let data_guard = from_net_data.lock().unwrap();

    // this hash will be use to find the entities that are no longuer in the player views
    // We copy the original hash and then remove all the entity found in the json
    // Then we delete the entitty of the leftover entry of the hash
    let mut entities_to_delete = id_to_entity.clone();

    for (id, gen, point, renderable) in &data_guard.map {
        if let Some(&entity) = id_to_entity.get(&(*id, *gen)) {
            if let Ok(mut transform) = transform_query.get_component_mut::<Transform>(entity) {
                let translation = &mut transform.translation;
                *translation.x_mut() = point.x as f32 * TILE_SIZE;
                *translation.y_mut() = point.y as f32 * TILE_SIZE;
            } else {
                print!("Bad query");
            }

            entities_to_delete.remove(&(*id, *gen));
        } else {
            // println!("new object {} {}", point.x, point.y);

            let sprit_component =
                get_sprite_component(&asset_server, renderable, &mut materials, point.x, point.y);
            let new_entity = commands.spawn(sprit_component).current_entity().unwrap();

            id_to_entity.insert((*id, *gen), new_entity);
        }
    }

    //delete entity than are no longer in views
    for (key, &entity) in &entities_to_delete {
        commands.despawn(entity);

        id_to_entity.remove(&key);
    }
}

fn get_sprite_component(
    asset_server: &Res<AssetServer>,
    renderable: &Renderable,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    x: i32,
    y: i32,
) -> SpriteComponents {
    let transform =
        Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.0));

    let texture_handle;
    if renderable.glyph == '8' as u8 {
        texture_handle = asset_server.load("sprites/tree.png");
    } else if renderable.glyph == 'M' as u8 {
        texture_handle = asset_server.load("sprites/squeletton.png");
    } else if renderable.glyph == '^' as u8 {
        texture_handle = asset_server.load("sprites/rock.png");
    } else if renderable.glyph == '*' as u8 {
        texture_handle = asset_server.load("sprites/loot.png");
    } else if renderable.glyph == 'A' as u8 {
        texture_handle = asset_server.load("sprites/purple_germ.png");
    } else if renderable.glyph == '@' as u8 {
        texture_handle = asset_server.load("sprites/character.png");
    } else {
        println!("unknown glyph {}", renderable.glyph as char);
        texture_handle = asset_server.load("sprites/unknown.png");
    }

    return SpriteComponents {
        material: materials.add(texture_handle.into()),
        transform,
        ..Default::default()
    };
}

fn deserialise_player_info_system(
    from_net_data: Res<Arc<Mutex<Data>>>,
    mut player_info: ResMut<PlayerInfo>,
) {
    let data_guard = from_net_data.lock().unwrap();

    match serde_json::from_str(&data_guard.info_string) {
        Ok(info) => {
            let temp: PlayerInfo = info;
            *player_info = temp.clone();
        }
        Err(_) => println!("unable to deserialize json"),
    }
}

fn camera_system(player_info: ResMut<PlayerInfo>, mut query: Query<(&Camera, &mut Transform)>) {
    for (camera, mut transform) in query.iter_mut() {
        if camera.name == Some("Camera2d".to_string()) {
            let translation = &mut transform.translation;

            *translation.x_mut() = player_info.my_info.pos.x as f32 * TILE_SIZE;
            *translation.y_mut() = player_info.my_info.pos.y as f32 * TILE_SIZE;
        }
    }
}
