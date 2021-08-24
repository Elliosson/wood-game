pub mod gui;
pub mod rltk_main;
pub mod runstate;

pub use rltk_main::rltk_init;
pub use rltk_main::State;
pub use runstate::Runstate;

pub struct Rect {
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}
