//Ce systeme va lire les info de la shared ressource map et de la hashmap des entity

//primo il retire tout les transforme des entity

//ensuite il va lire map du network
//si nouvelle enity, on la creer et on l'ajoute a la hash
//si deja dans la hash on check que le code de render sit bien le bon
//si non, on recharge la bonne sprite
//ensuite on ajoute le transform qui correspond au x, y transmi

use crate::Ball;

use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.

pub struct MapSystem;

impl<'s> System<'s> for MapSystem {
    type SystemData = (WriteStorage<'s, Transform>);

    fn run(&mut self, (mut locals): Self::SystemData) {
        println!("pass");
    }
}
