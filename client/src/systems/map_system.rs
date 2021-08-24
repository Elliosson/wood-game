//Ce systeme va lire les info de la shared ressource map et de la hashmap des entity

//primo il retire tout les transforme des entity

//ensuite il va lire map du network
//si nouvelle enity, on la creer et on l'ajoute a la hash
//si deja dans la hash on check que le code de render sit bien le bon
//si non, on recharge la bonne sprite
//ensuite on ajoute le transform qui correspond au x, y transmi

use super::TILE_SIZE;
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

        // this hash will be use to find the entities that are no longuer in the player views
        // We copy the original hash and then remove all the entity found in the json
        // Then we delete the entitty of the leftover entry of the hash
        let mut to_delete_hash = net_hash.clone();

        for (id, gen, point, renderable) in &data_guard.map {
            if let Some(&entity) = net_hash.get(&(*id, *gen)) {
                let trans = transforms.get_mut(entity).unwrap();
                trans.set_translation_xyz(
                    point.x as f32 * TILE_SIZE,
                    point.y as f32 * TILE_SIZE,
                    0.,
                );

                to_delete_hash.remove(&(*id, *gen));
            } else {
                let new_entity = entities.create();

                net_hash.insert((*id, *gen), new_entity);
                let mut transform = Transform::default();
                transform.set_translation_xyz(
                    point.x as f32 * TILE_SIZE,
                    point.y as f32 * TILE_SIZE,
                    0.,
                );
                transforms
                    .insert(new_entity, transform)
                    .expect("Unable to insert");
                sprite_renders
                    .insert(new_entity, sprites[renderable.glyph as usize].clone())
                    .expect("Unable to insert");
            }
        }

        //delete entity than are no longer in views
        for (key, &entity) in &to_delete_hash {
            entities
                .delete(entity)
                .expect("Error, unable to delete entity");
            net_hash.remove(&key);
        }
    }
}
