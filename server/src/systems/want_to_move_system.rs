extern crate specs;
use crate::{
    CombatStats, FacingDirection, LastMove, Map, Position, PrecisePosition, Viewshed, WantToMove,
    WantToPreciseMove, WantsToMelee, MAPHEIGHT, MAPWIDTH,
};
use specs::prelude::*;
use std::cmp::{max, min};
use std::time::Instant;

const MOVE_PEDIOD_MS: u128 = 90;

pub struct WantToMoveSystem {}

impl<'a> System<'a> for WantToMoveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, WantToMove>,
        WriteStorage<'a, FacingDirection>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, LastMove>,
        WriteStorage<'a, PrecisePosition>,
        WriteStorage<'a, WantToPreciseMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut map,
            mut viewshed,
            mut want_to_moves,
            mut facing_directions,
            mut wants_to_melees,
            combat_stats,
            mut last_moves,
            precise_positions,
            mut want_to_precise_moves,
        ) = data;

        //add that if precise move, calculate the closest precise move and send to the precise move systeme

        // let mut to_remove = Vec::new();

        for (entity, movement, pos, precise_pos) in (
            &entities,
            &mut want_to_moves,
            &positions,
            &precise_positions,
        )
            .join()
        {
            let dest_delta_x = movement.delta_x as f32;
            let dest_delta_y = movement.delta_y as f32;

            //limite the delta to 0.2
            let delta_x = f32::min(0.1, f32::max(-0.1, dest_delta_x));
            let delta_y = f32::min(0.1, f32::max(-0.1, dest_delta_y));

            println!(
                " {}: {}, {}: {}",
                dest_delta_x, delta_x, dest_delta_y, delta_y,
            );

            want_to_precise_moves
                .insert(entity, WantToPreciseMove { delta_x, delta_y })
                .unwrap();
        }

        // for (entity, pos, viewshed, want_to_move) in
        //     (&entities, &mut positions, &mut viewshed, &mut want_to_moves).join()
        // {
        //     //arbitrary limite to the frequency of movement for some entity, for now
        //     if let Some(last_move) = last_moves.get_mut(entity) {
        //         if let Some(last_time) = last_move.time {
        //             if last_time.elapsed().as_millis() < MOVE_PEDIOD_MS {
        //                 break;
        //             }
        //         }
        //         last_move.time = Some(Instant::now());
        //     }

        //     if let Some(facing) = facing_directions.get_mut(entity) {
        //         facing.update(want_to_move.delta_x, want_to_move.delta_y);
        //     }
        //     if pos.x() + want_to_move.delta_x < 1
        //         || pos.x() + want_to_move.delta_x > map.width - 1
        //         || pos.y() + want_to_move.delta_y < 1
        //         || pos.y() + want_to_move.delta_y > map.height - 1
        //     {
        //         break;
        //     }
        //     let destination_idx = map.xy_idx(
        //         pos.x() + want_to_move.delta_x,
        //         pos.y() + want_to_move.delta_y,
        //     );
        //     if !map.is_blocked(destination_idx) {
        //         let x = min(MAPWIDTH as i32 - 1, max(0, pos.x() + want_to_move.delta_x));
        //         let y = min(MAPHEIGHT as i32 - 1, max(0, pos.y() + want_to_move.delta_y));
        //         pos.moving(x, y, &mut map.dirty);
        //         map.set_blocked(destination_idx, true);
        //         viewshed.dirty = true;
        //     } else {
        //         //potential combat
        //         if let Some(tile_content) = map.tile_content.get(&destination_idx) {
        //             for enemy_entity in tile_content.iter() {
        //                 if let Some(_combat_stat) = combat_stats.get(*enemy_entity) {
        //                     wants_to_melees
        //                         .insert(
        //                             entity,
        //                             WantsToMelee {
        //                                 targets: vec![*enemy_entity],
        //                             },
        //                         )
        //                         .expect("Unable to insert");
        //                 }
        //             }
        //         }
        //     }
        // }
        want_to_moves.clear();
    }
}
