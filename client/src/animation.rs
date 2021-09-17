use crate::bevy_components::{Direction2D, Movement, SpriteState};
use bevy::prelude::*;

pub fn move_element(
    commands: &mut Commands,
    entity: Entity,
    mut sprite: Mut<TextureAtlasSprite>,
    translation: &mut Vec3,
    movement: Mut<Movement>,
    mut sprite_state: Mut<SpriteState>,
) {
    sprite_state.counter += 1;
    if movement.direction != sprite_state.direction {
        sprite_state.direction = movement.direction.clone();
        sprite_state.counter = 0;
    }

    update_sprite(
        movement.direction.clone(),
        sprite_state.counter,
        &mut *sprite,
    );

    commands.entity(entity).remove::<Movement>();

    //move the entity
    *translation = Vec3::new(
        movement.destination.x,
        movement.destination.y,
        translation.z,
    );
}

pub fn update_sprite(direction: Direction2D, counter: usize, sprite: &mut TextureAtlasSprite) {
    let sprite_list;
    if direction == Direction2D::Up {
        sprite_list = [9, 10, 11]
    } else if direction == Direction2D::Down {
        sprite_list = [0, 1, 2]
    } else if direction == Direction2D::Left {
        sprite_list = [3, 4, 5]
    } else {
        sprite_list = [6, 7, 8]
    }

    println!("{:?} {:?}", direction, sprite_list);

    let sprite_numer = counter % sprite_list.len();
    println!(
        "sprite number {}, index {}",
        sprite_numer, sprite_list[sprite_numer] as u32
    );

    sprite.index = sprite_list[sprite_numer] as u32;
}
