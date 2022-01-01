pub mod components;
#[allow(dead_code)]
pub mod input;
#[allow(dead_code)]
pub mod maths;
pub mod network;
pub mod physics;
pub mod players;

mod session;
use std::ops::DerefMut;

pub use session::Session;

use bevy::{log, prelude::*};

pub const WORLD_SIZE_Y: f32 = 600.0;
pub const WORLD_SIZE_X: f32 = 800.0;

#[derive(Default)]
/// A plugin that contains universal assets for the Torus engine.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        log::info!("Building core plugins...");
        app.add_plugin(network::CoreNetworkPlugin::default())
            .add_plugin(players::CorePlayerPlugin::default())
            .add_plugin(physics::PhysicsPlugin::default())
            .insert_resource(GameTick::default())
            .add_system_to_stage(CoreStage::Last, update.system());
    }
}

fn update(mut game_tick: ResMut<GameTick>) {
    game_tick.deref_mut().next();
}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================

#[derive(Debug, Default, Clone)]
pub struct GameTick {
    frame: u64,
}

impl GameTick {
    pub fn next(&mut self) {
        self.frame += 1;
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    InGame,
    Loading,
}
