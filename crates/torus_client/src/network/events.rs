use bevy::{prelude::*, log};
use bevy_networking_turbulence::{NetworkResource, NetworkEvent};
use torus_core::network::messages::ClientRequest;


pub fn handle_events(mut net: ResMut<NetworkResource>, mut network_events: EventReader<NetworkEvent>) {
    for event in network_events.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                Some(_connection) => {
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
            NetworkEvent::Disconnected(_handle) => {
                log::info!("Disconnected.");
            }
            _ => {}
        }
    }
}
