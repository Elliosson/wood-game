use super::Data;
use crate::systems::{CameraSystem, MapSystem};
use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, Entity, World},
    error::Error,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct GameBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for GameBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(CameraSystem, "camera_system", &[]);
        builder.add(MapSystem, "map_system", &[]);

        // hashmap used to do the link between network entity and game entity
        world.insert(HashMap::<(u32, i32), Entity>::new());
        Ok(())
    }
}

pub struct NetworkBundle {
    pub protect_data: Arc<Mutex<Data>>,
    pub to_send: Arc<Mutex<Vec<String>>>,
}

impl<'a, 'b> SystemBundle<'a, 'b> for NetworkBundle {
    fn build(
        self,
        world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        world.insert(self.protect_data.clone());
        world.insert(self.to_send.clone());

        //for noew send speudo here
        let mut send_guard = self.to_send.lock().unwrap();
        send_guard.push(format!("register {}", "rerer"));
        Ok(())
    }
}
