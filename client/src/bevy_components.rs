use crate::TILE_SIZE;
use bevy::prelude::*;
use instant::Instant;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Direction2D {
    Up,
    Down,
    Right,
    Left,
    None,
}
pub struct CharacAnimation {
    pub counter: usize,
}

pub struct Sens {
    pub direction: Direction2D,
}

pub struct Player {}
pub struct NonPlayer {}

pub struct ServerState {
    pub x: i32,
    pub y: i32,
    pub id: u32,
    pub gen: i32,
}

pub enum MovementKind {
    Teleport,
    Walk,
}
pub struct Movement {
    pub origin: FPoint,
    pub destination: FPoint,
    pub tdestination: IPoint,
    pub direction: Direction2D,
    pub kind: MovementKind,
    pub counter: usize,
    pub next_time: Instant,
}

#[derive(Clone, Debug)]
pub struct IPoint {
    pub x: i32,
    pub y: i32,
}

impl IPoint {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn _to_fpos(self) -> FPoint {
        FPoint {
            x: self.x as f32 * TILE_SIZE,
            y: self.y as f32 * TILE_SIZE,
        }
    }
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
    pub fn _to_tile(self) -> IPoint {
        //todo, I am realy realy not sure this is correct(round error)
        IPoint {
            x: (self.x / TILE_SIZE) as i32,
            y: (self.y / TILE_SIZE) as i32,
        }
    }
}
// pub struct ButtonMaterials {
//     pub normal: Handle<ColorMaterial>,
//     pub hovered: Handle<ColorMaterial>,
//     pub pressed: Handle<ColorMaterial>,
// }

// impl FromResources for ButtonMaterials {
//     fn from_resources(resources: &Resources) -> Self {
//         let mut materials = resources.get_mut::<Assets<ColorMaterial>>().unwrap();
//         ButtonMaterials {
//             normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
//             hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
//             pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
//         }
//     }
// }

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
