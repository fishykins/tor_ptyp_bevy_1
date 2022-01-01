use bevy::{log, prelude::*};
use bevy_networking_turbulence::NetworkResource;
use torus_core::network::messages::{ClientRequest, ServerResponse};

use crate::events::ClientEvent;

/// Handles explicit requests sent from clients.
pub fn handle_requests(
    mut client_events: EventWriter<ClientEvent>,
    mut net: ResMut<NetworkResource>,
) {
    let mut pending_replies = Vec::new();
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(msg) = channels.recv::<ClientRequest>() {
            match msg {
                ClientRequest::Join => {
                    // First, spawn a new player entity.
                    client_events.send(ClientEvent::Spawn(*handle));
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
                log::error!(
                    "Error sending message to client [{}]: {:?}",
                    message.0,
                    error
                );
            }
        }
    }
}
