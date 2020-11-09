use super::Data;
use super::TILE_SIZE;
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn bevy_init(protect_data: Arc<Mutex<Data>>, to_send: Arc<Mutex<Vec<String>>>) {
    App::build()
        .add_plugins(DefaultPlugins)
        .init_resource::<ButtonMaterials>()
        .add_resource(protect_data)
        .add_startup_system(setup.system())
        .add_system(button_system.system())
        .add_system(player_movement_system.system())
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

fn button_system(
    mut commands: Commands,
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
                text.value = "Press".to_string();
                *material = button_materials.pressed.clone();
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
        .spawn(SpriteComponents {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)),
            sprite: Sprite::new(Vec2::new(120.0, 30.0)),
            ..Default::default()
        })
        .with(Player {});
}

fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
) {
    for (player, mut transform) in query.iter_mut() {
        let mut direction_x = 0.0;
        let mut direction_y = 0.0;
        if keyboard_input.pressed(KeyCode::Left) {
            direction_x -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction_y += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction_y -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction_x += 1.0;
        }
        println!("dir x {} y {}", direction_x, direction_y);

        let translation = &mut transform.translation;
        // move the paddle horizontally
        *translation.x_mut() += time.delta_seconds * direction_x * 500.;
        *translation.y_mut() += time.delta_seconds * direction_y * 500.;
        // bound the paddle within the walls
        *translation.x_mut() = translation.x().min(380.0).max(-380.0);
    }
}

fn map_system(
    from_net_data: Res<Arc<Mutex<Data>>>,
    mut id_to_entity: ResMut<HashMap<(u32, i32), Entity>>,
    mut query: Query<(&Player, &mut Transform)>,
    mut entity_query: Query<Entity>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let data_guard = from_net_data.lock().unwrap();

    // this hash will be use to find the entities that are no longuer in the player views
    // We copy the original hash and then remove all the entity found in the json
    // Then we delete the entitty of the leftover entry of the hash
    let mut entities_to_delete = id_to_entity.clone();

    for (id, gen, point, renderable) in &data_guard.map {
        if let Some(&entity) = id_to_entity.get(&(*id, *gen)) {
            let mut transform = query.get_component_mut::<Transform>(entity).unwrap();
            let translation = &mut transform.translation;
            *translation.x_mut() = point.x as f32 * TILE_SIZE;
            *translation.y_mut() = point.y as f32 * TILE_SIZE;

            entities_to_delete.remove(&(*id, *gen));
        } else {
            let new_entity = commands
                .spawn(SpriteComponents {
                    material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
                    transform: Transform::from_translation(Vec3::new(
                        point.x as f32 * TILE_SIZE,
                        point.y as f32 * TILE_SIZE,
                        0.0,
                    )),
                    sprite: Sprite::new(Vec2::new(5.0, 5.0)),
                    ..Default::default()
                })
                .current_entity()
                .unwrap();

            id_to_entity.insert((*id, *gen), new_entity);
        }
    }

    //delete entity than are no longer in views
    for (key, &entity) in &entities_to_delete {
        commands.despawn(entity);

        id_to_entity.remove(&key);
    }
}
