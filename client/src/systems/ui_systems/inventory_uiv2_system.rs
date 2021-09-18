use crate::bevy_components::Tool;
use crate::{BuildRequests, Data, PlayerInfo};
use crate::{FakeInventory, FakeInventoryItem, UiState};
use bevy::prelude::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn inventory_uiv2_system(
    //todo I need to have only one panel, so I need to put this in main ui somhow
    egui_ctx: ResMut<EguiContext>,
    mut ui_state: ResMut<UiState>,
    player_info: Res<PlayerInfo>,
    mut tool: ResMut<Tool>,
    mut fake_inventory: ResMut<FakeInventory>,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
) {
    if ui_state.inventory {
        let inventory = &player_info.inventory;
        egui::Window::new("Inventoryv2")
            .scroll(true)
            .show(egui_ctx.ctx(), |ui| {
                ui.label("Pocket");
                ui.horizontal(|ui| {
                    for i in 0..10 {
                        let name;

                        if let Some(item) = inventory.items.get(&i) {
                            name = item.name.clone();
                        } else {
                            name = "nean".to_string();
                        }

                        if ui.button(name.clone()).clicked() {
                            change_place(
                                &mut &mut fake_inventory,
                                &mut ui_state,
                                i,
                                &to_send,
                                &net_data,
                            )
                        }
                    }
                });

                ui.label("Backpack");

                ui.vertical(|ui| {
                    for i in 1..10 {
                        ui.horizontal(|ui| {
                            for j in 0..10 {
                                let name;
                                let id = i * 10 + j;

                                if let Some(item) = inventory.items.get(&id) {
                                    name = item.name.clone();
                                } else {
                                    name = "nean".to_string();
                                }

                                if ui.button(name.clone()).clicked() {
                                    //send a command to server instead, and then server will reac appropriently
                                    change_place(
                                        &mut &mut fake_inventory,
                                        &mut ui_state,
                                        id,
                                        &to_send,
                                        &net_data,
                                    )
                                }
                            }
                        });
                    }
                });
            });
    }
}

//todo a switch instead, also this will be server side
fn change_place(
    fake_inventory: &mut FakeInventory,
    ui_state: &mut ResMut<UiState>,
    clicked_id: u32,
    to_send: &ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: &ResMut<Arc<Mutex<Data>>>,
) {
    ui_state.item_selected = match ui_state.item_selected {
        None => Some(clicked_id),
        Some(value) => {
            let mut to_send_guard = to_send.lock().unwrap();
            let data_guard = net_data.lock().unwrap();

            to_send_guard.push(format!(
                "{} {} {} {}",
                data_guard.my_uid, "switch_item", value, clicked_id,
            ));

            None
        }
    }
}
