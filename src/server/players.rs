use bevy::{prelude::*, log};

use crate::core::components::Controller;

// ===============================================================
// ====================== SERVER PLAYERS =========================
// ===============================================================

#[derive(Default)]
/// A plugin that handles players from the server.
pub struct ServerPlayersPlugin;

impl Plugin for ServerPlayersPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(handle_players.system());
    }
}

// ===============================================================
// ======================== SYSTEMS ==============================
// ===============================================================

fn handle_players(query: Query<(&Agent, &Transform)>,) {
    for (_agent, _transform) in query.iter() {
        
    }
}


// ===============================================================
// ========================== COMPONENTS =========================
// ===============================================================

/// We can store all of a players data in an Agent. 
#[derive(Debug, Default, Clone)]
pub struct Agent {
    /// The client who owns this agent.
    owner: u32,
}

impl Agent {
    pub fn owner(&self) -> u32 {
        self.owner.clone()
    }
}

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub agent: Agent,
    pub controller: Controller,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(handle: u32) -> Self {
        Self {
            agent: Agent { owner: handle },
            controller: Controller::default(),
            transform: Transform::from_translation(Vec3::ZERO),
        }
    }
}