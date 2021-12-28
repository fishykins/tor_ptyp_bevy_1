use std::net::SocketAddr;

use bevy::{log, prelude::*};
use bevy_networking_turbulence::{
    NetworkEvent, NetworkResource, NetworkingPlugin as TurbulenceNetPlugin,
};

use crate::{
    core::{
        network::{ClientRequest, ServerResponse},
        Session,
    },
    server::players::PlayerBundle,
};

// ===============================================================
// ===================== CORE NETWORKING =========================
// ===============================================================

#[derive(Default)]
/// A plugin that handles basic universal network events.
pub struct ServerNetworkPlugin;

impl Plugin for ServerNetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_plugin(TurbulenceNetPlugin::default())
            .add_startup_system(startup.system())
            .add_system(handle_packets.system())
            .add_system(handle_messages.system());
    }
}

// ===============================================================
// ======================== SYSTEMS ==============================
// ===============================================================

fn startup(mut net: ResMut<NetworkResource>, session: ResMut<Session>) {
    info!("Starting server...");
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, session.port);
    net.listen(socket_address, None, None);
    info!("Listening on {}", socket_address);
}

/// Handles network events, such as connections and disconects.
fn handle_packets(mut net: ResMut<NetworkResource>, mut network_events: EventReader<NetworkEvent>) {
    for event in network_events.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                Some(connection) => match connection.remote_address() {
                    Some(remote_address) => {
                        log::info!(
                            "Incoming connection on [{}] from [{}]",
                            handle,
                            remote_address
                        );
                    }
                    None => {
                        log::debug!("Connected on [{}]", handle);
                    }
                },
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            NetworkEvent::Disconnected(handle) => {
                log::info!("[{}] has dissconnected.", handle);
            }
            NetworkEvent::Error(handle, error) => {
                log::error!("Network error on connection [{}]: {:?}", handle, error);
            }
            _ => {}
        }
    }
}

/// Handles explicit messages sent from clients.
fn handle_messages(mut commands: Commands, mut net: ResMut<NetworkResource>) {
    let mut pending_replies = Vec::new();
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(msg) = channels.recv::<ClientRequest>() {
            match msg {
                ClientRequest::Join => {
                    // First, spawn a new player entity.
                    commands.spawn_bundle(PlayerBundle::new(*handle));
                    // Send a reply to the client, containing their unique ID.
                    log::info!("Handing id to client [{}]", *handle);
                    pending_replies.push((*handle, ServerResponse::Id(*handle)));
                }
                ClientRequest::Spawn => {
                    log::info!("Sending spawn to client [{}]", *handle);
                    pending_replies.push((*handle, ServerResponse::Spawn(Vec2::splat(32.0))));
                }
            }
        }
    }
    for message in pending_replies {
        match net.send_message(message.0, message.1) {
            Ok(_) => {}
            Err(error) => {
                log::error!("Error sending message to client [{}]: {:?}", message.0, error);
            }
        }
    }
}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
