mod network;
mod players;

use bevy::{prelude::*, log};

use crate::core::AppState;

#[derive(Default)]
pub(crate) struct ServerPlugin {}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        log::info!("Building server plugins...");
        app.add_state(AppState::InGame)
        .add_plugin(network::ServerNetworkPlugin::default())
        .add_plugin(players::ServerPlayersPlugin::default());
    }
}
