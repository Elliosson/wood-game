use crate::animation::*;
use crate::bevy_components::{FPoint, Movement, Player};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use std::time::Instant;

//todo refactor
pub fn camera_system(
    mut commands: Commands,
    mut query_camera: Query<(&Camera, &mut Transform)>,
    mut query_player_mov: Query<(
        Entity,
        &Player,
        &mut Transform,
        &mut Movement,
        &mut TextureAtlasSprite,
    )>,
    // mut query_player: Query<(Entity, &Player, &mut Transform)>,
) {
    //handle the movement of the player(to do in a separate fonction)
    //if teleport just change to coohordinate

    //if walk, update the coordinate of 1/4 of case
    //update sprite
    //if final position remove the movement, presisely place the player

    let mut new_player_position: Option<FPoint> = None;

    for (entity, _player, mut transform, movement, sprite) in query_player_mov.iter_mut() {
        let now = Instant::now();
        let translation = &mut transform.translation;

        if movement.next_time < now {
            //update the position

            move_element(&mut commands, entity, sprite, translation, movement, now);
        }
        new_player_position = Some(FPoint::new(translation.x(), translation.y()));
    }

    //if there is currently a player movement, move the camera and player accordingly
    //else create the movement

    for (camera, mut transform) in query_camera.iter_mut() {
        if camera.name == Some("Camera2d".to_string()) {
            if let Some(new_pos) = new_player_position.clone() {
                let translation = &mut transform.translation;

                *translation.x_mut() = new_pos.x;
                *translation.y_mut() = new_pos.y;
            }
        }
    }
}
