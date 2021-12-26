use bevy::{log, prelude::*};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};

use crate::network::ClientMessage;

pub(crate) fn handle_packets(
    mut net: ResMut<NetworkResource>,
    mut network_events: EventReader<NetworkEvent>,
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

                    log::info!("Sending Handshake on [{}]", handle);
                    match net
                        .send_message(*handle, ClientMessage::Handshake("Hello goon".to_string()))
                    {
                        Ok(msg) => match msg {
                            Some(msg) => {
                                error!("Unable to send Handshake: {:?}", msg);
                            }
                            None => {}
                        },
                        Err(err) => {
                            error!("Unable to send Handshake: {:?}", err);
                        }
                    };
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            _ => {}
        }
    }
}
