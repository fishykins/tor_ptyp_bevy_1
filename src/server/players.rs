use crate::core::{
    components::{Controller, Goon},
    network::Local,
    physics::Body,
    players::Biped,
    WORLD_SIZE_X, WORLD_SIZE_Y,
};
use bevy::prelude::*;
use rand::Rng;

// ===============================================================
// ====================== SERVER PLAYERS =========================
// ===============================================================

#[derive(Default)]
/// A plugin that handles players from the server.
pub struct ServerPlayersPlugin;

impl Plugin for ServerPlayersPlugin {
    fn build(&self, _app: &mut bevy::prelude::AppBuilder) {

    }
}

// ===============================================================
// ======================== SYSTEMS ==============================
// ===============================================================


// ===============================================================
// ======================== COMPONENTS ===========================
// ===============================================================

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub goon: Goon,
    pub transform: Transform,
    pub body: Body<Local>,
    pub controller: Controller,
    pub biped: Biped,
}

impl PlayerBundle {
    pub fn new(handle: u32) -> Self {
        let mut rng = rand::thread_rng();
        let pos_x = rng.gen_range(0.0..WORLD_SIZE_X) as f32;
        let pos_y = rng.gen_range(0.0..WORLD_SIZE_Y) as f32;

        Self {
            goon: Goon::new(handle),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
            body: Body::<Local>::from_translation(Vec2::new(pos_x, pos_y)),
            controller: Controller::default(),
            biped: Biped::default(),
        }
    }
}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
