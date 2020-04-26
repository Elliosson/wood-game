use super::{components::*, Data, UiCom};
use crate::systems::*;
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
        builder.add(PlayerInputSystem, "player_input_system", &[]);
        builder.add(DeserialiseSystem, "deserialise_system", &[]);
        builder.add(InteractionUiSystem::default(), "interaction_ui_system", &[]);
        builder.add(InventoryUiSystem::default(), "inventory_ui_system", &[]);
        builder.add(BuildUiSystem::default(), "build_ui_system", &[]);

        world.insert(UiCom::default());
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
        _builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        world.insert(self.protect_data.clone());
        world.insert(self.to_send.clone());
        world.insert(PlayerInfo {
            inventaire: Vec::new(),
            close_interations: Vec::new(),
            my_info: MyInfo {
                pos: Position { x: 0, y: 0 },
                hp: 0,
                max_hp: 0,
                player_log: vec![],
            },
            possible_builds: Vec::new(),
            equipement: Vec::new(),
            combat_stats: Default::default(),
        });

        Ok(())
    }
}
