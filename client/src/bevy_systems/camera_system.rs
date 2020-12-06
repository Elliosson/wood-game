use crate::bevy_components::{CharacAnimation, Direction2D, FPoint, Movement, Player, ServerState};
use crate::{PlayerInfo, TILE_SIZE};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use std::time::{Duration, Instant};

//todo refactor
pub fn camera_system(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    player_info: ResMut<PlayerInfo>,
    mut query_camera: Query<(&Camera, &mut Transform)>,
    mut query_player_mov: Query<(
        Entity,
        &Player,
        &mut Transform,
        &mut Movement,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    // mut query_player: Query<(Entity, &Player, &mut Transform)>,
) {
    //handle the movement of the player(to do in a separate fonction)
    //if teleport just change to coohordinate

    //if walk, update the coordinate of 1/4 of case
    //update sprite
    //if final position remove the movement, presisely place the player

    let mut new_player_position: Option<FPoint> = None;

    for (entity, _player, mut transform, mut movement, mut sprite, texture_atlas_handle) in
        query_player_mov.iter_mut()
    {
        let now = Instant::now();
        let translation = &mut transform.translation;

        if movement.next_time < now {
            //update the position

            let new_x = movement.destination.x;
            let new_y = movement.destination.y;

            let old_x = movement.origin.x;
            let old_y = movement.origin.y;

            if movement.counter == 4 {
                *translation.x_mut() = new_x;
                *translation.y_mut() = new_y;

                //update sprite and remve mouvement
                update_sprite(
                    movement.direction.clone(),
                    movement.counter,
                    &mut *sprite,
                    &texture_atlases,
                    texture_atlas_handle,
                );

                commands.remove_one::<Movement>(entity);
            } else {
                *translation.x_mut() = old_x + (new_x - old_x) * (movement.counter as f32 / 4.);
                *translation.y_mut() = old_y + (new_y - old_y) * (movement.counter as f32 / 4.);
                //update sprite
                update_sprite(
                    movement.direction.clone(),
                    movement.counter,
                    &mut *sprite,
                    &texture_atlases,
                    texture_atlas_handle,
                );

                movement.counter += 1;
                movement.next_time = now + Duration::from_millis(7);
            }
        }
        new_player_position = Some(FPoint::new(translation.x(), translation.y()));
    }

    //if there is currently a player movement, move the camera and player accordingly
    //else create the movement
    {
        for (camera, mut transform) in query_camera.iter_mut() {
            if camera.name == Some("Camera2d".to_string()) {
                if let Some(new_pos) = new_player_position.clone() {
                    let translation = &mut transform.translation;

                    *translation.x_mut() = new_pos.x;
                    *translation.y_mut() = new_pos.y;
                }
            }
        }

        // // move the player at the same time that the camera to avoid camera desync
        // for (entity, _player, mut transform) in query_player.iter_mut() {
        //     let translation = &mut transform.translation;
        //     let new_x = player_info.my_info.pos.x as f32 * TILE_SIZE;
        //     let new_y = player_info.my_info.pos.y as f32 * TILE_SIZE;
        //     if new_x != *translation.x_mut() || new_y != *translation.y_mut() {
        //         *translation.x_mut() = new_x;
        //         *translation.y_mut() = new_y;

        //         // commands.insert_one(entity, CharacAnimation { counter: 0 });
        //     }
        // }
    }
}

pub fn update_sprite(
    direction: Direction2D,
    counter: usize,
    sprite: &mut TextureAtlasSprite,
    texture_atlases: &Res<Assets<TextureAtlas>>,
    texture_atlas_handle: &Handle<TextureAtlas>,
) {
    let sprite_list;
    if direction == Direction2D::Up {
        sprite_list = [9, 10, 11]
    } else if direction == Direction2D::Down {
        sprite_list = [0, 1, 2]
    } else if direction == Direction2D::Left {
        sprite_list = [3, 4, 5]
    } else {
        sprite_list = [6, 7, 8]
    }

    println!("{:?} {:?}", direction, sprite_list);

    let sprite_numer = counter % sprite_list.len();
    println!(
        "sprite number {}, index {}",
        sprite_numer, sprite_list[sprite_numer] as u32
    );

    sprite.index = sprite_list[sprite_numer] as u32;
}
