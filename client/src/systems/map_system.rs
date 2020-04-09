//Ce systeme va lire les info de la shared ressource map et de la hashmap des entity

//primo il retire tout les transforme des entity

//ensuite il va lire map du network
//si nouvelle enity, on la creer et on l'ajoute a la hash
//si deja dans la hash on check que le code de render sit bien le bon
//si non, on recharge la bonne sprite
//ensuite on ajoute le transform qui correspond au x, y transmi

use crate::{Ball, Data, InMap};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use amethyst::{
    assets::{AssetStorage, Loader},
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{
        Entities, Entity, Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteExpect,
        WriteStorage,
    },
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.

pub struct MapSystem;

impl<'s> System<'s> for MapSystem {
    type SystemData = (
        Entities<'s>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Ball>,
        WriteStorage<'s, InMap>,
        WriteStorage<'s, SpriteRender>,
        ReadExpect<'s, Arc<Mutex<Data>>>,
        WriteExpect<'s, HashMap<(u32, i32), Entity>>,
        WriteExpect<'s, Loader>,
        WriteExpect<'s, AssetStorage<Texture>>,
        WriteExpect<'s, AssetStorage<SpriteSheet>>,
        ReadExpect<'s, Vec<SpriteRender>>,
    );

    fn run(
        &mut self,
        (
            entities,
            mut transforms,
            balls,
            mut in_maps,
            mut sprite_renders,
            net_data,
            mut net_hash,
            loader,
            texture_storage,
            sheet_storage,
            sprites,
        ): Self::SystemData,
    ) {
        let data_guard = net_data.lock().unwrap();

        // clear les transform de inmap, for new clear all transform

        //transforms.clear();

        //lis la map de net data

        //TODO add something to supress compoment
        for (id, gen, point, renderable) in &data_guard.map {
            if let Some(&entity) = net_hash.get(&(*id, *gen)) {
                println!("know entity");

                let trans = transforms.get_mut(entity).unwrap();
                trans.set_translation_xyz(501., 501., 0.);

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
                transform.set_translation_xyz(500., 500., 0.);
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
