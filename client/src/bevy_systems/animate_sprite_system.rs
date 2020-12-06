use crate::animation::*;
use crate::bevy_components::{Movement, NonPlayer};
use bevy::prelude::*;
use std::time::Instant;

pub fn animate_sprite_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &NonPlayer,
        &mut Transform,
        &mut Movement,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, _non_player, mut transform, movement, sprite) in query.iter_mut() {
        let now = Instant::now();
        let translation = &mut transform.translation;

        if movement.next_time < now {
            //update the position

            move_element(&mut commands, entity, sprite, translation, movement, now);
        }
    }
}
