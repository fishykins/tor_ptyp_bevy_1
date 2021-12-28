use std::{net::SocketAddr};

use bevy::{log, prelude::*};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};

use crate::core::{
    network::{ClientId, ClientRequest, ServerResponse, ServerMessage},
    Session,
};


// ===============================================================
// ====================== CLIENT NETWORKING ======================
// ===============================================================

/// This plugin handles network initialization and events.
#[derive(Default)]
pub(crate) struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(handle_packets.system())
            .add_system(handle_messages.system())
            .insert_resource(ClientId::default());
    }
}

// ===============================================================
// ========================= SYSTEMS =============================
// ===============================================================

fn startup(mut net: ResMut<NetworkResource>, session: ResMut<Session>) {
    info!("Starting client...");
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, session.port);
    net.connect(socket_address);
    info!("Connecting to {}...", socket_address);
}

fn handle_packets(mut net: ResMut<NetworkResource>, mut network_events: EventReader<NetworkEvent>) {
    for event in network_events.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                Some(_connection) => {
                    log::info!("Connected.");

                    match net.send_message(*handle, ClientRequest::Join) {
                        Ok(msg) => match msg {
                            Some(msg) => {
                                log::error!("Unable to send handshake: {:?}", msg);
                            }
                            None => {}
                        },
                        Err(err) => {
                            log::error!("Unable to send handshake: {:?}", err);
                        }
                    };
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            _ => {}
        }
    }
}

/// Handles explicit messages sent from server..
fn handle_messages(
    mut net: ResMut<NetworkResource>,
    mut commands: Commands,
) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(msg) = channels.recv::<ServerResponse>() {
            match msg {
                ServerResponse::Id(id) => {
                    log::info!("Server has allocated {}", id);
                    commands.insert_resource(ClientId::new(id));
                }
                ServerResponse::Spawn(spawn) => {
                    log::info!("Spawning data: {}", spawn);
                }
            }
        }
        while let Some(msg) = channels.recv::<ServerMessage>() {
            match msg {
                ServerMessage::GoonState(goons) => {
                    commands.insert_resource(goons);
                }
            }
        }
    }
}

// ===============================================================
// ========================= RESOURCES ===========================
// ===============================================================
