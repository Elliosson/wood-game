use crate::bevy_components::{MouseLoc, Tool};
use crate::{Data, UiCom, TILE_SIZE};
use bevy::input::mouse::*;
use bevy::input::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
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
    if keyboard_input.just_pressed(KeyCode::E) {
        to_send_guard.push(format!("{} {}", data_guard.my_uid, "pickup"));
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
        ui_com.build = false;
        ui_com.interaction = false;
        ui_com.inventory = false;
        ui_com.craft = false;
    }
}

#[derive(Default)]
pub struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}
pub fn mouse_press_system(
    mut state: Local<State>,
    windows: Res<Windows>,
    mouse_pressed_events: Res<Events<MouseButtonInput>>,
    mouse_pos: ResMut<MouseLoc>,
    tool: ResMut<Tool>,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
    query_camera: Query<(&Camera, &Transform)>,
) {
    let mut to_send_guard = to_send.lock().unwrap();
    let data_guard = net_data.lock().unwrap();
    let uid = data_guard.my_uid.clone();

    //get the camera pos
    //todo proprement avec id connue de la main camera
    let mut camera_pos_x = 0.;
    let mut camera_pos_y = 0.;

    for (camera, transform) in query_camera.iter() {
        if camera.name == Some("Camera2d".to_string()) {
            let translation = &transform.translation;

            camera_pos_x = translation.x();
            camera_pos_y = translation.y();
        }
    }

    for event in state.mouse_button_event_reader.iter(&mouse_pressed_events) {
        if event.state == ElementState::Released {
            match event.button {
                MouseButton::Left => {
                    println!("event: {:?} position: {:?}", event, mouse_pos.0);
                    let pos = mouse_pos.0;
                    //pos is in pixel in the screen, need to be transform in equivalent in transform
                    //convert the click in tile pos

                    let coord = screen_coord_to_world_coord(
                        &windows,
                        camera_pos_x,
                        camera_pos_y,
                        pos.x(),
                        pos.y(),
                    );
                    let x = (coord.0 / TILE_SIZE) as i32;
                    let y = (coord.1 / TILE_SIZE) as i32;

                    if let Some(tool_name) = tool.name.clone() {
                        to_send_guard
                            .push(format!("{} {} {} {} {}", uid, "build", x, y, tool_name));
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn screen_coord_to_world_coord(
    windows: &Res<Windows>,
    cam_x: f32,
    cam_y: f32,
    screen_x: f32,
    screen_y: f32,
) -> (f32, f32) {
    let window = windows.get_primary().unwrap();
    let center_x = window.width() as f32 / 2.;
    let center_y = window.height() as f32 / 2.;

    let x = (screen_x - center_x) + cam_x;
    let y = (screen_y - center_y) + cam_y;
    println!("click to {} {}", x, y);

    return (x, y);
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
