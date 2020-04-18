use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader},
    ecs::prelude::Entity,
    ecs::*,
    input::{get_key, is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    renderer::{loaders::load_from_srgba, palette::Srgba, types::TextureData, Texture},
    ui::*,
    ui::{
        ToNativeWidget, UiButtonBuilder, UiCreator, UiEvent, UiEventType, UiFinder, UiImage,
        UiText, UiTransformData, UiWidget,
    },
};

use std::sync::{Arc, Mutex};

use amethyst_imgui::{
    imgui,
    imgui::{im_str, ImString},
    RenderImgui,
};

use super::Data;
use super::PlayerInfo;
use super::UiCom;
use serde::Deserialize;

use log::info;

const BUTTON_INVENTORY: &str = "inventory";
const BUTTON_BUILD: &str = "build";
const INVENTORY_CONTAINER: &str = "inventory_container";

#[derive(Default, Debug)]
pub struct MyGame {
    bottom_ui_root: Option<Entity>,
    button_inventory: Option<Entity>,
    button_build: Option<Entity>,
    inventory_container: Option<Entity>,
}

impl SimpleState for MyGame {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let _ = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("game_ui.ron", ())));
    }

    fn update(&mut self, state_data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        // only search for buttons if they have not been found yet
        let StateData { world, .. } = state_data;

        if self.button_inventory.is_none()
            || self.button_build.is_none()
            || self.inventory_container.is_none()
        {
            world.exec(|ui_finder: UiFinder<'_>| {
                self.button_inventory = ui_finder.find(BUTTON_INVENTORY);
                self.button_build = ui_finder.find(BUTTON_BUILD);
                self.inventory_container = ui_finder.find(INVENTORY_CONTAINER);
            });
        }

        Trans::None
    }

    fn handle_event(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        let world = data.world;

        match event {
            StateEvent::Window(event) => {
                if is_close_requested(&event) {
                    log::info!("[Trans::Quit] Quitting Application!");
                    Trans::Quit
                } else if is_key_down(&event, VirtualKeyCode::Escape) {
                    log::info!("[Trans::Switch] Switching back to WelcomeScreen!");
                    Trans::None
                } else {
                    Trans::None
                }
            }
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if Some(target) == self.button_inventory {
                    let inventory_root =
                        Some(world.exec(|mut creator: UiCreator<'_>| {
                            creator.create("inventory.ron", ())
                        }));

                    if let Some(inv_cont) = inventory_root {
                        let player_info = world.read_resource::<PlayerInfo>();

                        {
                            let fdg = world.read_resource::<Widgets<UiButton, u32>>();
                        }

                        let loader = world.read_resource::<Loader>();
                        let texture_assets = world.read_resource::<AssetStorage<Texture>>();
                        let texture_builder = load_from_srgba(Srgba::new(0., 0., 0., 1.));
                        let texture_handle: Handle<Texture> = loader.load_from_data(
                            TextureData::from(texture_builder),
                            (),
                            &texture_assets,
                        );

                        // for (i, item) in player_info.inventaire.iter().enumerate() {
                        // add new button to invetory
                        UiButtonBuilder::<(), u32>::new("invbutton")
                            .with_image(texture_handle.clone())
                            .with_hover_image(UiImage::SolidColor([0.1, 0.1, 0.1, 1.]))
                            .with_press_image(UiImage::SolidColor([0.1, 0.1, 0.1, 1.]))
                            .with_parent(inv_cont)
                            .with_position(20., 0.)
                            .with_font_size(12.0f32)
                            .with_text_color([0.0f32, 0.0, 0.0, 1.0])
                            .with_hover_text_color([0.1, 0.1, 0.1, 1.])
                            .with_press_text_color([0.15, 0.15, 0.15, 1.])
                            .build_from_world(&world);
                        // }
                    }

                    return Trans::None;
                }
                if Some(target) == self.button_build {
                    log::info!("[Trans::Switch] Switching to Game!");
                    return Trans::None;
                }

                Trans::None
            }
            StateEvent::Input(InputEvent::KeyPressed {
                /// `VirtualKeyCode`, used for semantic info. i.e. "W" was pressed
                key_code,
                /// Scancode, used for positional info. i.e. The third key on the first row was pressed.
                scancode,
            }) => {
                match key_code {
                    VirtualKeyCode::F => {
                        /*
                        //open the interaction windows
                        let interaction_root = Some(world.exec(|mut creator: UiCreator<'_>| {
                            creator.create("inventory.ron", ())
                        }));

                        let font_handle = {
                            let loader = world.fetch::<Loader>();
                            let font_storage = world.fetch::<AssetStorage<FontAsset>>();
                            loader.load("font/square.ttf", TtfFormat, (), &font_storage)
                        };

                        if let Some(int_cont) = interaction_root {
                            let player_info = world.read_resource::<PlayerInfo>();
                            let loader = world.read_resource::<Loader>();
                            let texture_assets = world.read_resource::<AssetStorage<Texture>>();
                            let texture_builder = load_from_srgba(Srgba::new(0., 0., 0., 1.));
                            let texture_handle: Handle<Texture> = loader.load_from_data(
                                TextureData::from(texture_builder),
                                (),
                                &texture_assets,
                            );
                            println!("pass ");
                            for (i, item) in player_info.close_interations.iter().enumerate() {
                                // add new button to invetory
                                println!("pass with the item");
                                let (_, uibutton) = UiButtonBuilder::<(), u32>::new("invbutton")
                                    //.with_image(texture_handle.clone())
                                    //.with_hover_image(UiImage::SolidColor([0.1, 0.1, 0.1, 1.]))
                                    //.with_press_image(UiImage::SolidColor([0.1, 0.1, 0.1, 1.]))
                                    .with_parent(int_cont)
                                    .with_position(40. * i as f32 + 100., 0.)
                                    .with_font_size(12.0f32)
                                    .with_text_color([0.0f32, 0.0, 0.0, 1.0])
                                    .with_hover_text_color([0.1, 0.1, 0.1, 1.])
                                    .with_press_text_color([0.15, 0.15, 0.15, 1.])
                                    .with_id(555)
                                    .build_from_world(&world);

                                //world.entities().delete(uibutton.text_entity);
                                //world.entities().delete(uibutton.image_entity);
                            }
                        }
                        */
                        let mut ui_com = world.write_resource::<UiCom>();
                        ui_com.interaction = true;
                    }

                    VirtualKeyCode::I => {
                        let mut ui_com = world.write_resource::<UiCom>();
                        ui_com.inventory = true;
                    }
                    VirtualKeyCode::B => {
                        let mut ui_com = world.write_resource::<UiCom>();
                        ui_com.build = true;
                    }
                    VirtualKeyCode::G => {
                        let mut to_send = world.write_resource::<Arc<Mutex<Vec<String>>>>();
                        let mut data = world.write_resource::<Arc<Mutex<Data>>>();

                        let mut to_send_guard = to_send.lock().unwrap();
                        let data_guard = data.lock().unwrap();

                        to_send_guard.push(format!("{} {}", data_guard.my_uid, "pickup"));
                    }

                    _ => {}
                }
                Trans::None
            }
            _ => Trans::None,
        }
    }

    fn on_stop(&mut self, data: StateData<GameData>) {}
}

#[derive(Clone, Deserialize)]
enum CustomUi {
    // Example widget which repeats its `item`
    Repeat {
        x_move: f32,
        y_move: f32,
        count: usize,
        item: UiWidget<CustomUi>,
    },
}

impl ToNativeWidget for CustomUi {
    type PrefabData = ();
    fn to_native_widget(self, _: ()) -> (UiWidget<CustomUi>, Self::PrefabData) {
        match self {
            CustomUi::Repeat {
                count,
                item,
                x_move,
                y_move,
            } => {
                #[allow(clippy::redundant_closure)] // Inference fails on Default::default otherwise
                let transform = item
                    .transform()
                    .cloned()
                    .unwrap_or_else(|| Default::default());
                let mut pos = (0., 0., 100.);
                let children = std::iter::repeat(item)
                    .map(|widget| {
                        let widget = match widget {
                            UiWidget::Button {
                                transform,
                                mut button,
                            } => {
                                button.text = format!("button {}", 44);
                                UiWidget::Button { transform, button }
                            }
                            x => x,
                        };
                        let new_widget = UiWidget::Container {
                            background: None,
                            transform: UiTransformData::default()
                                .with_position(pos.0, pos.1, pos.2),
                            children: vec![widget],
                        };
                        pos.0 += x_move;
                        pos.1 += y_move;
                        new_widget
                    })
                    .take(count)
                    .collect();
                let widget = UiWidget::Container {
                    background: None,
                    transform,
                    children,
                };
                (widget, ())
            }
        }
    }
}
