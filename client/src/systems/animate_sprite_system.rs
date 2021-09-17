use crate::animation_and_movement::*;
use crate::bevy_components::{Movement, NonPlayer, SpriteState};
use bevy::prelude::*;

pub fn animate_sprite_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &NonPlayer,
        &mut Transform,
        &mut Movement,
        &mut SpriteState,
        &mut TextureAtlasSprite,
    )>,
) {
    for (entity, _non_player, mut transform, movement, sprite_state, sprite) in query.iter_mut() {
        let translation = &mut transform.translation;

        move_element(
            &mut commands,
            entity,
            sprite,
            translation,
            movement,
            sprite_state,
        );
    }
}
