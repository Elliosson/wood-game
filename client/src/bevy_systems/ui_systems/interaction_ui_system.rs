use crate::InteractionRequests;
use crate::PlayerInfo;
use crate::UiState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn interaction_ui_system(
    egui_ctx: ResMut<EguiContext>,
    ui_state: ResMut<UiState>,
    player_info: Res<PlayerInfo>,
    mut interaction_requests: ResMut<InteractionRequests>,
) {
    if ui_state.interaction {
        egui::Window::new("Interaction")
            .scroll(true)
            .show(egui_ctx.ctx(), |ui| {
                ui.vertical(|ui| {
                    for item in &player_info.close_interations {
                        if ui.button(item.interaction_name.clone()).clicked() {
                            interaction_requests.items.push(item.clone())
                        }
                    }
                });
            });
    }
}
