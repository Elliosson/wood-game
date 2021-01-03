use crate::bevy_components::{TextInfoUi, Tool};
use crate::PlayerInfo;
use bevy::prelude::*;

pub fn text_info_ui_system(
    player_info: Res<PlayerInfo>,
    tool: Res<Tool>,
    mut query: Query<(&mut Text, &TextInfoUi)>,
) {
    for (mut text, _text_info_ui) in query.iter_mut() {
        let my_info = &player_info.my_info;

        let life = format!("life {}/{}", my_info.hp, my_info.max_hp);
        let mut logs = "".to_string();

        let tool_str = if let Some(name) = tool.name.clone() {
            name
        } else {
            "Empty".to_string()
        };

        logs = format!("{}\n Tool: {}", logs, tool_str);

        for log in &my_info.player_log {
            logs = format!("{}\n{}", logs, log);
        }
        text.value = format!("{}\n{}", life, logs);
    }
}
