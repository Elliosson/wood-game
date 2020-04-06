// ici on va lire les donne du network sur notre position et centrer le tou en consequant

use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (WriteStorage<'s, Transform>,);

    fn run(&mut self, (mut locals): Self::SystemData) {}
}
