use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadExpect, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::Data;
use std::sync::{Arc, Mutex};

/// This system is responsible for moving all the paddles according to the user
/// provided input.

pub struct InputSystem;

impl<'s> System<'s> for InputSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, Arc<Mutex<Vec<String>>>>,
        ReadExpect<'s, Arc<Mutex<Data>>>,
    );

    fn run(&mut self, (mut transforms, time, input, to_send, data): Self::SystemData) {
        let mut to_send_guard = to_send.lock().unwrap();
        let data_guard = data.lock().unwrap();

        //TODO not ok ma
        /*
        let data_guard = self.data.lock().unwrap();

        match serde_json::from_str(&data_guard.info_string) {
            Ok(info) => self.player_info = info,
            Err(_) => {
                consol_print("unable to deserialize json".to_string());
            }
        }*/

        // Iterate over all planks and move them according to the input the user
        // provided.

        let opt_movement = input.axis_value("left_paddle");

        if let Some(movement) = opt_movement {
            if movement < 0. {
                to_send_guard.push(format!("{} {}", data_guard.my_uid, "down"));
            } else if movement > 0. {
                to_send_guard.push(format!("{} {}", data_guard.my_uid, "up"));
            }
            println!("{}", movement);
            //todo send move message to server
        }
    }
}
