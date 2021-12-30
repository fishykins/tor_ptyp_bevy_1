use crate::core::{
    components::{Controller, Goon},
    players::GBodyLocal,
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
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system_to_stage(CoreStage::Update, update_player_transforms.system().label("update_player_transforms"));
    }
}

// ===============================================================
// ======================== SYSTEMS ==============================
// ===============================================================
fn update_player_transforms(mut query: Query<(&mut Transform, &GBodyLocal), With<Goon>>) {
    for (mut transform, gbody) in query.iter_mut() {
        transform.translation = gbody.translation;
    }
}

// ===============================================================
// ======================== COMPONENTS ===========================
// ===============================================================

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub goon: Goon,
    pub transform: Transform,
    pub gbody: GBodyLocal,
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
            gbody: GBodyLocal::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
            controller: Controller::default(),
        }
    }
}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
