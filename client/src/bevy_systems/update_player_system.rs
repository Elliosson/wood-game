use crate::bevy_components::{Player, ServerState};
use crate::PlayerInfo;
use bevy::prelude::*;

pub fn update_player_system(
    player_info: ResMut<PlayerInfo>,
    mut query_player: Query<(&Player, &mut ServerState)>,
) {
    //synchronise player position
    //todo in a separate system

    for (_player, mut server_state) in query_player.iter_mut() {
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
