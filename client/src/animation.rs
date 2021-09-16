use crate::bevy_components::{Direction2D, Movement};
use bevy::prelude::*;

use instant::{Duration, Instant};

pub fn move_element(
    commands: &mut Commands,
    entity: Entity,
    mut sprite: Mut<TextureAtlasSprite>,
    translation: &mut Vec3,
    mut movement: Mut<Movement>,
    now: Instant,
) {
    let new_x = movement.destination.x;
    let new_y = movement.destination.y;

    let old_x = movement.origin.x;
    let old_y = movement.origin.y;

    // if movement.counter == 4 {
    *translation = Vec3::new(new_x, new_y, translation.z);

    //update sprite and remve mouvement
    // update_sprite(movement.direction.clone(), movement.counter, &mut *sprite);

    // commands.remove::<Movement>(entity);
    commands.entity(entity).remove::<Movement>();
    // } else {
    //     let new_x = old_x + (new_x - old_x) * (movement.counter as f32 / 4.);
    //     let new_y = old_y + (new_y - old_y) * (movement.counter as f32 / 4.);
    //     *translation = Vec3::new(new_x, new_y, translation.z);

    //     //update sprite
    //     update_sprite(movement.direction.clone(), movement.counter, &mut *sprite);

    //     movement.counter += 1;
    //     movement.next_time = now + Duration::from_millis(7);
    // }
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
