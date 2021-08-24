use amethyst::ecs::prelude::*;

use crate::Data;
use crate::UiCom;

use crate::PlayerInfo;
use amethyst_imgui::{imgui, imgui::im_str};
use std::sync::{Arc, Mutex};

pub struct InventoryUiSystem {
    active: bool,
}

impl Default for InventoryUiSystem {
    fn default() -> Self {
        InventoryUiSystem { active: true }
    }
}

impl<'s> System<'s> for InventoryUiSystem {
    type SystemData = (
        ReadExpect<'s, PlayerInfo>,
        ReadExpect<'s, Arc<Mutex<Vec<String>>>>,
        ReadExpect<'s, Arc<Mutex<Data>>>,
        WriteExpect<'s, UiCom>,
    );

    fn run(&mut self, (player_info, to_send, data, mut ui_com): Self::SystemData) {
        if ui_com.inventory {
            ui_com.inventory = false;
            if self.active == true {
                self.active = false;
            } else {
                self.active = true;
            }
        }

        if self.active {
            let mut _to_send_guard = to_send.lock().unwrap();
            let _data_guard = data.lock().unwrap();
            let mut open = true;
            amethyst_imgui::with(|ui| {
                let title = im_str!("Inventory");
                let window = imgui::Window::new(&title)
                    .bg_alpha(0.35)
                    .movable(true)
                    .no_decoration()
                    .always_auto_resize(true)
                    .save_settings(false)
                    .focus_on_appearing(false)
                    .no_nav()
                    .position([500., 500.], imgui::Condition::FirstUseEver)
                    .opened(&mut open);

                window.build(ui, || {
                    ui.text("Inventory");
                    //create a button for eaxh interaction il the area
                    for inventory in &player_info.inventaire {
                        let name = inventory.name.clone();
                        if ui.button(&im_str!("{}", name), [0.0, 0.0]) {
                            //nothing for now
                        }
                    }
                });
            });
        }
    }
}
