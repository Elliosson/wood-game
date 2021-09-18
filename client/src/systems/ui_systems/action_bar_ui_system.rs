use crate::bevy_components::Tool;
use crate::PlayerInfo;
use crate::UiState;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::collections::HashMap;

#[derive(Default)]
struct FakeInventoryItem {
    pub name: String,
    pub count: u32,
}

#[derive(Default)]
struct FakeInventory {
    pub inventory: HashMap<u32, FakeInventoryItem>,
}

pub fn action_bar_ui_system(
    egui_ctx: ResMut<EguiContext>,
    ui_state: ResMut<UiState>,
    player_info: Res<PlayerInfo>,
    mut tool: ResMut<Tool>,
) {
    let mut fake_inventory = FakeInventory::default();

    fake_inventory.inventory.insert(
        1,
        FakeInventoryItem {
            name: "sword".to_string(),
            count: 1,
        },
    );

    egui::TopBottomPanel::bottom("Action Bar").show(egui_ctx.ctx(), |ui| {
        ui.label("You would normally chose either panels OR windows.");
        ui.horizontal(|ui| {
            for i in 0..10 {
                let name;

                if let Some(item) = fake_inventory.inventory.get(&i) {
                    name = item.name.clone();
                } else {
                    name = "nean".to_string();
                }

                if ui.button(name.clone()).clicked() {
                    tool.name = Some(name.clone());
                }
            }
        });
    });
}
