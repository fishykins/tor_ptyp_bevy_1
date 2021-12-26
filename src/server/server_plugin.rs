use super::server_broadcast::broadcast_agents;
use super::{server_init::startup, server_messages::handle_messages};
use crate::network::Broadcast;
use bevy::prelude::*;

#[derive(Default)]
pub(crate) struct ServerPlugin {}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Broadcast { frame: 0 })
            .add_startup_system(startup.system())
            .add_system_to_stage(CoreStage::PreUpdate, handle_messages.system())
            .add_system_to_stage(CoreStage::PostUpdate, broadcast_agents.system());
    }
}
