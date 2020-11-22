use bevy::prelude::*;

pub struct InventoryButton {}
pub struct InventoryWindow {}
pub struct InventoryItemButton {
    pub name: String,
    pub index: u32,
    pub generation: i32,
}

pub struct InteractionButton {}
pub struct InteractionWindow {}
pub struct InteractionItemButton {
    pub interaction_name: String,
    pub object_name: String,
    pub index: u32,
    pub generation: i32,
}

pub struct BuildButton {}
pub struct BuildWindow {}
pub struct BuildItemButton {
    pub name: String,
}

pub struct ButtonMaterials {
    pub normal: Handle<ColorMaterial>,
    pub hovered: Handle<ColorMaterial>,
    pub pressed: Handle<ColorMaterial>,
}

impl FromResources for ButtonMaterials {
    fn from_resources(resources: &Resources) -> Self {
        let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
        }
    }
}
