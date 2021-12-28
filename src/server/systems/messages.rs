use crate::{
    core::{
        //components::{Agent, Controller},
        network::{ClientMessage, ServerMessage},
    },
    server::bundles::PlayerBundle,
};
use bevy::{log, prelude::*};
use bevy_networking_turbulence::NetworkResource;
use rand::Rng;

pub(crate) fn handle_messages(
    mut net: ResMut<NetworkResource>,
    mut commands: Commands,
    //mut agents: Query<(&Agent, &mut Controller)>,
) {
    let mut pending_messages = Vec::new();

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
                    log::info!("Client [{}] connected...", id);
                    let mut rng = rand::thread_rng();
                    let pos_x = rng.gen_range(0..400) as f32;
                    let pos_y = rng.gen_range(0..400) as f32;
                    log::info!("Spawning agent {} at [{},{}]", *handle, pos_x, pos_y);
                    commands.spawn_bundle(PlayerBundle::new(*handle, Vec2::new(pos_x, pos_y)));
                    pending_messages.push((*handle, ServerMessage::Handshake((*handle, "Welcome".to_string()))));

                }
                // ClientMessage::Movement(movement) => {
                //     for (agent, mut controller) in agents.iter_mut() {
                //         if agent.owner == *handle {
                //             controller.movement = movement;
                //             log::info!("Client [{}] sent a movement message", *handle);
                //             break;
                //         }
                //     }
                // }
            }
        }
    }

    for (handle, message) in pending_messages {
        match net.send_message(handle, message) {
            Ok(_) => {},
            Err(e) => log::error!("Failed to send message to [{}]: {}", handle, e),
        }
    }
}
