use bevy::{log, prelude::*};
use bevy_networking_turbulence::{
    ConnectionChannelsBuilder, NetworkEvent, NetworkResource,
    NetworkingPlugin as TurbulenceNetPlugin,
};

use bevy::prelude::ResMut;

use crate::core::resources::Session;

use super::{
    ClientMessage, GameStateMessage, CLIENT_STATE_MESSAGE_SETTINGS, GAME_STATE_MESSAGE_SETTINGS,
};

#[derive(Default)]
/// A plugin that handles basic universal network events.
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_plugin(TurbulenceNetPlugin::default())
            .add_startup_system(setup.system())
            .add_system(handle_packets.system());
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

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    mut network_events: EventReader<NetworkEvent>,
    session: Res<Session>,
) {
    for event in network_events.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                Some(connection) => {
                    match connection.remote_address() {
                        Some(remote_address) => {
                            log::info!(
                                "Incoming connection on [{}] from [{}]",
                                handle,
                                remote_address
                            );
                        }
                        None => {
                            log::info!("Connected on [{}]", handle);
                        }
                    }

                    if session.is_client() {
                        log::info!("Sending Hello on [{}]", handle);
                        match net.send_message(*handle, ClientMessage::Text("test".to_string())) {
                            Ok(msg) => match msg {
                                Some(msg) => {
                                    error!("Unable to send Hello: {:?}", msg);
                                }
                                None => {}
                            },
                            Err(err) => {
                                error!("Unable to send Hello: {:?}", err);
                            }
                        };
                    }
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            _ => {}
        }
    }
}
