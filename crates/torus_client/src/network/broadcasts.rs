use std::collections::HashMap;

use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;
use torus_core::{
    agents::Agent,
    control::Controller,
    flow::GameTick,
    network::{
        messages::{AgentData, ClientMessage, ServerMessage},
        Remote,
    },
    physics::Rigidbody,
};

use crate::agents::AgentEvent;

pub fn broadcast_client_data(mut net: ResMut<NetworkResource>, controller: Query<&Controller>) {
    for controller in controller.iter() {
        net.broadcast_message(ClientMessage::Input(controller.clone()));
    }
}

pub fn handle_server_broadcasts(
    mut net: ResMut<NetworkResource>,
    mut agent_events: EventWriter<AgentEvent>,
    tick: Res<GameTick>,
    mut query: Query<(&mut Rigidbody<Remote>, &Agent)>,
) {
    let mut pending_agent_updates = HashMap::<u32, AgentData>::new();
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(msg) = channels.recv::<ServerMessage>() {
            match msg {
                ServerMessage::AgentState(update_message) => {
                    for (handle, data) in update_message.agents.iter() {
                        pending_agent_updates.insert(*handle, data.clone());
                    }
                }
            }
        }
    }
    for (mut body, agent) in query.iter_mut() {
        if let Some(data) = pending_agent_updates.remove(&agent.owner) {
            body.position = data.position;
            body.rotation = data.rotation;
            body.set_last_update(tick.frame());
        }
    }

    for (handle, data) in pending_agent_updates.iter() {
        // Spawn this agent
        agent_events.send(AgentEvent::Spawn(*handle, data.clone()));
    }
}
