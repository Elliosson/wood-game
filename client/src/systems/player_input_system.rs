use amethyst::{
    core::{timing::Time, transform::Transform},
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::Data;
use std::sync::{Arc, Mutex};

/// This system is responsible for moving all the paddles according to the user
/// provided input.

pub struct PlayerInputSystem;

impl<'s> System<'s> for PlayerInputSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, Arc<Mutex<Vec<String>>>>,
        ReadExpect<'s, Arc<Mutex<Data>>>,
    );

    fn run(&mut self, (_transforms, _time, input, to_send, data): Self::SystemData) {
        let mut to_send_guard = to_send.lock().unwrap();
        let data_guard = data.lock().unwrap();

        let opt_movement = input.axis_value("y_axe");

        if let Some(movement) = opt_movement {
            if movement < 0. {
                to_send_guard.push(format!("{} {}", data_guard.my_uid, "down"));
            } else if movement > 0. {
                to_send_guard.push(format!("{} {}", data_guard.my_uid, "up"));
            }
        }

        let opt_movement = input.axis_value("x_axe");

        if let Some(movement) = opt_movement {
            if movement < 0. {
                to_send_guard.push(format!("{} {}", data_guard.my_uid, "left"));
            } else if movement > 0. {
                to_send_guard.push(format!("{} {}", data_guard.my_uid, "right"));
            }
        }
    }
}
