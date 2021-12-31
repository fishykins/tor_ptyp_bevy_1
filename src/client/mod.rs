mod assets;
mod bridge;
mod input;
mod interface;
mod network;
mod player;

use bevy::prelude::*;

use crate::core::AppState;

#[derive(Default)]
pub(crate) struct ClientPlugin {}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(AppState::Loading)
            .add_plugin(assets::ClientAssetsPlugin)
            .add_plugin(network::ClientNetworkPlugin)
            .add_plugin(interface::ClientInterfacePlugin)
            .add_plugin(input::ClientInputPlugin)
            .add_plugin(player::ClientPlayerPlugin);
    }
}
