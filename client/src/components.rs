use rltk::RGB;
use serde::Deserialize;
use specs::prelude::*;

//Special component for network, Do NOT serialize, it's could go badly
#[derive(Component, Deserialize, Debug, Clone)]
pub struct PlayerInfo {
    pub inventaire: Vec<InventaireItem>,
    pub close_interations: Vec<CloseInteration>,
    pub my_info: MyInfo,
    pub possible_builds: Vec<BuildingPlan>,
    pub equipement: Vec<InventaireItem>, //handled the same way that an inventaire item
    pub combat_stats: CombatStats,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct BuildingPlan {
    pub name: String,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct MyInfo {
    pub pos: Position,
    pub hp: i32,
    pub max_hp: i32,
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

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
    pub render_order: i32,
}

#[derive(Component, Debug, Deserialize, Clone, Default)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
    pub base_def: i32,
    pub base_att: i32,
    pub att: i32,
}
