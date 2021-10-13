extern crate specs;
use crate::{
    BlocksTile, CombatStats, FacingDirection, LastMove, Map, Position, PrecisePosition, Viewshed,
    WantToMove, WantToPreciseMove, WantsToMelee, MAPHEIGHT, MAPWIDTH,
};
use specs::prelude::*;
use std::cmp::{max, min};
use std::collections::HashSet;
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
        WriteStorage<'a, BlocksTile>,
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
            blockings,
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

        //get all the entity with precise position and blocking
        let mut blocking_precises = Vec::new();

        for (entity, precise_position, _blocking) in
            (&entities, &mut precise_positions, &blockings).join()
        {
            blocking_precises.push((entity, precise_position.clone()));
        }

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
            let new_pos = PrecisePosition {
                x: new_pos_x,
                y: new_pos_y,
            };

            if !is_coliding(entity, new_pos, &map, &blocking_precises) {
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

fn is_coliding(
    entity: Entity,
    pos: PrecisePosition,
    map: &Map,
    blocking_precises: &Vec<(Entity, PrecisePosition)>,
) -> bool {
    let occuped_tiles = get_occupied_tiles(pos.x, pos.y);

    for (ox, oy) in occuped_tiles {
        let idx = map.xy_idx(ox, oy);
        if map.is_blocked(idx) {
            return true;
        }
    }

    for (other_entity, other_pos) in blocking_precises.iter() {
        if *other_entity != entity {
            if precise_position_overlapping(&pos, &other_pos) {
                return true;
            }
        }
    }

    return false;
}

fn precise_position_overlapping(pos1: &PrecisePosition, pos2: &PrecisePosition) -> bool {
    //assume a size of 1 for now
    let width = 1.;
    let height = 1.;

    // let left_x1 = (pos1.x - width / 2.);
    // let right_x1 = (pos1.x + width / 2.);
    // let top_y1 = (pos1.y + height / 2.);
    // let bottom_y1 = (pos1.y - height / 2.);

    // let left_x2 = (pos2.x - width / 2.);
    // let right_x2 = (pos2.x + width / 2.);
    // let top_y2 = (pos2.y + height / 2.);
    // let bottom_y2 = (pos2.y - height / 2.);

    let left_x1 = (pos1.x);
    let right_x1 = (pos1.x + width);
    let top_y1 = (pos1.y) + height;
    let bottom_y1 = (pos1.y);

    let left_x2 = (pos2.x);
    let right_x2 = (pos2.x + width);
    let top_y2 = (pos2.y + height);
    let bottom_y2 = (pos2.y);

    println!("pre pos {:?} {:?}", pos1, pos2);
    println!(
        "pre pos {} {} {} {} {} {} {} {}",
        left_x1, left_x2, right_x1, right_x2, top_y1, top_y2, bottom_y1, bottom_y2
    );

    if left_x1 > right_x2 || left_x2 > right_x1 || bottom_y1 > top_y2 || bottom_y2 > top_y1 {
        return false;
    } else {
        println!("colide");
        return true;
    }
}

//this only work if the square is of size 1
fn get_occupied_tiles(x: f32, y: f32) -> Vec<(i32, i32)> {
    //x,y is the center
    let width = 1.;
    let height = 1.;
    let mut occupied_tile = HashSet::new();

    occupied_tile.insert((x as i32, y as i32));

    // let left_x = (x - width / 2.) as i32;
    // let right_x = (x + width / 2.) as i32;
    // let top_y = (y + height / 2.) as i32;
    // let bottom_y = (y - height / 2.) as i32;

    let left_x = (x) as i32;
    let right_x = (x + width) as i32;
    let top_y = (y + height) as i32;
    let bottom_y = (y) as i32;

    occupied_tile.insert((left_x, top_y));
    occupied_tile.insert((left_x, bottom_y));
    occupied_tile.insert((right_x, top_y));
    occupied_tile.insert((right_x, bottom_y));

    return occupied_tile.into_iter().collect();
}
