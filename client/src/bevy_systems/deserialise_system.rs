use crate::{Data, PlayerInfo};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub fn deserialise_player_info_system(
    from_net_data: Res<Arc<Mutex<Data>>>,
    mut player_info: ResMut<PlayerInfo>,
) {
    let data_guard = from_net_data.lock().unwrap();

    match serde_json::from_str(&data_guard.info_string) {
        Ok(info) => {
            let temp: PlayerInfo = info;
            *player_info = temp.clone();
        }
        Err(_) => println!("unable to deserialize json"),
    }
}
