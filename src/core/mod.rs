pub mod components;
#[allow(dead_code)]
pub mod input;
pub mod maths;
pub mod network;
pub mod players;

mod session;
use std::ops::DerefMut;

pub use session::Session;

use bevy::{
    app::PluginGroupBuilder,
    log,
    prelude::*,
};

pub const WORLD_SIZE_Y: f32 = 600.0;
pub const WORLD_SIZE_X: f32 = 800.0;

#[derive(Default)]
/// A plugin that contains universal assets for the Torus engine.
pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        log::info!("Building core plugins...");
        group.add(network::CoreNetworkPlugin::default());
        group.add(players::CorePlayerPlugin::default());
        group.add(CoreAppPlugin::default());
    }
}

#[derive(Default)]
/// A plugin that handles basic universal network events.
struct CoreAppPlugin;

impl Plugin for CoreAppPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.insert_resource(GameTick::default())
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
