extern crate specs;
use crate::{systems::WantToPreciseMoveSystem, PrecisePosition, WantToPreciseMove};

use super::{
    ApplyMove, BlocksTile, EntityMoved, Map, Position, RunState, Speed, Viewshed, MOVE_COST,
};
use specs::prelude::*;
use std::cmp::{max, min};

pub struct MovementSystem {}

impl<'a> System<'a> for MovementSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
        WriteStorage<'a, ApplyMove>,
        WriteStorage<'a, EntityMoved>,
        WriteStorage<'a, Viewshed>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, RunState>,
        WriteStorage<'a, Speed>,
        WriteStorage<'a, PrecisePosition>,
        WriteStorage<'a, WantToPreciseMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            mut position,
            blockers,
            entities,
            mut apply_move,
            mut moved,
            mut viewsheds,
            _player_entity,
            mut _runstate,
            mut speeds,
            precise_positions,
            mut want_to_precise_moves,
        ) = data;

        let mut to_remove = Vec::new();

        // for (entity, movement, pos, precise_pos) in
        //     (&entities, &apply_move, &position, &precise_positions).join()
        // {
        //     let (dest_x, dest_y) = map.idx_xy(movement.dest_idx as usize);

        //     let dest_delta_x = (dest_x - pos.x()) as f32;
        //     let dest_delta_y = (dest_y - pos.y()) as f32;

        //     let delta_x = 0.2f32.min(-0.2f32.min(dest_delta_x));
        //     let delta_y = 0.2f32.min(-0.2f32.min(dest_delta_y));

        //     //limite the delta to 0.2
        //     // want_to_precise_moves
        //     //     .insert(entity, WantToPreciseMove { delta_x, delta_y })
        //     //     .unwrap();

        //     if delta_x == dest_delta_x && delta_y == dest_delta_y {
        //         to_remove.push(entity);
        //     }
        // }

        // // Apply broad movement
        // for (entity, movement, pos, speed) in
        //     (&entities, &apply_move, &mut position, &mut speeds).join()
        // {
        //     if speed.move_point >= MOVE_COST {
        //         let start_idx = map.xy_idx(pos.x(), pos.y());
        //         let dest_idx = movement.dest_idx as usize;
        //         let is_blocking = blockers.get(entity);
        //         if is_blocking.is_some() {
        //             map.set_blocked(start_idx, false);
        //             map.set_blocked(dest_idx, true);
        //         }
        //         pos.moving(
        //             movement.dest_idx % map.width,
        //             movement.dest_idx / map.width,
        //             &mut map.dirty,
        //         );

        //         if let Some(vs) = viewsheds.get_mut(entity) {
        //             vs.dirty = true;
        //         }
        //         moved
        //             .insert(entity, EntityMoved {})
        //             .expect("Unable to insert");

        //         speed.move_point -= MOVE_COST;
        //     }
        // }

        for entity in to_remove.drain(..) {
            apply_move.remove(entity);
        }
    }
}
