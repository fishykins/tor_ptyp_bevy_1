use std::collections::HashMap;

use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;
use torus_core::{agents::Goon, control::Controller, network::messages::{ClientMessage, GoonUpdateMessage, ServerMessage}, flow::GameTick};

/// Takes client controller data and pushes it to goons.
pub fn handle_client_broadcasts(
    mut net: ResMut<NetworkResource>,
    mut query: Query<(&Goon, &mut Controller)>,
) {
    let mut controller_map = HashMap::<u32, Controller>::new();
    for (handle, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(msg) = channels.recv::<ClientMessage>() {
            match msg {
                ClientMessage::Input(client_controller) => {
                    controller_map.insert(*handle, client_controller);
                }
            }
        }
    }

    for (goon, mut controller) in query.iter_mut() {
        let remote_controller = controller_map.remove(&goon.owner);
        if let Some(remote_controller) = remote_controller {
            controller.translation = remote_controller.translation;
            controller.target_direction = remote_controller.target_direction;
            controller.target_look = remote_controller.target_look;
        }
    }
}

/// Handles the broadcast of generic client data, such as position and rotation.
pub fn broadcast_client_data(
    mut net: ResMut<NetworkResource>,
    game_tick: Res<GameTick>,
    query: Query<(&Goon, &Transform)>,
) {
    let mut update_message = GoonUpdateMessage {
        frame: game_tick.frame(),
        goons: Vec::new(),
    };

    for (goon, transform) in query.iter() {
        update_message.goons.push((
            goon.owner,
            Vec2::new(transform.translation.x, transform.translation.y),
        ));
    }
    net.broadcast_message(ServerMessage::GoonState(update_message));
}
