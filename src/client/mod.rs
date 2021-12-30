mod input;
mod network;
mod interface;
mod bridge;
mod player;

use bevy::{prelude::*, app::PluginGroupBuilder, log};

#[derive(Default)]
pub(crate) struct ClientPlugins {}

impl PluginGroup for ClientPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        log::info!("Building client plugins...");
        group.add(network::ClientNetworkPlugin::default());
        group.add(interface::ClientInterfacePlugin::default());
        group.add(player::ClientPlayerPlugin::default());
        group.add(input::ClientInputPlugin::default());
    }
}