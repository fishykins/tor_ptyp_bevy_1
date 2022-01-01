use bevy::{prelude::*, log};
use bevy_networking_turbulence::{NetworkResource, NetworkEvent};


/// Handles network events, such as connections and disconects.
pub fn handle_events(mut net: ResMut<NetworkResource>, mut network_events: EventReader<NetworkEvent>) {
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