use bevy::prelude::*;
use torus_core::flow::AppState;

use super::{spawn::spawn_agents, AgentEvent};

#[derive(Default)]
pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AgentEvent>();
        app.add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(spawn_agents.system())
                .label("spawn")
                .before("simulation")
                .after("receive"),
        );
    }
}
