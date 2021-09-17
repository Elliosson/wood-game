use bevy::prelude::*;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Direction2D {
    Up,
    Down,
    Right,
    Left,
    None,
}

impl Default for Direction2D {
    fn default() -> Self {
        Direction2D::Down
    }
}

pub struct Sens {
    pub direction: Direction2D,
}

pub struct Player {}
pub struct NonPlayer {}

#[derive(Debug, Default)]
pub struct ServerState {
    pub x: f32,
    pub y: f32,
    pub id: u32,
    pub gen: i32,
}

pub struct Movement {
    pub origin: FPoint,
    pub destination: FPoint,
    pub direction: Direction2D,
}

#[derive(Default)]
pub struct SpriteState {
    pub direction: Direction2D,
    pub counter: usize,
}

#[derive(Clone, Debug)]
pub struct FPoint {
    pub x: f32,
    pub y: f32,
}

impl FPoint {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

pub struct MouseLoc(pub Vec2);

impl Default for MouseLoc {
    fn default() -> Self {
        MouseLoc(Vec2::new(0., 0.))
    }
}

#[derive(Default)]
pub struct Tool {
    pub name: Option<String>,
}
