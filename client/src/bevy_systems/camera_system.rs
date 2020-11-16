use crate::{PlayerInfo, TILE_SIZE};
use bevy::prelude::*;
use bevy::render::camera::Camera;

pub fn camera_system(player_info: ResMut<PlayerInfo>, mut query: Query<(&Camera, &mut Transform)>) {
    for (camera, mut transform) in query.iter_mut() {
        if camera.name == Some("Camera2d".to_string()) {
            let translation = &mut transform.translation;

            *translation.x_mut() = player_info.my_info.pos.x as f32 * TILE_SIZE;
            *translation.y_mut() = player_info.my_info.pos.y as f32 * TILE_SIZE;
        }
    }
}
