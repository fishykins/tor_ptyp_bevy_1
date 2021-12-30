use crate::core::{
    components::{Controller, Goon},
    WORLD_SIZE_X, WORLD_SIZE_Y, players::PLAYER_SPEED,
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
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_to_stage(CoreStage::Update, update_goons.system());
    }
}

// ===============================================================
// ======================== SYSTEMS ==============================
// ===============================================================
fn update_goons(time: Res<Time>, mut query: Query<(&mut Transform, &Controller), With<Goon>>) {
    for (mut transform, controller) in query.iter_mut() {
        if controller.forward != 0.0 || controller.lateral != 0.0 {
            let step_move = Vec3::new(controller.lateral, controller.forward, 0.0).normalize() * time.delta_seconds() * PLAYER_SPEED;
            transform.translation += step_move;
        }

        if transform.translation.x > WORLD_SIZE_X {
            transform.translation.x = 0.1;
        } else if transform.translation.x < 0.0 {
            transform.translation.x = WORLD_SIZE_X;
        }

        if transform.translation.y > WORLD_SIZE_Y {
            transform.translation.y = 0.1;
        } else if transform.translation.y < 0.0 {
            transform.translation.y = WORLD_SIZE_Y;
        }
    }
}

// ===============================================================
// ======================== COMPONENTS ===========================
// ===============================================================

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub goon: Goon,
    pub transform: Transform,
    pub controller: Controller,
}

impl PlayerBundle {
    pub fn new(handle: u32) -> Self {
        let mut rng = rand::thread_rng();
        let pos_x = rng.gen_range(0.0..WORLD_SIZE_X) as f32;
        let pos_y = rng.gen_range(0.0..WORLD_SIZE_Y) as f32;

        Self {
            goon: Goon::new(handle),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
            controller: Controller::default(),
        }
    }
}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
