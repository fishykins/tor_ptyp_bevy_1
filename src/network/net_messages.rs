use bevy::math::Vec3;
use serde::{Deserialize, Serialize};

// Housing for all the message types that can be sent between the client and server.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Handshake(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameStateMessage {
    pub frame: u32,
    pub agents: Vec<(u32, Vec3)>,
}