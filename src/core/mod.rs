pub mod components;
#[allow(dead_code)]
pub mod input;
pub mod network;
pub mod players;

mod session;
pub use session::Session;

use bevy::{app::PluginGroupBuilder, log, prelude::PluginGroup};

#[derive(Default)]
/// A plugin that contains universal assets for the Torus engine.
pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        log::info!("Building core plugins...");
        group.add(network::CoreNetworkPlugin::default());
        group.add(players::CorePlayerPlugin::default());
    }
}

pub const WORLD_SIZE_X: f32 = 800.0;
pub const WORLD_SIZE_Y: f32 = 600.0;

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