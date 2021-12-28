use std::collections::HashMap;

use bevy::{prelude::*, log};

use crate::core::{components::Goon, network::GoonUpdateMessage};

// ===============================================================
// ====================== CLIENT NETWORKING ======================
// ===============================================================

#[derive(Default)]
pub(crate) struct ClientPlayerPlugin;

impl Plugin for ClientPlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(PlayerResource::default())
            .add_system_to_stage(CoreStage::PreUpdate, update_players.system());
    }
}

// ===============================================================
// ========================= SYSTEMS =============================
// ===============================================================

fn update_players(
    broadcast: Option<ResMut<GoonUpdateMessage>>,
    mut commands: Commands,
    mut query: Query<(&mut Transform, &Goon)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(mut update) = broadcast {
        let mut to_spawn: HashMap<u32, Vec3> = HashMap::new();
        for (mut transform, goon) in query.iter_mut() {
            if let Some(index) = update
                .goons
                .iter()
                .position(|&update| update.0 == goon.owner())
            {
                let (_id, translation) = update.goons.remove(index);
                transform.translation = Vec3::new(translation.x, translation.y, 0.0);
            }
        }

        // These should be unspawned players.
        for (id, translation) in update.goons.drain(..) {
            if !to_spawn.contains_key(&id) {
                to_spawn.insert(id, Vec3::new(translation.x, translation.y, 0.0));
            }
        }

        for (id, translation) in to_spawn {
            log::info!("Spawning player {}", id);
            commands
                .spawn_bundle(SpriteBundle {
                    material: materials.add(
                        Color::rgb(0.8 - (id as f32 / 5.0), 0.2, 0.2 + (id as f32 / 5.0)).into(),
                    ),
                    transform: Transform::from_translation(translation),
                    sprite: Sprite::new(Vec2::new(30.0, 30.0)),
                    ..Default::default()
                })
                .insert(Goon::new(id));
        }
    }
}

// ===============================================================
// ======================= COMPONENTS ============================
// ===============================================================

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
pub struct PlayerResource {}

impl Default for PlayerResource {
    fn default() -> Self {
        Self {}
    }
}
