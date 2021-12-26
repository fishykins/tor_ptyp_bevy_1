use crate::{network::{GameStateMessage, ClientMessage}};
use bevy::{prelude::*, log};
use bevy_networking_turbulence::NetworkResource;

pub(crate) fn handle_messages(mut net: ResMut<NetworkResource>) {
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(client_message) = channels.recv::<ClientMessage>() {
            log::debug!(
                "ClientMessage received on [{}]: {:?}",
                handle,
                client_message
            );
            match client_message {
                ClientMessage::Handshake(id) => {
                    log::info!("Client [{}] connected on [{}]", id, handle);
                }
            }
        }
        while let Some(_state_message) = channels.recv::<GameStateMessage>() {
            log::error!("GameStateMessage received on [{}]", handle);
        }
    }
}