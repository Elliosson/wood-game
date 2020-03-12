extern crate rltk;
use rltk::{Console, Rltk, VirtualKeyCode, RGB};
extern crate specs;
use super::{CloseInteration, InventaireItem, PlayerInfo};

pub const WINDOWWIDTH: usize = 100;
pub const WINDOWHEIGHT: usize = 80;

#[derive(PartialEq, Copy, Clone)]
pub enum InteractionMenuResult {
    Cancel,
    NoResponse,
    Selected,
}
//get all interaction on player position, print them, and get the choice
pub fn show_object_interaction_choice(
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
) -> (InteractionMenuResult, Option<(i32, i32, CloseInteration)>) {
    let count = player_info.close_interations.len();

    //Draw the box to print the possible interaction
    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Action Choice",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    let mut j = 0;
    let mut possible_interactions: Vec<CloseInteration> = Vec::new();

    // get of interactable object

    for interaction in player_info.close_interations.iter() {
        //get all possible interaction

        //print name of interaction
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            97 + j as u8,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );
        ctx.print(
            21,
            y,
            &format!(
                "{}: {}",
                interaction.object_name, interaction.interaction_name
            ),
        ); //TODO for know interaction are just names
        y += 1;
        j += 1;

        possible_interactions.push(interaction.clone());
    }

    match ctx.key {
        None => (InteractionMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => (InteractionMenuResult::Cancel, None),
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        //TODO transmettre une entieté d'interaction au lieu de transmettre un nom
                        return (
                            InteractionMenuResult::Selected,
                            Some((
                                player_info.my_info.pos.x,
                                player_info.my_info.pos.y,
                                possible_interactions[selection as usize].clone(),
                            )),
                        );
                    }
                    (InteractionMenuResult::NoResponse, None)
                }
            }
        }
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum ItemMenuResult {
    Cancel,
    NoResponse,
    Selected,
}

pub fn show_inventory(
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
) -> (ItemMenuResult, Option<InventaireItem>) {
    let inventory = &player_info.inventaire;
    let count = inventory.len();

    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Inventory",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    let mut j = 0;
    for item in inventory {
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            97 + j as u8,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );

        ctx.print(21, y, &item.name);

        y += 1;
        j += 1;
    }

    match ctx.key {
        None => (ItemMenuResult::NoResponse, None),
        Some(key) => match key {
            VirtualKeyCode::Escape => (ItemMenuResult::Cancel, None),
            _ => {
                let selection = rltk::letter_to_option(key);
                if selection > -1 && selection < count as i32 {
                    return (
                        ItemMenuResult::Selected,
                        Some(inventory[selection as usize].clone()),
                    );
                }
                (ItemMenuResult::NoResponse, None)
            }
        },
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum BuildingMenuResult {
    Cancel,
    NoResponse,
    Selected,
}
//get all interaction on player position, print them, and get the choice
pub fn show_building_choice(
    ctx: &mut Rltk,
    player_info: &PlayerInfo,
) -> (BuildingMenuResult, Option<(i32, i32, String)>) {
    let mut possible_buildings: Vec<String> = Vec::new();
    // get all building possible to build for this entity
    let building_choice = &player_info.possible_builds;

    let count = building_choice.len();

    //Draw the box to print the possible interaction
    let mut y = (25 - (count / 2)) as i32;
    ctx.draw_box(
        15,
        y - 2,
        31,
        (count + 3) as i32,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
    ctx.print_color(
        18,
        y - 2,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Building Choice",
    );
    ctx.print_color(
        18,
        y + count as i32 + 1,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "ESCAPE to cancel",
    );

    let mut j = 0;

    //get all possible interaction
    for building in building_choice.iter() {
        //print name of interaction
        ctx.set(
            17,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437('('),
        );
        ctx.set(
            18,
            y,
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
            97 + j as u8,
        );
        ctx.set(
            19,
            y,
            RGB::named(rltk::WHITE),
            RGB::named(rltk::BLACK),
            rltk::to_cp437(')'),
        );
        ctx.print(21, y, &format!("{}", building.name));
        y += 1;
        j += 1;

        possible_buildings.push(building.name.clone());
    }

    match ctx.key {
        None => (BuildingMenuResult::NoResponse, None),
        Some(key) => {
            match key {
                VirtualKeyCode::Escape => (BuildingMenuResult::Cancel, None),
                _ => {
                    let selection = rltk::letter_to_option(key);
                    if selection > -1 && selection < count as i32 {
                        //TODO transmettre une entieté d'interaction au lieu de transmettre un nom
                        return (
                            BuildingMenuResult::Selected,
                            Some((
                                player_info.my_info.pos.x,
                                player_info.my_info.pos.y,
                                possible_buildings[selection as usize].clone(),
                            )),
                        );
                    }
                    (BuildingMenuResult::NoResponse, None)
                }
            }
        }
    }
}

pub fn draw_ui(ctx: &mut Rltk, player_info: &PlayerInfo) {
    let buf = format!("commands");
    ctx.print(140, 1, &buf.to_string());
    let buf = format!("move:        arrow keys");
    ctx.print(140, 2, &buf.to_string());
    let buf = format!("inventory:   i");
    ctx.print(140, 3, &buf.to_string());
    let buf = format!("interaction: f");
    ctx.print(140, 4, &buf.to_string());
    let buf = format!("get item:    g");
    ctx.print(140, 5, &buf.to_string());
    let buf = format!("build:       b");
    ctx.print(140, 6, &buf.to_string());
    let buf = format!("destroy:     space");
    ctx.print(140, 7, &buf.to_string());

    bottom_gui(ctx, player_info);
}

pub fn show_pseudo(ctx: &mut Rltk, pseudo: &String) {
    ctx.print_color(
        75,
        30,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        "Please enter your pseudo",
    );

    ctx.print_color(
        75,
        32,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        pseudo,
    );
}

pub fn bottom_gui(ctx: &mut Rltk, player_info: &PlayerInfo) {
    let hp = player_info.my_info.hp;
    let max_hp = player_info.my_info.max_hp;

    let health = format!(" HP: {} / {} ", hp, max_hp);
    ctx.print_color(
        12,
        WINDOWHEIGHT as i32 - 7,
        RGB::named(rltk::YELLOW),
        RGB::named(rltk::BLACK),
        &health,
    );
    ctx.draw_bar_horizontal(
        28,
        WINDOWHEIGHT as i32 - 7,
        WINDOWWIDTH as i32 + 1,
        hp,
        max_hp,
        RGB::named(rltk::RED),
        RGB::named(rltk::BLACK),
    );
}
