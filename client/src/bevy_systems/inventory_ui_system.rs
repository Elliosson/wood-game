use crate::PlayerInfo;
use crate::UiState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

pub fn inventory_ui_system(
    mut egui_ctx: ResMut<EguiContext>,
    assets: Res<AssetServer>,
    ui_state: ResMut<UiState>,
    player_info: Res<PlayerInfo>,
) {
    if ui_state.inventory {
        egui::Window::new("Inventory")
            .scroll(true)
            .show(egui_ctx.ctx(), |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");

                ui.vertical(|ui| {
                    for item in &player_info.inventaire {
                        ui.button(item.name.clone()).clicked();
                    }
                });
            });
    }
}
