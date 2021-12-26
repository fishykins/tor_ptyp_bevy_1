use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, Packet};

/// A system that handles global network events. These can be sent/received in either direction, hence the ambiguity of server/client.
pub(crate) fn handle_packets(
    mut net: ResMut<NetworkResource>,
    time: Res<Time>,
    mut reader: EventReader<NetworkEvent>,
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                info!("Got packet on [{}]: {}", handle, message);
                if message == "PING" {
                    let message = format!("PONG @ {}", time.seconds_since_startup());
                    match net.send(*handle, Packet::from(message)) {
                        Ok(()) => {
                            info!("Sent PONG");
                        }
                        Err(error) => {
                            info!("PONG send error: {}", error);
                        }
                    }
                }
            }
            NetworkEvent::Connected(handle) => {
                info!("New connection: [{}]", handle);
            }
            _ => {}
        }
    }
}
