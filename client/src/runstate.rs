use rltk::{Rltk, VirtualKeyCode};

extern crate specs;
use std::sync::{Arc, Mutex};

use super::{components::*, gui, Rect};

#[derive(Debug, Clone)]
pub enum Runstate {
    Register,
    BaseState,
    Inventory,
    Interaction,
    Build,
}

pub fn player_input(
    uid: String,
    to_send: Arc<Mutex<Vec<String>>>,
    ctx: &mut Rltk,
    runstate: &Runstate,
    rect: &mut Rect,
    player_info: &PlayerInfo,
    pseudo: &mut String,
) -> Runstate {
    let mut to_send_guard = to_send.lock().unwrap();
    let newrunstate = match runstate {
        Runstate::Register => choose_pseudo(ctx, pseudo, &mut to_send_guard),
        Runstate::BaseState => player_base_state(uid, ctx, rect, &mut to_send_guard),
        Runstate::Inventory => player_inventory(uid, ctx, player_info, &mut to_send_guard),
        Runstate::Interaction => player_interaction(uid, ctx, player_info, &mut to_send_guard),
        Runstate::Build => player_build(uid, ctx, player_info, &mut to_send_guard),
    };
    newrunstate
}

pub fn player_base_state(
    uid: String,
    ctx: &mut Rltk,
    rect: &mut Rect,
    to_send: &mut Vec<String>,
) -> Runstate {
    // Player movement
    let newrunstate;

    match ctx.key {
        None => {
            newrunstate = Runstate::BaseState;
        } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => {
                to_send.push(format!("{} {}", uid, "left"));
                newrunstate = Runstate::BaseState;
                rect.x -= 1
            }

            VirtualKeyCode::Right => {
                to_send.push(format!("{} {}", uid, "right"));
                newrunstate = Runstate::BaseState;
                rect.x += 1
            }
            VirtualKeyCode::Up => {
                to_send.push(format!("{} {}", uid, "up"));
                newrunstate = Runstate::BaseState;
                rect.y -= 1
            }
            VirtualKeyCode::Down => {
                to_send.push(format!("{} {}", uid, "down"));
                newrunstate = Runstate::BaseState;
                rect.y += 1
            }
            VirtualKeyCode::F => {
                newrunstate = Runstate::Interaction;
            }
            VirtualKeyCode::I => {
                newrunstate = Runstate::Inventory;
            }
            VirtualKeyCode::B => {
                newrunstate = Runstate::Build;
            }
            //pickup
            VirtualKeyCode::G => {
                to_send.push(format!("{} {}", uid, "pickup"));
                newrunstate = Runstate::BaseState;
            }
            //destroy
            VirtualKeyCode::Space => {
                to_send.push(format!("{} {}", uid, "destroy"));
                newrunstate = Runstate::BaseState;
            }

            _ => {
                newrunstate = Runstate::BaseState;
            }
        },
    }
    newrunstate
}

pub fn player_interaction(
    uid: String,
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
    to_send: &mut Vec<String>,
) -> Runstate {
    let mut newrunstate = Runstate::Interaction;

    let result = gui::show_object_interaction_choice(ctx, player_info);
    match result.0 {
        gui::InteractionMenuResult::Cancel => newrunstate = Runstate::BaseState,
        gui::InteractionMenuResult::NoResponse => {}
        gui::InteractionMenuResult::Selected => {
            let interaction_tuple = result.1.unwrap();
            let (x, y, interaction) = interaction_tuple;

            //send the response
            to_send.push(format!(
                "{} {} {} {} {} {} {}",
                uid,
                "interact",
                x,
                y,
                interaction.interaction_name,
                interaction.index,
                interaction.generation
            ));

            newrunstate = Runstate::BaseState;
        }
    }
    newrunstate
}

pub fn player_inventory(
    uid: String,
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
    to_send: &mut Vec<String>,
) -> Runstate {
    let mut newrunstate = Runstate::Inventory;
    let result = gui::show_inventory(ctx, player_info);
    match result.0 {
        gui::ItemMenuResult::Cancel => newrunstate = Runstate::BaseState,
        gui::ItemMenuResult::NoResponse => {}
        gui::ItemMenuResult::Selected => {
            let item = result.1.unwrap();

            to_send.push(format!(
                "{} {} {} {}",
                uid, "consume", item.index, item.generation
            ));
            newrunstate = Runstate::BaseState;
        }
    }

    newrunstate
}

pub fn player_build(
    uid: String,
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
    to_send: &mut Vec<String>,
) -> Runstate {
    let mut newrunstate = Runstate::Build;
    let result = gui::show_building_choice(ctx, player_info);

    match result.0 {
        gui::BuildingMenuResult::Cancel => newrunstate = Runstate::BaseState,
        gui::BuildingMenuResult::NoResponse => {}
        gui::BuildingMenuResult::Selected => {
            let interaction_tuple = result.1.unwrap();
            let (x, y, building_name) = interaction_tuple;

            to_send.push(format!("{} {} {} {} {}", uid, "build", x, y, building_name,));

            newrunstate = Runstate::BaseState;
        }
    }
    newrunstate
}

fn choose_pseudo(ctx: &mut Rltk, pseudo: &mut String, to_send: &mut Vec<String>) -> Runstate {
    let mut newrunstate = Runstate::Register;

    gui::show_pseudo(ctx, pseudo);

    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Back => {
                pseudo.pop();
            }
            VirtualKeyCode::Return => {
                to_send.push(format!("register {}", pseudo));

                newrunstate = Runstate::BaseState;
            }
            VirtualKeyCode::Escape => {
                pseudo.clear();
            }
            _ => {
                let key_value = key as u32 + 55;
                if key_value >= 65 && key_value <= 90 {
                    let key_value = key_value as u8;
                    let key_char = key_value as char;
                    pseudo.push(key_char);
                }
            }
        },
    }
    newrunstate
}
