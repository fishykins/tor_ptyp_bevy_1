use crate::{
    core::components::Agent,
    network::{ClientMessage, GameStateMessage},
};
use bevy::{log, prelude::*};
use bevy_networking_turbulence::NetworkResource;
use rand::Rng;

pub(crate) fn handle_messages(
    mut net: ResMut<NetworkResource>,
    mut commands: Commands,
    mut agents: Query<(&Agent, &mut Transform)>,
) {
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
                    commands.spawn_bundle((
                        Agent {
                            controller: *handle,
                        },
                        Transform::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
                    ));
                }
                ClientMessage::Input(input) => {
                    for (agent, mut transform) in agents.iter_mut() {
                        if agent.controller == *handle {
                            match input.0 {
                                crate::core::input::Movement::Forward => transform.translation.y += input.1,
                                crate::core::input::Movement::Right => transform.translation.x += input.1,
                            }
                            break;
                        }
                    }
                }
            }
        }
        while let Some(_state_message) = channels.recv::<GameStateMessage>() {
            log::error!("GameStateMessage received on [{}]", handle);
        }
    }
}
