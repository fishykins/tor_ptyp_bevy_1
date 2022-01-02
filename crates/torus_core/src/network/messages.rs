use bevy::math::Vec2;
use serde::{Deserialize, Serialize};
use crate::control::Controller;

/// Client FYI broadcasts, such as input data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Input(Controller),
}

/// Client requests that require server response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientRequest {
    Join,
    Spawn,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {
    AgentState(AgentUpdateMessage)
}

/// A response to a specific client's request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerResponse {
    Id(u32),
    Spawn(Vec2),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentUpdateMessage {
    pub frame: u64,
    // agent id, position
    pub agents: Vec<(u32, Vec2)>,
}