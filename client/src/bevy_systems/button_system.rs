use crate::bevy_components::ButtonMaterials;
use bevy::prelude::*;

pub fn button_system(
    _commands: Commands,
    button_materials: Res<ButtonMaterials>,
    mut interaction_query: Query<(&Button, Mutated<Interaction>, &mut Handle<ColorMaterial>)>,
) {
    for (_button, interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}
