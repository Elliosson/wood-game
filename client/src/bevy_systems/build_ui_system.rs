use crate::BuildRequests;
use crate::BuildingPlan;
use crate::PlayerInfo;
use crate::UiState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

pub fn build_ui_system(
    mut egui_ctx: ResMut<EguiContext>,
    assets: Res<AssetServer>,
    ui_state: ResMut<UiState>,
    player_info: Res<PlayerInfo>,
    mut build_request: ResMut<BuildRequests>,
) {
    if ui_state.build {
        egui::Window::new("Build")
            .scroll(true)
            .show(egui_ctx.ctx(), |ui| {
                ui.vertical(|ui| {
                    for build in &player_info.possible_builds {
                        if ui.button(build.name.clone()).clicked() {
                            build_request.items.push(build.clone())
                        }
                    }
                });
            });
    }
}
