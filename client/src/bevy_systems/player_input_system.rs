use crate::{
    bevy_components::{MouseLoc, Tool},
    PlayerInfo,
};

use crate::bevy_components::ServerState;
use crate::{Data, UiCom, TILE_SIZE};
use bevy::input::mouse::*;
use bevy::input::*;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use std::sync::{Arc, Mutex, MutexGuard};

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
    server_state_query: Query<(Entity, &ServerState)>,
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

                    if in_alowed_zone(pos, &windows) {
                        //pos is in pixel in the screen, need to be transform in equivalent in transform
                        //convert the click in tile pos

                        let coord = screen_coord_to_world_coord(
                            &windows,
                            camera_pos_x,
                            camera_pos_y,
                            pos.x(),
                            pos.y(),
                        );
                        let x = ((coord.0 + TILE_SIZE / 2.) / TILE_SIZE) as i32; //get the tile coordinate, need to offset by half the tile
                        let y = ((coord.1 + TILE_SIZE / 2.) / TILE_SIZE) as i32;

                        if let Some(tool_name) = tool.name.clone() {
                            send_command(
                                &mut to_send_guard,
                                &uid,
                                x,
                                y,
                                tool_name,
                                &server_state_query,
                            );
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

pub fn send_command(
    to_send_guard: &mut MutexGuard<Vec<String>>,
    uid: &String,
    x: i32,
    y: i32,
    tool_name: String,
    server_state_query: &Query<(Entity, &ServerState)>,
) {
    //todo Here I will need to have a tab that link tool to action

    //get the object form the x, y position

    if tool_name == "axe" || tool_name == "WoodenSpear" {
        //search an entity on position
        for (entity, server_state) in server_state_query.iter() {
            if server_state.x == x && server_state.y == y {
                let index = server_state.id;
                let generation = server_state.gen;

                //send to system that get the first building on position
                // geting the good thing to cut will be problematique
                // en plus je viens de creer un truc qui permet de detruire n'importe quel truc, lol
                //I think I should just send the order to the server that will deduce if it's ok to cut something
                to_send_guard.push(format!(
                    "{} {} {} {} {} {} {}",
                    uid,
                    "interact",
                    x, // click position
                    y,
                    "chop_tree",
                    index,
                    generation
                ));
            }
        }
    } else {
        to_send_guard.push(format!("{} {} {} {} {}", uid, "build", x, y, tool_name));
    }
}

pub fn in_alowed_zone(pos: Vec2, windows: &Res<Windows>) -> bool {
    let window = windows.get_primary().unwrap();
    //for now allowed zone is everything above 50
    if pos.y() < 50. {
        return false;
    }
    return true;
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
