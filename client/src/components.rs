use serde::Deserialize;
use specs::prelude::*;

//Special component for network, Do NOT serialize, it's could go badly
#[derive(Component, Deserialize, Debug, Clone)]
pub struct PlayerInfo {
    pub inventaire: Vec<InventaireItem>,
    pub close_interations: Vec<CloseInteration>,
    pub my_info: MyInfo,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct MyInfo {
    pub pos: Position,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InventaireItem {
    pub name: String,
    pub index: u32,
    pub generation: i32,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CloseInteration {
    pub interaction_name: String,
    pub object_name: String,
    pub index: u32,
    pub generation: i32,
}
