use crate::{Data, Renderable, TILE_SIZE};
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn map_system(
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
    } else if renderable.glyph == 'O' as u8 {
        texture_handle = asset_server.load("sprites/ghost.png");
    } else if renderable.glyph == 'T' as u8 {
        texture_handle = asset_server.load("sprites/tree.png");
    } else if renderable.glyph == 'D' as u8 {
        texture_handle = asset_server.load("sprites/rock.png");
    } else if renderable.glyph == 'X' as u8 {
        texture_handle = asset_server.load("sprites/wall.png");
    } else if renderable.glyph == '+' as u8 {
        texture_handle = asset_server.load("sprites/door.png");
    } else if renderable.glyph == 'S' as u8 {
        texture_handle = asset_server.load("sprites/craftshop.png");
    } else if renderable.glyph == 'G' as u8 {
        texture_handle = asset_server.load("sprites/garden.png");
    } else if renderable.glyph == 'C' as u8 {
        texture_handle = asset_server.load("sprites/carrot_plant.png");
    } else if renderable.glyph == 'B' as u8 {
        texture_handle = asset_server.load("sprites/bed.png");
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
