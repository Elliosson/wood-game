// ici on va lire les donne du network sur notre position et centrer le tou en consequant

use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use crate::{Data, PlayerInfo};
use std::sync::{Arc, Mutex};

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

        for (transform, camera) in (&mut transforms, &cameras).join() {
            //TODO set camera with my position
            transform.set_translation_xyz(
                (player_info.my_info.pos.x * 10) as f32,
                (player_info.my_info.pos.y * 10) as f32,
                1.,
            );
        }
    }
}
