use amethyst::{
    ecs::prelude::Entity,
    ecs::*,
    input::{is_close_requested, is_key_down, InputEvent, VirtualKeyCode},
    prelude::*,
    ui::*,
};

use std::sync::{Arc, Mutex};

use super::Data;

use super::UiCom;
use serde::Deserialize;

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
        data: StateData<'_, GameData<'_, '_>>,
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
                let _forget = scancode;
                match key_code {
                    VirtualKeyCode::F => {
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
                        let to_send = world.write_resource::<Arc<Mutex<Vec<String>>>>();
                        let data = world.write_resource::<Arc<Mutex<Data>>>();

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

    fn on_stop(&mut self, _data: StateData<GameData>) {}
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
