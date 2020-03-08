use rltk::{Rltk, VirtualKeyCode};

use web_sys::WebSocket;
extern crate specs;

use super::{components::*, gui, Rect};

#[derive(Debug, Clone)]
pub enum Runstate {
    BaseState,
    Inventory,
    Interaction,
    Build,
}

pub fn player_input(
    uid: String,
    ws: WebSocket,
    ctx: &mut Rltk,
    runstate: &Runstate,
    rect: &mut Rect,
    player_info: &PlayerInfo,
) -> Runstate {
    let newrunstate = match runstate {
        Runstate::BaseState => player_base_state(uid, ws, ctx, rect),
        Runstate::Inventory => player_inventory(uid, ctx, player_info, ws),
        Runstate::Interaction => player_interaction(uid, ctx, player_info, ws),
        Runstate::Build => player_build(uid, ctx, player_info, ws),
    };
    newrunstate
}

pub fn player_base_state(uid: String, ws: WebSocket, ctx: &mut Rltk, rect: &mut Rect) -> Runstate {
    // Player movement
    let newrunstate;

    match ctx.key {
        None => {
            newrunstate = Runstate::BaseState;
        } // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => {
                ws.send_with_str(&format!("{} {}", uid, "left"))
                    .expect("Unable to send the message");
                newrunstate = Runstate::BaseState;
                rect.x -= 1
            }

            VirtualKeyCode::Right => {
                ws.send_with_str(&format!("{} {}", uid, "right"))
                    .expect("Unable to send the message");
                newrunstate = Runstate::BaseState;
                rect.x += 1
            }
            VirtualKeyCode::Up => {
                ws.send_with_str(&format!("{} {}", uid, "up"))
                    .expect("Unable to send the message");
                newrunstate = Runstate::BaseState;
                rect.y -= 1
            }
            VirtualKeyCode::Down => {
                ws.send_with_str(&format!("{} {}", uid, "down"))
                    .expect("Unable to send the message");
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
                ws.send_with_str(&format!("{} {}", uid, "pickup"))
                    .expect("Unable to send the message");
                newrunstate = Runstate::BaseState;
            }
            //destroy
            VirtualKeyCode::Space => {
                ws.send_with_str(&format!("{} {}", uid, "destroy"))
                    .expect("Unable to send the message");
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
    ws: WebSocket,
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
            ws.send_with_str(&format!(
                "{} {} {} {} {} {} {}",
                uid,
                "interact",
                x,
                y,
                interaction.interaction_name,
                interaction.index,
                interaction.generation
            ))
            .expect("Unable to send the message");

            newrunstate = Runstate::BaseState;
        }
    }
    newrunstate
}

pub fn player_inventory(
    uid: String,
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
    ws: WebSocket,
) -> Runstate {
    let mut newrunstate = Runstate::Inventory;
    let result = gui::show_inventory(ctx, player_info);
    match result.0 {
        gui::ItemMenuResult::Cancel => newrunstate = Runstate::BaseState,
        gui::ItemMenuResult::NoResponse => {}
        gui::ItemMenuResult::Selected => {
            let item = result.1.unwrap();

            ws.send_with_str(&format!(
                "{} {} {} {} {}",
                uid, "want_use", item.name, item.index, item.generation
            ))
            .expect("Unable to send the message");
            newrunstate = Runstate::BaseState;
        }
    }

    newrunstate
}

pub fn player_build(
    uid: String,
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
    ws: WebSocket,
) -> Runstate {
    let mut newrunstate = Runstate::Build;
    let result = gui::show_building_choice(ctx, player_info);

    match result.0 {
        gui::BuildingMenuResult::Cancel => newrunstate = Runstate::BaseState,
        gui::BuildingMenuResult::NoResponse => {}
        gui::BuildingMenuResult::Selected => {
            let interaction_tuple = result.1.unwrap();
            let (x, y, building_name) = interaction_tuple;

            ws.send_with_str(&format!(
                "{} {} {} {} {}",
                uid, "build", x, y, building_name,
            ))
            .expect("Unable to send the message");

            newrunstate = Runstate::BaseState;
        }
    }
    newrunstate
}
