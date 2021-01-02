use crate::bevy_components::MouseLoc;
use crate::{Data, UiCom};
use bevy::input::mouse::*;
use bevy::input::*;
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub fn keyboard_intput_system(
    keyboard_input: Res<Input<KeyCode>>,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
    mut ui_com: ResMut<UiCom>,
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

    if keyboard_input.just_pressed(KeyCode::G) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "pickup"));
    }

    if keyboard_input.just_pressed(KeyCode::F) {
        ui_com.interaction = !ui_com.interaction;
    }
    if keyboard_input.just_pressed(KeyCode::B) {
        ui_com.build = !ui_com.build;
    }
    if keyboard_input.just_pressed(KeyCode::I) {
        ui_com.inventory = !ui_com.inventory;
    }
}

#[derive(Default)]
pub struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}
pub fn mouse_press_system(
    mut state: Local<State>,
    mouse_pressed_events: Res<Events<MouseButtonInput>>,
    mouse_pos: ResMut<MouseLoc>,
) {
    for event in state.mouse_button_event_reader.iter(&mouse_pressed_events) {
        if event.state == ElementState::Released {
            match event.button {
                MouseButton::Left => {
                    println!("event: {:?} position: {:?}", event, mouse_pos.0);
                }
                _ => {}
            }
        }
    }
}

pub fn mouse_movement_updating_system(
    mut mouse_pos: ResMut<MouseLoc>,
    mut state: Local<State>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        mouse_pos.0 = event.position;
    }
}
