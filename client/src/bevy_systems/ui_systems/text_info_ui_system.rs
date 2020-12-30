use crate::bevy_components::TextInfoUi;
use crate::PlayerInfo;
use bevy::prelude::*;

pub fn text_info_ui_system(
    player_info: Res<PlayerInfo>,
    mut query: Query<(&mut Text, &TextInfoUi)>,
) {
    for (mut text, _text_info_ui) in query.iter_mut() {
        let my_info = &player_info.my_info;

        let life = format!("life {}/{}", my_info.hp, my_info.max_hp);
        let mut logs = "".to_string();

        for log in &my_info.player_log {
            logs = format!("{}\n{}", logs, log);
        }
        text.value = format!("{}\n{}", life, logs);
    }
}
