mod network;
mod players;

use bevy::{prelude::*, log};

#[derive(Default)]
pub(crate) struct ServerPlugins {}

impl PluginGroup for ServerPlugins {
    fn build(&mut self, group: &mut bevy::app::PluginGroupBuilder) {
        log::info!("Building server plugins...");
        group.add(network::ServerNetworkPlugin::default());
        group.add(players::ServerPlayersPlugin::default());
    }
}
