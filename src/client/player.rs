use std::collections::HashMap;

use bevy::{log, prelude::*};

use crate::core::{
    components::{Controller, Goon},
    network::{ClientId, GoonUpdateMessage},
    players::PLAYER_SPEED,
    GameTick,
};

// ===============================================================
// ====================== CLIENT NETWORKING ======================
// ===============================================================

#[derive(Default)]
pub(crate) struct ClientPlayerPlugin;

impl Plugin for ClientPlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            CoreStage::Update,
            update_player_remote
                .system()
                .label("update_player_remote")
                .before("update_player_transforms"),
        )
        .add_system_to_stage(
            CoreStage::Update,
            update_player_local
                .system()
                .label("update_player_local")
                .before("update_player_transforms"),
        )
        .add_system_to_stage(
            CoreStage::Update,
            update_player_transforms
                .system()
                .label("update_player_transforms"),
        );
    }
}

// ===============================================================
// ========================= SYSTEMS =============================
// ===============================================================

fn update_player_remote(
    broadcast: Option<ResMut<GoonUpdateMessage>>,
    client_id: Res<ClientId>,
    game_tick: Res<GameTick>,
    mut commands: Commands,
    mut query: Query<(&mut GBodyRemote, &Goon)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if let Some(mut update) = broadcast {
        let mut to_spawn: HashMap<u32, Vec3> = HashMap::new();
        for (mut gbody_remote, goon) in query.iter_mut() {
            if let Some(index) = update
                .goons
                .iter()
                .position(|&update| update.0 == goon.owner())
            {
                let (_id, translation) = update.goons.remove(index);
                gbody_remote.translation = Vec3::new(translation.x, translation.y, 0.0);
                gbody_remote.tick = game_tick.frame();
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
            let mut entity = commands.spawn_bundle(SpriteBundle {
                material: materials
                    .add(Color::rgb(0.8 - (id as f32 / 5.0), 0.2, 0.2 + (id as f32 / 5.0)).into()),
                transform: Transform::from_translation(translation),
                sprite: Sprite::new(Vec2::new(30.0, 30.0)),
                ..Default::default()
            });
            entity.insert(Goon::new(id)).insert(GBodyRemote::default());
            if client_id.is_equal(id) {
                entity
                    .insert(Player::default())
                    .insert(Controller::default())
                    .insert(GBodyLocal::default());
            }
        }
    }
}

/// Updates local gbody positions based on controller input. This is purely speculative, and will have no effect on the server predictions.
fn update_player_local(
    time: Res<Time>,
    mut query: Query<(&Controller, &mut GBodyLocal), With<Player>>,
) {
    for (controller, mut gbody_local) in query.iter_mut() {
        if controller.forward != 0.0 || controller.lateral != 0.0 {
            let step_move = Vec3::new(controller.lateral, controller.forward, 0.0).normalize()
                * time.delta_seconds()
                * PLAYER_SPEED;
            gbody_local.translation += step_move;
        }
    }
}

fn update_player_transforms(
    game_tick: Res<GameTick>,
    mut query: Query<(&mut Transform, &GBodyRemote, Option<&mut GBodyLocal>)>,
) {
    for (mut transform, gbody_remote, gbody_local) in query.iter_mut() {
        let mut target_translation = gbody_remote.translation;
        if let Some(mut gbody_local) = gbody_local {
            if game_tick.frame() > gbody_remote.tick {
                // Remote gbody is out of date, interpolate towards local gbody.
                target_translation = gbody_local.translation;
            } else {
                // We have both a local and a remote body. Lets average them!
                let dist = gbody_local.translation.distance_squared(gbody_remote.translation);
                if dist > 1.0 {
                    // Smoothly interpolate towards the correct position.
                    target_translation = Vec3::lerp(gbody_local.translation, gbody_remote.translation, 0.3);
                    log::info!("Interpolating ({})", dist);
                } else {
                    target_translation = gbody_remote.translation;
                }
            }
            gbody_local.translation = target_translation;
        }
        transform.translation = target_translation;
    }
}
// ===============================================================
// ======================= COMPONENTS ============================
// ===============================================================

/// A simple tag to help identify our player quickly.
#[derive(Debug, Default)]
pub struct Player {}

/// A local representation of a goon's body and where we think it is. No one else cares about this.
#[derive(Debug, Default)]
pub struct GBodyLocal {
    pub translation: Vec3,
}

/// This is the authoritative representation of the goon's body, and represents exactly where the server thinks it is.
#[derive(Debug, Default)]
pub struct GBodyRemote {
    pub tick: u64,
    pub translation: Vec3,
}

// ===============================================================
// ======================== RESOURCES ============================
// ===============================================================
