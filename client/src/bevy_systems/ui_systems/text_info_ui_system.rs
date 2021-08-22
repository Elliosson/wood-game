use crate::bevy_components::{TextInfoUi, Tool};
use crate::InteractionRequests;
use crate::PlayerInfo;
use crate::UiState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

pub fn text_info_ui_system(
    mut egui_ctx: ResMut<EguiContext>,
    player_info: Res<PlayerInfo>,
    tool: Res<Tool>,
) {
    egui::Window::new("Info")
        .scroll(true)
        .show(egui_ctx.ctx(), |ui| {
            ui.vertical(|ui| {
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

                let mut text = format!("{}\n{}", life, logs);
                ui.text_edit_multiline(&mut text)
            });
        });
}
