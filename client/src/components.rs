use rltk::RGB;
use serde::Deserialize;
use specs::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Inventory {
    pub items: HashMap<u32, InventoryItem>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InventoryItem {
    pub count: u32,
    pub name: String,
}

//Special component for network, Do NOT serialize, it's could go badly
#[derive(Component, Deserialize, Debug, Clone, Default)]
pub struct PlayerInfo {
    pub inventaire: Vec<InventaireItem>,
    pub inventory: Inventory,
    pub close_interations: Vec<CloseInteration>,
    pub my_info: MyInfo,
    pub possible_builds: Vec<BuildingPlan>,
    pub equipement: Vec<InventaireItem>, //handled the same way that an inventaire item
    pub combat_stats: CombatStats,
}

#[derive(Default, Clone, Debug)]
pub struct FakeInventoryItem {
    pub name: String,
    pub count: u32,
}

#[derive(Default)]
pub struct FakeInventory {
    pub inventory: HashMap<u32, FakeInventoryItem>,
}

#[derive(Component, Debug, Clone, Default)]
pub struct UiState {
    pub label: String,
    pub value: f32,
    pub inverted: bool,
    pub inventory: bool,
    pub build: bool,
    pub interaction: bool,
    pub item_selected: Option<u32>,
}

#[derive(Component, Debug, Clone, Default)]
pub struct InteractionRequests {
    pub items: Vec<CloseInteration>,
}

#[derive(Component, Deserialize, Debug, Clone)]
pub struct BuildingPlan {
    pub name: String,
}

#[derive(Component, Debug, Clone, Default)]
pub struct BuildRequests {
    pub items: Vec<BuildingPlan>,
}

#[derive(Component, Deserialize, Debug, Clone, Default)]
pub struct MyInfo {
    pub pos: Position,
    pub hp: i32,
    pub max_hp: i32,
    pub player_log: Vec<String>,
}

#[derive(Component, Deserialize, Debug, Clone, Default)]
pub struct Position {
    pub x: f32,
    pub y: f32,
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
    pub x: f32,
    pub y: f32,
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
