use bevy::prelude::*;

use super::AgentEvent;

#[derive(Default)]
pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AgentEvent>();
    }
}