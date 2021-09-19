extern crate specs;
use crate::{
    gamelog::{GameLog, GeneralLog},
    Action, Dead, DeathCause, FacingDirection, Map, Position, Speed, WantsToMelee,
};
use specs::prelude::*;

pub struct ActionSystem {}

impl<'a> System<'a> for ActionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Map>,
        WriteExpect<'a, GameLog>,
        WriteExpect<'a, GeneralLog>,
        WriteStorage<'a, Action>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, FacingDirection>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            map,
            mut log,
            mut general_log,
            mut actions,
            positions,
            directions,
            mut want_to_melees,
        ) = data;

        for (entity, action, pos, direction) in
            (&entities, &mut actions, &positions, &directions).join()
        {
            println!("action");
            if action.name == "punch" {
                println!("action punch {:?}", direction);
                let idx = map.xy_idx(
                    pos.x() + direction.front_tile.x,
                    pos.y() + direction.front_tile.y,
                );
                if let Some(contents) = map.tile_content.get(&idx) {
                    println!("action content on tile");
                    let mut want_to_melee = WantsToMelee {
                        targets: Vec::new(),
                    };
                    for target in contents {
                        println!("want to melee something");
                        want_to_melee.targets.push(target.clone())
                    }
                    want_to_melees.insert(entity, want_to_melee).unwrap();
                }

                //search all element on the front tile

                //attach a want to mele for each( probleme I can only want to melee one at a time(only mele one randomly ? only melle the blocking one ?))
                //think that this will replace the combat for now
                //ultimatly I want to be ablle to melee multiple enemy, so I should put a vector in the target of want to melee

                //basically the order is: action (punch) -> get all the entity in the front tile -> melee combat (domage and/or spaw), and then we have our thing
            }
        }
    }
}
