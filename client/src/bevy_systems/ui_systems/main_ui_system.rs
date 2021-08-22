use crate::UiState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin, EguiSettings};

pub fn main_ui_system(
    mut egui_ctx: ResMut<EguiContext>,
    assets: Res<AssetServer>,
    mut ui_state: ResMut<UiState>,
) {
    let mut load = false;
    let mut remove = false;
    let mut invert = false;
    let mut inventory = false;
    let mut build = false;
    let mut interaction = false;

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(egui_ctx.ctx(), |ui| {
            ui.heading("Side Panel");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut ui_state.label);
            });

            ui.add(egui::Slider::new(&mut ui_state.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                ui_state.value += 1.0;
            }

            ui.allocate_space(egui::Vec2::new(1.0, 100.0));
            ui.horizontal(|ui| {
                load = ui.button("Load").clicked();
                invert = ui.button("Invert").clicked();
                remove = ui.button("Remove").clicked();
                inventory = ui.button("Inventory").clicked();
                build = ui.button("Build").clicked();
                interaction = ui.button("Interaction").clicked();
            });

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui/").text("powered by egui"),
                );
            });
        });

    if inventory {
        ui_state.inventory = !ui_state.inventory;
    }
    if build {
        ui_state.build = !ui_state.build;
    }
    if interaction {
        ui_state.interaction = !ui_state.interaction;
    }
}
