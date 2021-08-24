use crate::bevy_components::{NonPlayer, ServerState};
use crate::{bevy_init::MAX_RENDER_PRIORITY, Data, Renderable, TILE_SIZE};
use bevy::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn map_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    from_net_data: Res<Arc<Mutex<Data>>>,
    mut id_to_entity: ResMut<HashMap<(u32, i32), Entity>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut server_state_query: Query<&mut ServerState>,
) {
    let data_guard = from_net_data.lock().unwrap();

    // this hash will be use to find the entities that are no longuer in the player views
    // We copy the original hash and then remove all the entity found in the json
    // Then we delete the entitty of the leftover entry of the hash
    let mut entities_to_delete = id_to_entity.clone();

    for (id, gen, point, renderable) in &data_guard.map {
        if let Some(&entity) = id_to_entity.get(&(*id, *gen)) {
            if let Ok(mut server_state) =
                server_state_query.get_component_mut::<ServerState>(entity)
            {
                if server_state.x != point.x || server_state.y != point.y {
                    println!("move to {} {}", point.x, point.y);
                    server_state.x = point.x;
                    server_state.y = point.y;
                }
            } else {
                println!("Bad ServerSate query");
            }

            entities_to_delete.remove(&(*id, *gen));
        } else {
            //create entity
            println!("new object {} {}", point.x, point.y);

            let sprit_component = get_sprite_sheet_component(
                &asset_server,
                renderable,
                point.x,
                point.y,
                renderable.render_order,
                &mut texture_atlases,
            );
            let new_entity = commands
                .spawn_bundle(sprit_component)
                .insert(ServerState {
                    x: point.x,
                    y: point.y,
                    id: *id,
                    gen: *gen,
                })
                .insert(NonPlayer {})
                .id();
            id_to_entity.insert((*id, *gen), new_entity);
        }
    }

    //delete entity than are no longer in views
    for (key, &entity) in &entities_to_delete {
        commands.entity(entity).despawn();

        id_to_entity.remove(&key);
    }
}

pub struct SheetInfo {
    pub path: &'static str, //todo not sure about the static
    pub collumns: usize,
    pub row: usize,
}

fn get_sprite_sheet_component(
    asset_server: &Res<AssetServer>,
    renderable: &Renderable,
    x: i32,
    y: i32,
    render_order: i32,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> SpriteSheetBundle {
    let transform = Transform::from_translation(Vec3::new(
        x as f32 * TILE_SIZE,
        y as f32 * TILE_SIZE,
        MAX_RENDER_PRIORITY - render_order as f32, // invert render order, because in the server, 0 is the highest priority
    ));

    let sheet_info = if renderable.glyph == '8' as u8 {
        SheetInfo {
            path: "sprites/tree32.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'M' as u8 {
        SheetInfo {
            path: "sprites/squeletton_sheet.png",
            collumns: 3,
            row: 4,
        }
    } else if renderable.glyph == '^' as u8 {
        SheetInfo {
            path: "sprites/rock.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == '*' as u8 {
        SheetInfo {
            path: "sprites/loot.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'A' as u8 {
        SheetInfo {
            path: "sprites/purple_germ.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == '@' as u8 {
        SheetInfo {
            path: "sprites/character_sheet.png",
            collumns: 3,
            row: 4,
        }
    } else if renderable.glyph == 'O' as u8 {
        SheetInfo {
            path: "sprites/ghost.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'T' as u8 {
        SheetInfo {
            path: "sprites/tree32.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'D' as u8 {
        SheetInfo {
            path: "sprites/rock.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'X' as u8 {
        SheetInfo {
            path: "sprites/wall32.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == '+' as u8 {
        SheetInfo {
            path: "sprites/door.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'S' as u8 {
        SheetInfo {
            path: "sprites/craftshop.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'G' as u8 {
        SheetInfo {
            path: "sprites/garden.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'C' as u8 {
        SheetInfo {
            path: "sprites/carrot_plant.png",
            collumns: 1,
            row: 1,
        }
    } else if renderable.glyph == 'B' as u8 {
        SheetInfo {
            path: "sprites/bed.png",
            collumns: 1,
            row: 1,
        }
    } else {
        println!("unknown glyph {}", renderable.glyph as char);
        SheetInfo {
            path: "sprites/unknown.png",
            collumns: 1,
            row: 1,
        }
    };

    let texture_handle = asset_server.load(sheet_info.path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(32.0, 32.0),
        sheet_info.collumns,
        sheet_info.row,
    );

    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    return SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform,
        ..Default::default()
    };
}
