use crate::bevy_components::{Direction2D, FPoint, Movement, ServerState};
use crate::TILE_SIZE;
use bevy::prelude::*;

pub fn movement_decision_system(
    mut commands: Commands,
    mut query_server_state: Query<(Entity, &mut Transform, &ServerState)>,
) {
    for (entity, mut transform, server_state) in query_server_state.iter_mut() {
        let translation = &mut transform.translation;
        let server_pos_x = server_state.x as f32 * TILE_SIZE;
        let server_pos_y = server_state.y as f32 * TILE_SIZE;

        //check if we need a movement
        if translation.x != server_pos_x || translation.y != server_pos_y {
            println!("new movement translation {:?}", translation);
            println!("tpos {}, {}", server_pos_x, server_pos_y);

            let direction =
                get_direction((translation.x, translation.y), (server_pos_x, server_pos_y));

            commands.entity(entity).insert(Movement {
                origin: FPoint::new(translation.x, translation.y),
                destination: FPoint::new(server_pos_x, server_pos_y),
                direction,
            });
        }
    }
}

//don't handle the diagonal
pub fn get_direction(origin: (f32, f32), destination: (f32, f32)) -> Direction2D {
    if origin.0 > destination.0 {
        return Direction2D::Left;
    } else if origin.0 < destination.0 {
        return Direction2D::Right;
    } else if origin.1 > destination.1 {
        return Direction2D::Down;
    } else if origin.1 < destination.1 {
        return Direction2D::Up;
    } else {
        return Direction2D::None;
    }
}
