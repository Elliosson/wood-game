use crate::Data;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
) {
    let mut to_send_guard = to_send.lock().unwrap();
    let data_guard = net_data.lock().unwrap();

    if keyboard_input.pressed(KeyCode::Left) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "left"));
    }

    if keyboard_input.pressed(KeyCode::Right) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "right"));
    }

    if keyboard_input.pressed(KeyCode::Up) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "down")); //todo se to fix it
    }

    if keyboard_input.pressed(KeyCode::Down) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "up"));
    }

    if keyboard_input.pressed(KeyCode::G) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "pickup"));
    }
}
