use crate::core::network::Broadcast;
use bevy::prelude::*;

use super::systems::{network_init, handle_messages, broadcast_agents};

#[derive(Default)]
pub(crate) struct ServerPlugin {}

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Broadcast { frame: 0 })
            .add_startup_system(network_init.system())
            .add_system_to_stage(CoreStage::PreUpdate, handle_messages.system())
            .add_system_to_stage(CoreStage::PostUpdate, broadcast_agents.system());
    }
}
