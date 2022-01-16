use bevy::prelude::*;
use bevy_networking_turbulence::{
    ConnectionChannelsBuilder, NetworkResource, NetworkingPlugin as TurbulenceNetPlugin,
};

use super::{messages::*, protocols::*};
/// Initializes the core network features, such as message channels and the network plugin.
#[derive(Default)]
/// A plugin that handles basic universal network events.
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let net_plugin = TurbulenceNetPlugin::default();
        //net_plugin.idle_timeout_ms = Some(5000);
        //net_plugin.auto_heartbeat_ms = Some(2000);

        // This is global.
        app.add_plugin(net_plugin)
            .add_startup_system(setup.system());
    }
}

/// Universal initialization for network systems.
pub fn setup(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_MESSAGE_SETTINGS)
            .unwrap();
        builder
            .register::<ClientRequest>(CLIENT_REQUEST_SETTINGS)
            .unwrap();
        builder
            .register::<ServerMessage>(SERVER_MESSAGE_SETTINGS)
            .unwrap();
        builder
            .register::<ServerResponse>(SERVER_RESPONSE_SETTINGS)
            .unwrap();
    });
}
