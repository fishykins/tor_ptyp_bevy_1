use std::{net::SocketAddr, collections::HashMap};

use bevy::{log, prelude::*};
use bevy_networking_turbulence::{
    NetworkEvent, NetworkResource, NetworkingPlugin as TurbulenceNetPlugin,
};

use crate::{
    core::{
        network::{ClientRequest, ServerResponse, ClientMessage, GoonUpdateMessage, ServerMessage},
        Session, GameTick, components::{Goon, Controller},
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
            .add_system_to_stage(CoreStage::PreUpdate, handle_events.system())
            .add_system_to_stage(CoreStage::PreUpdate, handle_requests.system())
            .add_system_to_stage(CoreStage::PreUpdate, handle_client_broadcasts.system())
            .add_system_to_stage(CoreStage::PostUpdate, broadcast_client_data.system());
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
fn handle_events(mut net: ResMut<NetworkResource>, mut network_events: EventReader<NetworkEvent>) {
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

/// Handles explicit requests sent from clients.
fn handle_requests(mut commands: Commands, mut net: ResMut<NetworkResource>) {
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

/// Takes client controller data and pushes it to goons.
fn handle_client_broadcasts(mut net: ResMut<NetworkResource>, mut query: Query<(&Goon, &mut Controller)>) {
    let mut controller_map = HashMap::<u32, Controller>::new();
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(msg) = channels.recv::<ClientMessage>() {
            match msg {
                ClientMessage::Input(client_controller) => {
                    controller_map.insert(*handle, client_controller);
                },
            }
        }
    }

    for (goon, mut controller) in query.iter_mut() {
        let remote_controller = controller_map.remove(&goon.owner());
        if let Some(remote_controller) = remote_controller {
            controller.forward = remote_controller.forward;
            controller.lateral = remote_controller.lateral;
        }
    }
}

/// Handles the broadcast of generic client data, such as position and rotation.
fn broadcast_client_data(
    mut net: ResMut<NetworkResource>,
    game_tick: Res<GameTick>,
    query: Query<(&Goon, &Transform)>,
) {
    let mut update_message = GoonUpdateMessage {
        frame: game_tick.frame(),
        goons: Vec::new(),
    };

    for (goon, transform) in query.iter() {
        update_message
            .goons
            .push((goon.owner(), Vec2::new(transform.translation.x, transform.translation.y)));
    }
    net.broadcast_message(ServerMessage::GoonState(update_message));
}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
