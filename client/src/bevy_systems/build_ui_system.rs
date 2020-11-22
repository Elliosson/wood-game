use super::{base_button, text, window_node};
use crate::bevy_components::{BuildButton, BuildItemButton, BuildWindow, ButtonMaterials};
use crate::{Data, PlayerInfo, UiCom};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub fn build_button_system(
    _commands: Commands,
    mut ui_com: ResMut<UiCom>,
    mut query: Query<(&Button, &BuildButton, Mutated<Interaction>)>,
) {
    for (_button, _, interaction) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ui_com.build = !ui_com.build;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

//send data to the server when the button is pressed
pub fn build_item_button_system(
    _commands: Commands,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
    player_info: Res<PlayerInfo>,
    mut query: Query<(&Button, Mutated<Interaction>, &BuildItemButton)>,
) {
    let mut to_send_guard = to_send.lock().unwrap();
    let data_guard = net_data.lock().unwrap();

    for (_button, interaction, item) in query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                to_send_guard.push(format!(
                    "{} {} {} {} {}",
                    data_guard.my_uid,
                    "build",
                    player_info.my_info.pos.x,
                    player_info.my_info.pos.y,
                    item.name
                ));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

//create the window
pub fn build_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    player_info: Res<PlayerInfo>,
    mut ui_com: ResMut<UiCom>,
    mut query: Query<(Entity, &BuildWindow)>,
) {
    if ui_com.build == true && ui_com.build_active == false {
        ui_com.build_active = true;
        spawn_build_ui(
            commands,
            asset_server,
            materials,
            button_materials,
            player_info,
        );
    } else if ui_com.build == false && ui_com.build_active == true {
        //despawn the invetory ui
        ui_com.build_active = false;
        let mut to_despawns: Vec<Entity> = Vec::new();
        for (entity, _windows) in query.iter_mut() {
            to_despawns.push(entity);
        }

        for to_despawn in to_despawns.drain(..) {
            commands.despawn(to_despawn);
        }
    }
}

fn spawn_build_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    player_info: Res<PlayerInfo>,
) {
    let base_node = commands
        //have a preconficured node compoent for this ?
        .spawn(window_node(materials))
        .with(BuildWindow {});

    for build in &player_info.possible_builds {
        //create a button
        base_node.with_children(|parent| {
            parent
                .spawn(base_button(&button_materials))
                .with(BuildWindow {})
                .with(BuildItemButton {
                    name: build.name.clone(),
                })
                .with_children(|parent| {
                    parent
                        .spawn(text(build.name.clone(), &asset_server))
                        .with(BuildWindow {});
                });
        });
    }
}
