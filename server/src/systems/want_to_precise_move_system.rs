extern crate specs;
use crate::{
    CombatStats, FacingDirection, LastMove, Map, Position, PrecisePosition, Viewshed, WantToMove,
    WantToPreciseMove, WantsToMelee, MAPHEIGHT, MAPWIDTH,
};
use specs::prelude::*;
use std::cmp::{max, min};
use std::f32::*;
use std::time::Instant;

const MOVE_PEDIOD_MS: u128 = 90;

pub struct WantToPreciseMoveSystem {}

impl<'a> System<'a> for WantToPreciseMoveSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, PrecisePosition>,
        WriteExpect<'a, Map>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, WantToMove>,
        WriteStorage<'a, WantToPreciseMove>,
        WriteStorage<'a, FacingDirection>,
        WriteStorage<'a, WantsToMelee>,
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, LastMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut precise_positions,
            mut map,
            mut viewshed,
            mut want_to_moves,
            mut want_to_precise_moves,
            mut facing_directions,
            mut wants_to_melees,
            combat_stats,
            mut last_moves,
        ) = data;

        //iterate on all pmove
        //for each calculate new position
        //check normale colision
        //check player colision
        //if ok, move to new pos
        // block the 4 blocking case(expect if precisely on one case)
        //-> refactor the blocking system
        //also set the clasical position acordingly
        // it's a little simplistic, but for now it's should work

        for (entity, pos, prec_pos, movement) in (
            &entities,
            &mut positions,
            &mut precise_positions,
            &mut want_to_precise_moves,
        )
            .join()
        {
            let new_pos_x = prec_pos.x + movement.delta_x;
            let new_pos_y = prec_pos.y + movement.delta_y;

            if !is_coliding(entity, new_pos_x, new_pos_y) {
                //do the move
                prec_pos.x = new_pos_x;
                prec_pos.y = new_pos_y;

                if let Some(facing) = facing_directions.get_mut(entity) {
                    facing.update(
                        (movement.delta_x * 1000.) as i32,
                        (movement.delta_y * 1000.) as i32,
                    ); // todo, total shit, to refactor and add somewhere genric, also solve this movement thing( only let precise movement ?)
                }

                println!("{:?}", prec_pos);

                let (x, y) = getGrossPosition(prec_pos);
                pos.moving(x, y, &mut map.dirty) // todo, I am not sure of this
            }
        }
        want_to_precise_moves.clear();
    }
}

fn getGrossPosition(prec_pos: &PrecisePosition) -> (i32, i32) {
    let x = round_closest(prec_pos.x);
    let y = round_closest(prec_pos.y);

    return (x, y);
}

fn round_closest(num: f32) -> i32 {
    let sign = if num < 0. { -1 } else { 1 };

    let num = num.abs();

    let res = if num <= num.floor() + 0.5 {
        num.floor() as i32
    } else {
        num.ceil() as i32
    };

    return res * sign;
}

fn is_coliding(entity: Entity, x: f32, y: f32) -> bool {
    //todo, no colision check for now
    // collision with other precise entity

    //colision with gross entity
    // - get all the square that the new one will occupy
    // - if any is occupied, return false
    // occupied_tiles = get_occupied_tile(entity, x, y);
    // for tile in occupied_tile {
    //     if map.is_blocking() && blocking[tile] != entity {
    //         return False;
    //     }
    // }
    return false;

    // get the middle tile
    //check the 9 adj tile
    //get all entity
    // for each, check collision
    //if collision return false
}
