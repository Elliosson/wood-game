use crate::bevy_components::ButtonMaterials;
use bevy::prelude::*;

pub fn window_node(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeComponents {
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

pub fn item_node(materials: &mut ResMut<Assets<ColorMaterial>>) -> NodeComponents {
    NodeComponents {
        style: Style {
            size: Size::new(Val::Px(500.0), Val::Px(50.0)),
            position: Rect {
                left: Val::Percent(0.),
                top: Val::Percent(0.),
                ..Default::default()
            },
            flex_direction: FlexDirection::Row,
            // align_content: AlignContent::FlexStart,
            justify_content: JustifyContent::FlexStart,
            // justify_content: JustifyContent::FlexEnd,
            ..Default::default()
        },
        material: materials.add(Color::BLUE.into()),
        ..Default::default()
    }
}

pub fn base_button(button_materials: &Res<ButtonMaterials>) -> ButtonComponents {
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

pub fn text(string: String, asset_server: &Res<AssetServer>) -> TextComponents {
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
