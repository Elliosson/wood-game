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
        Runstate::Interaction => player_interaction(uid, ctx, player_info, ws),
        _ => Runstate::BaseState,
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
