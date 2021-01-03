use super::ui_bases::*;
use crate::bevy_components::{
    ButtonMaterials, EquipButton, InventoryButton, InventoryItemButton, InventoryWindow,
};
use crate::{Data, PlayerInfo, UiCom};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub fn inventory_button_system(
    _commands: Commands,
    mut ui_com: ResMut<UiCom>,
    mut interaction_query: Query<(&Button, &InventoryButton, Mutated<Interaction>)>,
) {
    for (_button, _inventory_button, interaction) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ui_com.inventory = !ui_com.inventory;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

//send data to the server when the button is pressed
pub fn inventory_item_button_system(
    _commands: Commands,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
    mut interaction_query: Query<(&Button, Mutated<Interaction>, &InventoryItemButton)>,
) {
    let mut to_send_guard = to_send.lock().unwrap();
    let data_guard = net_data.lock().unwrap();

    for (_button, interaction, item) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                to_send_guard.push(format!(
                    "{} {} {} {}",
                    data_guard.my_uid, "consume", item.index, item.generation
                ));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

//create the window of the inventory
pub fn inventory_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
    player_info: Res<PlayerInfo>,
    mut ui_com: ResMut<UiCom>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<(Entity, &InventoryWindow)>,
) {
    if ui_com.inventory == true && ui_com.inventory_active == false {
        //spawn the inventory ui

        ui_com.inventory_active = true;
        let base_node = commands
            .spawn(window_node(&mut materials))
            .with(InventoryWindow {});

        for item in &player_info.inventaire {
            //create a button
            base_node.with_children(|parent| {
                parent
                    .spawn(item_node(&mut materials))
                    .with(InventoryWindow {})
                    .with_children(|parent| {
                        parent
                            .spawn(base_button(&button_materials))
                            .with(InventoryWindow {})
                            .with(InventoryItemButton {
                                name: item.name.clone(),
                                index: item.index,
                                generation: item.generation,
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(text(item.name.clone(), &asset_server))
                                    .with(InventoryWindow {});
                            })
                            .spawn(base_button(&button_materials))
                            .with(InventoryWindow {})
                            .with(EquipButton {
                                name: item.name.clone(),
                                index: item.index,
                                generation: item.generation,
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn(text("equip".to_string(), &asset_server))
                                    .with(InventoryWindow {});
                            });
                    });
            });
        }
    } else if ui_com.inventory == false && ui_com.inventory_active == true {
        //despawn the invetory ui
        ui_com.inventory_active = false;
        let mut to_despawns: Vec<Entity> = Vec::new();
        for (entity, _inventory_windows) in query.iter_mut() {
            to_despawns.push(entity);
        }

        for to_despawn in to_despawns.drain(..) {
            commands.despawn(to_despawn);
        }
    }
}
