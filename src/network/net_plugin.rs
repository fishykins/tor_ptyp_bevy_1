use bevy::prelude::*;
use bevy_networking_turbulence::{
    ConnectionChannelsBuilder, NetworkResource, NetworkingPlugin as TurbulenceNetPlugin,
};

use bevy::prelude::ResMut;

use super::{
    ClientMessage, GameStateMessage, CLIENT_STATE_MESSAGE_SETTINGS, GAME_STATE_MESSAGE_SETTINGS,
};

#[derive(Default)]
/// A plugin that handles basic universal network events.
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_plugin(TurbulenceNetPlugin::default())
            .add_startup_system(setup.system());
        //.add_system(handle_packets.system());
    }
}

pub fn setup(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_STATE_MESSAGE_SETTINGS)
            .unwrap();
        builder
            .register::<GameStateMessage>(GAME_STATE_MESSAGE_SETTINGS)
            .unwrap();
    });
}
