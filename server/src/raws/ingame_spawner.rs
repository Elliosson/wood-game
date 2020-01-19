extern crate specs;
use crate::components::*;
use rltk::RGB;
use specs::prelude::*;

use super::{rawmaster::*, Raws};

type SpwanPropData<'a, 'b> = (
    &'b Entities<'a>,
    &'b mut WriteStorage<'a, Position>,
    &'b mut WriteStorage<'a, Renderable>,
    &'b mut WriteStorage<'a, Name>,
    &'b mut WriteStorage<'a, Item>,
    &'b mut WriteStorage<'a, Interactable>,
    &'b mut WriteStorage<'a, InteractableObject>,
    &'b mut WriteStorage<'a, Leaf>,
    &'b mut WriteStorage<'a, Tree>,
    &'b mut WriteStorage<'a, EnergyReserve>,
    &'b mut WriteStorage<'a, BlocksTile>,
    &'b mut WriteStorage<'a, Viewshed>,
    &'b mut WriteStorage<'a, Cow>,
);

//key is just a string, it's just the name of the entity
pub fn _spawn_named_prop_ingame(data: SpwanPropData, raws: &RawMaster, key: &str, pos: SpawnType) {
    let (
        entities,
        positions,
        renderables,
        names,
        items,
        interactables,
        interactable_objects,
        leafs,
        trees,
        en_res,
        block_tiles,
        viewsheds,
        cows,
    ) = data;

    println!("spawn_named_prop_ingame");
    if raws.prop_index.contains_key(key) {
        println!("key {}", key);
        let prop_template = &raws.raws.props[raws.prop_index[key]];

        let mut new_entity = entities.build_entity();

        // Spawn in the specified location
        match pos {
            SpawnType::AtPosition { x, y } => {
                new_entity = new_entity.with(Position { x, y }, positions);
            }
        }

        // Renderable
        if let Some(renderable) = &prop_template.renderable {
            let renderable_obj = Renderable {
                glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
                fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
                bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
                render_order: renderable.order,
            };
            new_entity = new_entity.with(renderable_obj, renderables);
        }

        new_entity = new_entity.with(
            Name {
                name: prop_template.name.clone(),
            },
            names,
        );

        // Interactable
        if let Some(interactable) = prop_template.interactable {
            if interactable {
                new_entity = new_entity.with(Interactable {}, interactables)
            };
        }

        // InteractableObject
        if let Some(interactable_object) = &prop_template.interactable_object {
            new_entity = new_entity.with(interactable_object.clone(), interactable_objects); //TODO comprendre pourquoi il ne fait pas comme ça( il passe par un itermediaire item_component)
        }

        if let Some(leaf) = prop_template.leaf {
            if leaf == true {
                new_entity = new_entity.with(Leaf { nutriments: 100 }, leafs); //TODO no default value
            }
        }

        if let Some(tree) = prop_template.tree {
            if tree == true {
                new_entity = new_entity.with(Tree {}, trees); //TODO no default value
            }
        }

        // EnergyReserve
        if let Some(energy_reserve) = &prop_template.energy_reserve {
            new_entity = new_entity.with(
                EnergyReserve {
                    reserve: energy_reserve.reserve,
                    max_reserve: energy_reserve.max_reserve,
                    base_consumption: energy_reserve.base_consumption,
                    hunger: Hunger::Full,
                },
                en_res,
            );
        }

        if let Some(block_tile) = prop_template.blocks_tile {
            if block_tile == true {
                new_entity = new_entity.with(BlocksTile {}, block_tiles); //TODO no default value
            }
        }

        // Viewshed
        if let Some(viewshed) = &prop_template.viewshed {
            new_entity = new_entity.with(
                Viewshed {
                    visible_tiles: Vec::new(),
                    range: viewshed.range,
                    dirty: true,
                },
                viewsheds,
            );
        }

        // Cow
        if let Some(cow) = prop_template.cow {
            if cow == true {
                new_entity = new_entity.with(Cow { life: 100 }, cows); //TODO no default value
            }
        }

        new_entity.build();

        println!("finish");
    }
}

/*

type SpwanMobData<'a, 'b> = (
    &'b Entities<'a>,
    &'b mut WriteStorage<'a, Position>,
    &'b mut WriteStorage<'a, Renderable>,
    &'b mut WriteStorage<'a, Name>,
    &'b mut WriteStorage<'a, Item>,
    &'b mut WriteStorage<'a, Interactable>,
    &'b mut WriteStorage<'a, InteractableObject>,
    &'b mut WriteStorage<'a, Leaf>,
    &'b mut WriteStorage<'a, Tree>,
);



pub fn _spawn_named_mob_ingame(
    data: SpwanMobData,
    raws: &RawMaster,
    key: &str,
    pos: SpawnType,
) {
    let (entities, positions, renderables, names, items, interactables, interactable_objects, leafs, trees) =
        data;
    if raws.mob_index.contains_key(key) {
        let mob_template = &raws.raws.mobs[raws.mob_index[key]];

        let mut eb = entities.build_entity();

        // Spawn in the specified location
        match pos {
            SpawnType::AtPosition { x, y } => {
                eb = eb.with(Position { x, y }, positions);
            }
        }

        // Renderable
        if let Some(renderable) = &prop_template.renderable {
            let renderable_obj = Renderable {
                glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
                fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
                bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
                render_order: renderable.order,
            };
            eb = eb.with(renderable_obj, renderables);
        }

        eb = eb.with(
            Name {
                name: prop_template.name.clone(),
            },
            names,
        );

        eb = eb.with(Monster{});
        if mob_template.blocks_tile {
            eb = eb.with(BlocksTile{}, monsters);
        }
        eb = eb.with(CombatStats{
            max_hp : mob_template.stats.max_hp,
            hp : mob_template.stats.hp,
            power : mob_template.stats.power,
            defense : mob_template.stats.defense
        }, );
        eb = eb.with(Viewshed{ visible_tiles : Vec::new(), range: mob_template.vision_range, dirty: true });

        return Some(eb.build());
    }
    None
}
*/
