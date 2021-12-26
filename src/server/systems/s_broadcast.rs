use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;

use crate::{network::{GameStateMessage, Broadcast}, core::components::Player};

pub(crate) fn broadcast_agents(
    mut state: ResMut<Broadcast>,
    mut net: ResMut<NetworkResource>,
    agent_query: Query<(Entity, &Transform), With<Player>>,
) {
    let mut message = GameStateMessage {
        frame: state.frame,
        agents: Vec::new(),
    };
    state.frame += 1;

    for (entity, transform) in agent_query.iter() {
        message
            .agents
            .push((entity.id(), transform.translation));
    }

    net.broadcast_message(message);
}