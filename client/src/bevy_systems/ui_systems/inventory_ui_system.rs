use crate::bevy_components::{
    ButtonMaterials, InventoryButton, InventoryItemButton, InventoryWindow,
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
            .spawn(NodeComponents {
                style: Style {
                    size: Size::new(Val::Px(500.0), Val::Px(500.0)),
                    position: Rect {
                        left: Val::Percent(0.),
                        top: Val::Percent(0.),
                        ..Default::default()
                    },
                    flex_direction: FlexDirection::Column,
                    // align_content: AlignContent::FlexStart,
                    // justify_content: JustifyContent::FlexStart,
                    justify_content: JustifyContent::FlexEnd,
                    ..Default::default()
                },
                material: materials.add(Color::WHITE.into()),
                ..Default::default()
            })
            .with(InventoryWindow {});

        for item in &player_info.inventaire {
            //create a button
            base_node.with_children(|parent| {
                parent
                    .spawn(ButtonComponents {
                        style: Style {
                            margin: Rect {
                                bottom: Val::Px(10.),
                                ..Default::default()
                            },
                            size: Size::new(Val::Px(70.0), Val::Px(30.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..Default::default()
                        },
                        material: button_materials.normal.clone(),
                        ..Default::default()
                    })
                    .with(InventoryWindow {})
                    .with(InventoryItemButton {
                        name: item.name.clone(),
                        index: item.index,
                        generation: item.generation,
                    })
                    .with_children(|parent| {
                        parent
                            .spawn(TextComponents {
                                text: Text {
                                    value: item.name.clone(),
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    style: TextStyle {
                                        font_size: 10.0,
                                        color: Color::rgb(0.9, 0.9, 0.9),
                                    },
                                },
                                ..Default::default()
                            })
                            .with(InventoryWindow {});
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
