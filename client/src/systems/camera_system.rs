// ici on va lire les donne du network sur notre position et centrer le tou en consequant

use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadExpect, System, WriteStorage},
    renderer::Camera,
};

use super::TILE_SIZE;
use crate::PlayerInfo;

/// This system is responsible for moving all balls according to their speed
/// and the time passed.

pub struct CameraSystem;

impl<'s> System<'s> for CameraSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Camera>,
        ReadExpect<'s, PlayerInfo>,
    );

    fn run(&mut self, (mut transforms, cameras, player_info): Self::SystemData) {
        //I don't know if it's a good idea to deserialise here.

        for (transform, _camera) in (&mut transforms, &cameras).join() {
            //TODO set camera with my position
            transform.set_translation_xyz(
                (player_info.my_info.pos.x as f32 * TILE_SIZE),
                (player_info.my_info.pos.y as f32 * TILE_SIZE),
                1.,
            );
        }
    }
}
