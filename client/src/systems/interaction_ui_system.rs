use amethyst::{core::transform::Transform, ecs::prelude::*, renderer::Camera};

use crate::Data;
use crate::UiCom;

use crate::PlayerInfo;
use amethyst_imgui::{
    imgui,
    imgui::{im_str, ImString},
    RenderImgui,
};
use std::sync::{Arc, Mutex};

pub struct InteractionUiSystem {
    active: bool,
}

impl Default for InteractionUiSystem {
    fn default() -> Self {
        InteractionUiSystem { active: true }
    }
}

impl<'s> System<'s> for InteractionUiSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadExpect<'s, PlayerInfo>,
        ReadExpect<'s, Arc<Mutex<Vec<String>>>>,
        ReadExpect<'s, Arc<Mutex<Data>>>,
        WriteExpect<'s, UiCom>,
    );

    fn run(&mut self, (mut transforms, player_info, to_send, data, mut ui_com): Self::SystemData) {
        if ui_com.interaction {
            ui_com.interaction = false;
            self.active ^= true;
        }

        if self.active {
            let mut to_send_guard = to_send.lock().unwrap();
            let data_guard = data.lock().unwrap();
            let mut open = true;
            amethyst_imgui::with(|ui| {
                let title = im_str!("Example: Simple overlay");
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
                    ui.text("Simple overlay\nin the corner of the screen");
                    //create a button for eaxh interaction il the area
                    for interaction in &player_info.close_interations {
                        let name = interaction.interaction_name.clone();
                        if ui.button(&im_str!("{}", name), [0.0, 0.0]) {
                            to_send_guard.push(format!(
                                "{} {} {} {} {} {} {}",
                                data_guard.my_uid,
                                "interact",
                                player_info.my_info.pos.x,
                                player_info.my_info.pos.y,
                                interaction.interaction_name,
                                interaction.index,
                                interaction.generation
                            ));
                        }
                    }
                });
            });
        }
    }
}
