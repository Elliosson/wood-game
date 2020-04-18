// ici on va lire les donne du network sur notre position et centrer le tou en consequant

use amethyst::ecs::prelude::{ReadExpect, System, WriteExpect};

use crate::{Data, PlayerInfo};
use std::sync::{Arc, Mutex};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.

pub struct DeserialiseSystem;

impl<'s> System<'s> for DeserialiseSystem {
    type SystemData = (
        ReadExpect<'s, Arc<Mutex<Data>>>,
        WriteExpect<'s, PlayerInfo>,
    );

    fn run(&mut self, (data, mut player_info): Self::SystemData) {
        let data_guard = data.lock().unwrap();

        match serde_json::from_str(&data_guard.info_string) {
            Ok(info) => {
                let temp: PlayerInfo = info;
                *player_info = temp.clone();
            }
            Err(_) => println!("unable to deserialize json"),
        }
    }
}
