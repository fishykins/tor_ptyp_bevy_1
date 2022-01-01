use std::collections::HashMap;

use bevy::{log, prelude::*};

use crate::core::{
    components::{Controller, Goon},
    network::{ClientId, GoonUpdateMessage, Local, Remote},
    physics::Body,
    players::Biped,
    AppState, GameTick,
};

use super::assets::TextureAssets;

// ===============================================================
// ====================== CLIENT NETWORKING ======================
// ===============================================================

#[derive(Default)]
pub(crate) struct ClientPlayerPlugin;

impl Plugin for ClientPlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::on_update(AppState::InGame).with_system(
                update_player_remote
                    .system()
                    .label("update_player_remote"),
            ),
        );
    }
}

// ===============================================================
// ========================= SYSTEMS =============================
// ===============================================================

/// Updates the player's remote position based on the server's position.
fn update_player_remote(
    broadcast: Option<ResMut<GoonUpdateMessage>>,
    client_id: Res<ClientId>,
    game_tick: Res<GameTick>,
    textures: Res<TextureAssets>,
    mut commands: Commands,
    mut query: Query<(&mut Body<Remote>, &Goon)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(mut update) = broadcast {
        let mut to_spawn: HashMap<u32, Vec3> = HashMap::new();
        for (mut remote, goon) in query.iter_mut() {
            if let Some(index) = update
                .goons
                .iter()
                .position(|&update| update.0 == goon.owner())
            {
                let (_id, translation) = update.goons.remove(index);
                remote.translation = Vec2::new(translation.x, translation.y);
                remote.set_last_update(game_tick.frame());
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
            let sprite_handle = textures.doddy.clone();
            let mut transform = Transform::from_translation(translation);
            transform.scale = Vec3::new(0.5, 0.5, 1.0);
            let mut entity = commands.spawn_bundle(SpriteBundle {
                material: materials.add(sprite_handle.into()),
                transform,
                ..Default::default()
            });
            entity
                .insert(Goon::new(id))
                .insert(Body::<Remote>::default())
                .insert(Biped::default());
            if client_id.is_equal(id) {
                entity
                    .insert(Player::default())
                    .insert(Controller::default())
                    .insert(Body::<Local>::from_translation(Vec2::new(
                        translation.x,
                        translation.y,
                    )));
            }
        }
    }
}

// ===============================================================
// ======================= COMPONENTS ============================
// ===============================================================

/// A simple tag to help identify our player quickly.
#[derive(Debug, Default)]
pub struct Player {}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
