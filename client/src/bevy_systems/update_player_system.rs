use crate::bevy_components::{CharacAnimation, Direction2D, Movement, Player, ServerState};
use crate::{PlayerInfo, TILE_SIZE};
use bevy::prelude::*;
use bevy::render::camera::Camera;
use std::time::{Duration, Instant};

pub fn update_player_system(
    mut commands: Commands,
    player_info: ResMut<PlayerInfo>,
    mut query_player: Query<(Entity, &Player, &mut ServerState)>,
) {
    //synchronise player position
    //todo in a separate system

    for (entity, _player, mut server_state) in query_player.iter_mut() {
        if server_state.x != player_info.my_info.pos.x
            || server_state.y != player_info.my_info.pos.y
        {
            println!(
                "move player to {} {}",
                player_info.my_info.pos.x, player_info.my_info.pos.y
            );
            server_state.x = player_info.my_info.pos.x;
            server_state.y = player_info.my_info.pos.y;
        }
    }
}
