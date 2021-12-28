pub mod components;
pub mod input;
pub mod network;

mod session;
pub use session::Session;

use bevy::{app::PluginGroupBuilder, prelude::PluginGroup, log};

#[derive(Default)]
/// A plugin that contains universal assets for the Torus engine.
pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        log::info!("Building core plugins...");
        group.add(network::CoreNetworkPlugin::default());
    }
}
