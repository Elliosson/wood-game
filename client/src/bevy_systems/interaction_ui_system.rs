use crate::bevy_components::{
    ButtonMaterials, InteractionButton, InteractionItemButton, InteractionWindow,
};
use crate::{Data, PlayerInfo, UiCom};
use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub fn interaction_button_system(
    _commands: Commands,
    mut ui_com: ResMut<UiCom>,
    mut interaction_query: Query<(&Button, &InteractionButton, Mutated<Interaction>)>,
) {
    for (_button, _interaction_button, interaction) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                ui_com.interaction = !ui_com.interaction;
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

//send data to the server when the button is pressed
pub fn interaction_item_button_system(
    _commands: Commands,
    to_send: ResMut<Arc<Mutex<Vec<String>>>>,
    net_data: ResMut<Arc<Mutex<Data>>>,
    player_info: Res<PlayerInfo>,
    mut interaction_query: Query<(&Button, Mutated<Interaction>, &InteractionItemButton)>,
) {
    let mut to_send_guard = to_send.lock().unwrap();
    let data_guard = net_data.lock().unwrap();

    for (_button, interaction, item) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                to_send_guard.push(format!(
                    "{} {} {} {} {} {} {}",
                    data_guard.my_uid,
                    "interact",
                    player_info.my_info.pos.x,
                    player_info.my_info.pos.y,
                    item.interaction_name,
                    item.index,
                    item.generation
                ));
            }
            Interaction::Hovered => {}
            Interaction::None => {}
        }
    }
}

//create the window of the interaction
pub fn interaction_ui_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    player_info: Res<PlayerInfo>,
    mut ui_com: ResMut<UiCom>,
    mut query: Query<(Entity, &InteractionWindow)>,
) {
    if ui_com.interaction == true && ui_com.interaction_active == false {
        ui_com.interaction_active = true;
        spawn_interaction_ui(
            commands,
            asset_server,
            materials,
            button_materials,
            player_info,
        );
    } else if ui_com.interaction == false && ui_com.interaction_active == true {
        //despawn the invetory ui
        ui_com.interaction_active = false;
        let mut to_despawns: Vec<Entity> = Vec::new();
        for (entity, _interaction_windows) in query.iter_mut() {
            to_despawns.push(entity);
        }

        for to_despawn in to_despawns.drain(..) {
            commands.despawn(to_despawn);
        }
    }
}

fn spawn_interaction_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    materials: ResMut<Assets<ColorMaterial>>,
    button_materials: Res<ButtonMaterials>,
    player_info: Res<PlayerInfo>,
) {
    let base_node = commands
        //have a preconficured node compoent for this ?
        .spawn(window_node(materials))
        .with(InteractionWindow {});

    for interact in &player_info.close_interations {
        //create a button
        base_node.with_children(|parent| {
            parent
                .spawn(base_button(&button_materials))
                .with(InteractionWindow {})
                .with(InteractionItemButton {
                    interaction_name: interact.interaction_name.clone(),
                    object_name: interact.object_name.clone(),
                    index: interact.index,
                    generation: interact.generation,
                })
                .with_children(|parent| {
                    parent
                        .spawn(text(interact.interaction_name.clone(), &asset_server))
                        .with(InteractionWindow {});
                });
        });
    }
}

fn window_node(mut materials: ResMut<Assets<ColorMaterial>>) -> NodeComponents {
    NodeComponents {
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
    }
}

fn base_button(button_materials: &Res<ButtonMaterials>) -> ButtonComponents {
    ButtonComponents {
        //todo have a predone style of button
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
    }
}

fn text(string: String, asset_server: &Res<AssetServer>) -> TextComponents {
    TextComponents {
        text: Text {
            //todo, same I must just have the same to add, I think I have a way to initialise a Compoent with a child
            value: string,
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            style: TextStyle {
                font_size: 10.0,
                color: Color::rgb(0.9, 0.9, 0.9),
            },
        },
        ..Default::default()
    }
}
