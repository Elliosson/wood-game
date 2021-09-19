extern crate specs;
use crate::{
    CombatStats, FacingDirection, LastMove, Map, Position, Viewshed, WantToMove, WantsToMelee,
    MAPHEIGHT, MAPWIDTH,
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
        ) = data;

        for (entity, pos, viewshed, want_to_move) in
            (&entities, &mut positions, &mut viewshed, &mut want_to_moves).join()
        {
            //arbitrary limite to the frequency of movement for some entity, for now
            if let Some(last_move) = last_moves.get_mut(entity) {
                if let Some(last_time) = last_move.time {
                    if last_time.elapsed().as_millis() < MOVE_PEDIOD_MS {
                        break;
                    }
                }
                last_move.time = Some(Instant::now());
            }

            if let Some(facing) = facing_directions.get_mut(entity) {
                facing.update(want_to_move.delta_x, want_to_move.delta_y);
            }
            if pos.x() + want_to_move.delta_x < 1
                || pos.x() + want_to_move.delta_x > map.width - 1
                || pos.y() + want_to_move.delta_y < 1
                || pos.y() + want_to_move.delta_y > map.height - 1
            {
                break;
            }
            let destination_idx = map.xy_idx(
                pos.x() + want_to_move.delta_x,
                pos.y() + want_to_move.delta_y,
            );
            if !map.is_blocked(destination_idx) {
                let x = min(MAPWIDTH as i32 - 1, max(0, pos.x() + want_to_move.delta_x));
                let y = min(MAPHEIGHT as i32 - 1, max(0, pos.y() + want_to_move.delta_y));
                pos.moving(x, y, &mut map.dirty);
                map.set_blocked(destination_idx, true);
                viewshed.dirty = true;
            } else {
                //potential combat
                if let Some(tile_content) = map.tile_content.get(&destination_idx) {
                    for enemy_entity in tile_content.iter() {
                        if let Some(_combat_stat) = combat_stats.get(*enemy_entity) {
                            wants_to_melees
                                .insert(
                                    entity,
                                    WantsToMelee {
                                        targets: vec![*enemy_entity],
                                    },
                                )
                                .expect("Unable to insert");
                        }
                    }
                }
            }
        }
        want_to_moves.clear();
    }
}
