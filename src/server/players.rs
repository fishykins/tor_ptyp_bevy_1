use crate::core::{
    components::Goon,
    network::{GoonUpdateMessage, ServerMessage},
    GameTick, WORLD_SIZE_X, WORLD_SIZE_Y,
};
use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;
use rand::Rng;

// ===============================================================
// ====================== SERVER PLAYERS =========================
// ===============================================================

#[derive(Default)]
/// A plugin that handles players from the server.
pub struct ServerPlayersPlugin;

impl Plugin for ServerPlayersPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_system(handle_players.system())
            .add_system_to_stage(CoreStage::PreUpdate, handle_players.system())
            .add_system_to_stage(CoreStage::PostUpdate, broadcast_players.system());
    }
}

// ===============================================================
// ======================== SYSTEMS ==============================
// ===============================================================

fn handle_players(time: Res<Time>, mut query: Query<(&RunnyBoi, &mut Transform)>) {
    for (boi, mut transform) in query.iter_mut() {
        let mut translation = transform.translation + (boi.velocity * time.delta_seconds());
        let mut x = translation.x as i32 % WORLD_SIZE_X as i32;
        let mut y = translation.y as i32 % WORLD_SIZE_Y as i32;
        if x < 0 {
            x += WORLD_SIZE_X as i32;
        }
        if y < 0 {
            y += WORLD_SIZE_Y as i32;
        }
        translation.x = x as f32;
        translation.y = y as f32;
        transform.translation = translation;
    }
}

fn broadcast_players(
    mut net: ResMut<NetworkResource>,
    game_tick: Res<GameTick>,
    query: Query<(&Goon, &Transform)>,
) {
    let mut update_message = GoonUpdateMessage {
        frame: game_tick.frame(),
        goons: Vec::new(),
    };

    for (goon, transform) in query.iter() {
        update_message
            .goons
            .push((goon.owner(), transform.translation.into()));
    }
    net.broadcast_message(ServerMessage::GoonState(update_message));
}

// ===============================================================
// ======================== COMPONENTS ===========================
// ===============================================================

#[derive(Debug, Default, Clone)]
pub struct RunnyBoi {
    pub velocity: Vec3,
}

#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub boi: RunnyBoi,
    pub goon: Goon,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(handle: u32) -> Self {
        let mut rng = rand::thread_rng();
        let vel_x = rng.gen_range(-0.5..0.5);
        let vel_y = rng.gen_range(-0.5..0.5);
        let pos_x = rng.gen_range(0.0..WORLD_SIZE_X) as f32;
        let pos_y = rng.gen_range(0.0..WORLD_SIZE_Y) as f32;
        let boi = RunnyBoi {
            velocity: 400.0 * Vec3::new(vel_x, vel_y, 0.0).normalize(),
        };

        Self {
            boi,
            goon: Goon::new(handle),
            transform: Transform::from_translation(Vec3::new(pos_x, pos_y, 1.0)),
        }
    }
}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
