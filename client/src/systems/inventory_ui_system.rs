use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Join, ReadExpect, System, WriteExpect, WriteStorage},
    renderer::Camera,
};

use crate::Data;
use crate::UiCom;

use crate::PlayerInfo;
use amethyst_imgui::{
    imgui,
    imgui::{im_str, ImString},
    RenderImgui,
};
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
        WriteStorage<'s, Transform>,
        ReadExpect<'s, PlayerInfo>,
        ReadExpect<'s, Arc<Mutex<Vec<String>>>>,
        ReadExpect<'s, Arc<Mutex<Data>>>,
        WriteExpect<'s, UiCom>,
    );

    fn run(&mut self, (mut transforms, player_info, to_send, data, mut ui_com): Self::SystemData) {
        if ui_com.inventory {
            ui_com.inventory = false;
            if self.active == true {
                self.active = false;
            } else {
                self.active = true;
            }
        }

        if self.active {
            let mut to_send_guard = to_send.lock().unwrap();
            let data_guard = data.lock().unwrap();
            let mut open = true;
            amethyst_imgui::with(|ui| {
                let title = im_str!("Inventory");
                let mut window = imgui::Window::new(&title)
                    .bg_alpha(0.35)
                    .movable(true)
                    .no_decoration()
                    .always_auto_resize(true)
                    .save_settings(false)
                    .focus_on_appearing(false)
                    .no_nav()
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
