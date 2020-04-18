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

pub struct BuildUiSystem {
    active: bool,
}

impl Default for BuildUiSystem {
    fn default() -> Self {
        BuildUiSystem { active: true }
    }
}

impl<'s> System<'s> for BuildUiSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadExpect<'s, PlayerInfo>,
        ReadExpect<'s, Arc<Mutex<Vec<String>>>>,
        ReadExpect<'s, Arc<Mutex<Data>>>,
        WriteExpect<'s, UiCom>,
    );

    fn run(&mut self, (mut transforms, player_info, to_send, data, mut ui_com): Self::SystemData) {
        if ui_com.build {
            ui_com.build = false;
            self.active ^= true;
        }

        if self.active {
            let mut to_send_guard = to_send.lock().unwrap();
            let data_guard = data.lock().unwrap();
            let mut open = true;
            amethyst_imgui::with(|ui| {
                let title = im_str!("Build");
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
                    ui.text("Build");
                    //create a button for eaxh interaction il the area
                    for build in &player_info.possible_builds {
                        let name = build.name.clone();
                        if ui.button(&im_str!("{}", name), [0.0, 0.0]) {
                            to_send_guard.push(format!(
                                "{} {} {} {} {}",
                                data_guard.my_uid,
                                "build",
                                player_info.my_info.pos.x,
                                player_info.my_info.pos.y,
                                name
                            ));
                        }
                    }
                });
            });
        }
    }
}
