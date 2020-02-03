extern crate specs;
use crate::{
    algo::*, Animal, ApplyMove, Carnivore, Cow, GoOnTarget, Leaf, Map, Point, Position, RunState,
    Specie, TargetReached, TargetedForEat, Viewshed, WantToEat, WantsToFlee,
};
use specs::prelude::*;
extern crate rltk;
use std::collections::HashMap;
//use std::time::{Duration, Instant};

pub struct OmnivoreAI {}

impl<'a> System<'a> for OmnivoreAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Cow>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Leaf>,
        WriteStorage<'a, WantToEat>,
        WriteStorage<'a, ApplyMove>,
        WriteStorage<'a, TargetedForEat>,
        WriteStorage<'a, GoOnTarget>,
        WriteStorage<'a, TargetReached>,
        WriteStorage<'a, Specie>,
        WriteStorage<'a, Animal>,
        WriteStorage<'a, Carnivore>,
        WriteStorage<'a, WantsToFlee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            _runstate,
            entities,
            viewsheds,
            cows,
            mut positions,
            leafs,
            mut want_to_eats,
            mut apply_move,
            mut targeted_eats,
            mut go_targets,
            target_reacheds,
            species,
            animals,
            carnivores,
            mut flees,
        ) = data;

        targeted_eats.clear(); //TODO dirty, create a system specificaly to clear this.

        //check if we managed to get a target
        for (entity, _animal, _pos, _carnivore, _cow) in
            (&entities, &animals, &mut positions, &carnivores, &cows).join()
        {
            println!("in");
            if let Some(reached) = target_reacheds.get(entity) {
                println!("send want to eat");
                //TODO for now it eat directly I must add a fight
                want_to_eats
                    .insert(
                        entity,
                        WantToEat {
                            target: reached.target,
                        },
                    )
                    .expect("Unable to insert");

            //TODO do not search a new target if the entity is already eating
            } else {
                println!("no target reached");
            }
        }

        //Chose the food to go
        //first try to have his favorite food
        for (entity, viewshed, _animal, carnivore, cow) in
            (&entities, &viewsheds, &animals, &carnivores, &cows).join()
        {
            //search for every possible food in the viewshed, and divide them acording to their categorie
            let mut found_leaf: Vec<Entity> = Vec::new();
            let mut found_other_specie: Vec<Entity> = Vec::new();
            let mut found_same_specie: Vec<Entity> = Vec::new();
            let my_specie = species.get(entity).unwrap();

            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                for maybe_food in map.tile_content[idx].iter() {
                    if let Some(_leaf) = leafs.get(*maybe_food) {
                        found_leaf.push(*maybe_food);
                    }
                    if let Some(specie) = species.get(*maybe_food) {
                        if specie.name == my_specie.name {
                            found_same_specie.push(*maybe_food);
                        } else {
                            found_other_specie.push(*maybe_food);
                        }
                    }
                }
            }

            //Choose if the animal prefere to go for vegetable or meat
            //TODO  add hunger conditon before going for the non prefered food
            if cow.digestion > carnivore.digestion {
                if !choose_food(
                    found_leaf,
                    entity,
                    &mut positions,
                    &mut targeted_eats,
                    &mut go_targets,
                ) {
                    choose_food(
                        found_other_specie,
                        entity,
                        &mut positions,
                        &mut targeted_eats,
                        &mut go_targets,
                    );
                }
            } else {
                if !choose_food(
                    found_other_specie,
                    entity,
                    &mut positions,
                    &mut targeted_eats,
                    &mut go_targets,
                ) {
                    choose_food(
                        found_leaf,
                        entity,
                        &mut positions,
                        &mut targeted_eats,
                        &mut go_targets,
                    );
                }
            }
        }

        //check someone want to eat us
        for (entity, _animal, _pos, _carnivore, _cow) in
            (&entities, &animals, &mut positions, &carnivores, &cows).join()
        {
            if let Some(targeted) = targeted_eats.get(entity) {
                //For now just flee if someone want to eat us
                let mut flee_list = Vec::new();
                flee_list.push(map.xy_idx(targeted.predator_pos.x, targeted.predator_pos.y) as i32);

                flees
                    .insert(entity, WantsToFlee { indices: flee_list })
                    .expect("Unable to insert");
            }
        }
    }
}

//In a list of possible food, choose the closest that is not taken by someone closer to the food
//return true if a food have been choosen
fn choose_food<'a>(
    found_foods: Vec<Entity>,
    entity: Entity,
    positions: &mut WriteStorage<'a, Position>,
    targeted_eats: &mut WriteStorage<'a, TargetedForEat>,
    go_targets: &mut WriteStorage<'a, GoOnTarget>,
) -> bool {
    let mut ret = false;
    let mut choosen_food: Option<Entity> = None;
    let mut min: f32 = std::f32::MAX;
    let pos = positions.get(entity).unwrap();
    for food in found_foods {
        let food_pos = positions.get(food).unwrap();
        let maybe_targeted_eat = targeted_eats.get(food);

        //if their is a other creature that want the target, then I only go if I am closer
        let mut competitor_distance = std::f32::MAX;
        if let Some(targeted) = maybe_targeted_eat {
            competitor_distance = targeted.distance;
        }
        let distance = rltk::DistanceAlg::Pythagoras
            .distance2d(Point::new(pos.x, pos.y), Point::new(food_pos.x, food_pos.y));
        if (distance < min) && (distance < competitor_distance) {
            choosen_food = Some(food);
            min = distance;
        }
    }
    if let Some(food) = choosen_food {
        targeted_eats
            .insert(
                food,
                TargetedForEat {
                    //TODO add a flee if we are target to eat
                    predator: entity,
                    distance: min,
                    predator_pos: Point::new(pos.x, pos.y),
                },
            )
            .expect("Unable ot insert");
        go_targets
            .insert(entity, GoOnTarget { target: food })
            .expect("Unable to insert");
        ret = true;
    }
    return ret;
}
