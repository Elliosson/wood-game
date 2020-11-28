use crate::bevy_components::{CharacAnimation, Direction2D, Sens};
use bevy::prelude::*;

pub fn animate_sprite_system(
    mut commands: Commands,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        Entity,
        &mut CharacAnimation,
        &mut Sens,
        &mut Timer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    let mut to_removes = Vec::new();
    for (entity, mut animation, sens, timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        let sprite_list;
        if sens.direction == Direction2D::Top {
            sprite_list = [9, 10, 11]
        } else if sens.direction == Direction2D::Down {
            sprite_list = [0, 1, 2]
        } else if sens.direction == Direction2D::Left {
            sprite_list = [3, 4, 5]
        } else {
            sprite_list = [6, 7, 8]
        }

        if timer.finished {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = sprite_list[(animation.counter % sprite_list.len())] as u32;
            animation.counter += 1;

            if animation.counter > 3 {
                to_removes.push(entity);
            }
        }
    }
    for entity in to_removes.drain(..) {
        commands.remove_one::<CharacAnimation>(entity);
    }
}
// animation.counter % sprite_list.len() as u32]
