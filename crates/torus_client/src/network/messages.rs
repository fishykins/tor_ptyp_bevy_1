use bevy::{prelude::*, log};
use bevy_networking_turbulence::NetworkResource;
use torus_core::network::{messages::{ServerResponse, ServerMessage}, data::ClientId};

use crate::agents::AgentEvent;

/// Handles explicit messages sent from server..
pub fn handle_messages(mut net: ResMut<NetworkResource>, mut commands: Commands, mut events: EventWriter<AgentEvent>) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(msg) = channels.recv::<ServerResponse>() {
            match msg {
                ServerResponse::Id(id) => {
                    log::info!("Server has allocated us ClientId({}).", id);
                    commands.insert_resource(ClientId::new(id));
                }
                ServerResponse::Spawn(spawn) => {
                    log::info!("Spawning data: {}", spawn);
                }
            }
        }
        while let Some(msg) = channels.recv::<ServerMessage>() {
            match msg {
                ServerMessage::AgentState(agents) => {
                    events.send(AgentEvent::Update(agents));
                }
            }
        }
    }
}