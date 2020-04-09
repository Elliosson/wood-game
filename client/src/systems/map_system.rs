//Ce systeme va lire les info de la shared ressource map et de la hashmap des entity

//primo il retire tout les transforme des entity

//ensuite il va lire map du network
//si nouvelle enity, on la creer et on l'ajoute a la hash
//si deja dans la hash on check que le code de render sit bien le bon
//si non, on recharge la bonne sprite
//ensuite on ajoute le transform qui correspond au x, y transmi

use crate::Data;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Entities, Entity, ReadExpect, System, WriteExpect, WriteStorage},
    renderer::SpriteRender,
};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.

pub struct MapSystem;

impl<'s> System<'s> for MapSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Arc<Mutex<Data>>>,
        WriteExpect<'s, HashMap<(u32, i32), Entity>>,
        ReadExpect<'s, Vec<SpriteRender>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            mut sprite_renders,
            net_data,
            mut net_hash,
            sprites,
        ): Self::SystemData,
    ) {
        let data_guard = net_data.lock().unwrap();

        // clear les transform de inmap, for new clear all transform

        //transforms.clear();

        //lis la map de net data

        //TODO add something to supress compoment
        for (id, gen, point, _renderable) in &data_guard.map {
            if let Some(&entity) = net_hash.get(&(*id, *gen)) {
                println!("know entity");

                let trans = transforms.get_mut(entity).unwrap();
                trans.set_translation_xyz(point.x as f32 * 10., point.y as f32 * 10., 0.);

            /*let mut transform = Transform::default();
            transform.set_translation_xyz(500., 500., 0.);
            transforms
                .insert(entity, transform)
                .expect("Unable to insert");*/
            } else {
                println!("new entity");
                let new_entity = entities.create();

                net_hash.insert((*id, *gen), new_entity);
                let mut transform = Transform::default();
                transform.set_translation_xyz(point.x as f32 * 10., point.y as f32 * 10., 0.);
                transforms
                    .insert(new_entity, transform)
                    .expect("Unable to insert");
                sprite_renders
                    .insert(new_entity, sprites[0].clone())
                    .expect("Unable to insert");
            }
            println!("point {}", point.x);
        }

        //-si dans la hash ajoute la transform
        //-sinon creer l'entity et l'ajoute dans la hash
    }
}
