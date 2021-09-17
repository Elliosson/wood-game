use crate::bevy_components::{Direction2D, FPoint, IPoint, Movement, MovementKind, ServerState};
use crate::TILE_SIZE;
use bevy::prelude::*;
use instant::Instant;

// Compare the position of the entity with the server position and choose
//the apropriate movement to execute

pub fn movement_decision_system(
    mut commands: Commands,
    mut query_server_state: Query<(Entity, &mut Transform, &ServerState)>,
    query_movements: Query<(Entity, &Movement)>,
) {
    //if there is currently a player movement, move the camera and player accordingly
    //else create the movement

    for (entity, mut transform, server_state) in query_server_state.iter_mut() {
        // if let Ok(movement) = query_movements.get_component::<Movement>(entity) {
        //     if movement.tdestination.x == server_state.x
        //         && movement.tdestination.y == server_state.y
        //     {
        //         //ok
        //         continue;
        //     } else {
        //         //remove the movement and make a classic movement dessision
        //         println!("Movement interupted");
        //         commands.entity(entity).remove::<Movement>();
        //     }
        // }

        let translation = &mut transform.translation;
        let server_pos_x = server_state.x as f32 * TILE_SIZE;
        let server_pos_y = server_state.y as f32 * TILE_SIZE;

        //todo, not realy good for original position
        // let tpos_x = (translation.x / TILE_SIZE);
        // let tpos_y = (translation.y / TILE_SIZE);

        // println!("tposx {}, sposx{}", tpos_x, server_state.x);

        //check if we need a movement
        if translation.x != server_pos_x || translation.y != server_pos_y {
            println!("new movement translation {:?}", translation);
            println!("tpos {}, {}", server_pos_x, server_pos_y);

            //deside if we must teleport of move
            // let distance =
            //     (translation.x - server_pos_x).abs() + (translation.y - server_pos_y).abs();

            // if distance > 2. * TILE_SIZE {
            //todo put 1 instead when the isue is resolved
            //teleport

            let direction =
                get_direction((translation.x, translation.y), (server_pos_x, server_pos_y));

            commands.entity(entity).insert(Movement {
                origin: FPoint::new(translation.x, translation.y),
                destination: FPoint::new(server_pos_x, server_pos_y),
                tdestination: IPoint::new(server_state.x as i32, server_state.y as i32),
                direction,
                kind: MovementKind::Walk,
                counter: 0,
                next_time: Instant::now(),
            });

            *translation = Vec3::new(server_pos_x, server_pos_y, translation.z);
            // } else {
            //     println!(
            //         "insert walking movement from {} {} to {} {}",
            //         translation.x, translation.y, server_pos_x, server_pos_y
            //     );
            //     //walking movement
            //     let direction =
            //         get_direction((translation.x, translation.y), (server_pos_x, server_pos_y));
            //     println!("direction {:?}", direction);

            //     commands.entity(entity).insert(Movement {
            //         origin: FPoint::new(translation.x, translation.y),
            //         destination: FPoint::new(server_pos_x, server_pos_y),
            //         tdestination: IPoint::new(server_state.x, server_state.y),
            //         direction,
            //         kind: MovementKind::Walk,
            //         counter: 0,
            //         next_time: Instant::now(),
            //     });
            // }
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
