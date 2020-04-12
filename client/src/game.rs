use amethyst::{
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    ui::{UiCreator, UiEvent, UiEventType, UiFinder, UiText},
};

use log::info;

pub struct MyGame;

impl SimpleState for MyGame {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let _ = Some(world.exec(|mut creator: UiCreator<'_>| creator.create("game_ui.ron", ())));
    }
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }

    fn on_stop(&mut self, data: StateData<GameData>) {}
}
