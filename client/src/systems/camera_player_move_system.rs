use crate::animation_and_movement::*;
use crate::bevy_components::{FPoint, Movement, Player, SpriteState};
use bevy::prelude::*;
use bevy::render::camera::Camera;

//todo refactor
pub fn camera_player_move_system(
    mut commands: Commands,
    mut queries: QuerySet<(
        //todo refactor, separate into 2 system ?
        Query<(&Camera, &mut Transform)>,
        Query<(
            Entity,
            &Player,
            &mut Transform,
            &mut Movement,
            &mut SpriteState,
            &mut TextureAtlasSprite,
        )>,
    )>,
) {
    //handle the movement of the player(to do in a separate fonction)

    let mut new_player_position: Option<FPoint> = None;

    {
        let query_player_mov = queries.q1_mut();

        for (entity, _player, mut transform, movement, sprite_state, sprite) in
            query_player_mov.iter_mut()
        {
            let translation = &mut transform.translation;

            move_element(
                &mut commands,
                entity,
                sprite,
                translation,
                movement,
                sprite_state,
            );

            new_player_position = Some(FPoint::new(translation.x, translation.y));
        }
    }

    {
        let query_camera = queries.q0_mut();

        //if there is currently a player movement, move the camera and player accordingly
        //else create the movement

        for (camera, mut transform) in query_camera.iter_mut() {
            if camera.name == Some("Camera2d".to_string()) {
                if let Some(new_pos) = new_player_position.clone() {
                    let translation = &mut transform.translation;

                    *translation = Vec3::new(new_pos.x, new_pos.y, translation.z);
                }
            }
        }
    }
}
