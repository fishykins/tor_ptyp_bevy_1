use bevy::prelude::*;

use super::network::{Local, Remote};
use super::{AppState, GameTick, WORLD_SIZE_X, WORLD_SIZE_Y};

#[derive(Default, Debug, Clone)]
pub struct PhysicsPlugin;
// ===============================================================
// ====================== PHYSICS PLUGIN =========================
// ===============================================================

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::on_update(AppState::InGame)
                .with_system(wrap_around.system())
                .with_system(apply_transforms.system())
                .with_system(update_velocity.system()),
        );
    }
}

// ===============================================================
// ======================== SYSTEMS ==============================
// ===============================================================
fn apply_transforms(
    game_tick: Res<GameTick>,
    mut query: Query<(
        &mut Transform,
        Option<&mut Body<Local>>,
        Option<&Body<Remote>>,
    )>,
) {
    for (mut transform, local_body, remote_body) in query.iter_mut() {
        if local_body.is_none() && remote_body.is_none() {
            // This is a static object.
            continue;
        }

        if local_body.is_some() && remote_body.is_none() {
            // Only has a local body so we can do what we like!
            let local_body = local_body.unwrap();
            transform.translation =
                Vec3::new(local_body.translation.x, local_body.translation.y, 1.0);
            transform.rotation = local_body.rotation;
            transform.scale = Vec3::new(local_body.scale.x, local_body.scale.y, 1.0);
            continue;
        }

        if remote_body.is_some() && local_body.is_none() {
            // Only has a remote body so update the transform.
            let remote_body = remote_body.unwrap();
            transform.translation =
                Vec3::new(remote_body.translation.x, remote_body.translation.y, 1.0);
            transform.rotation = remote_body.rotation;
            transform.scale = Vec3::new(remote_body.scale.x, remote_body.scale.y, 1.0);
            continue;
        }

        // We have both a local and remote body so we need to do some work.
        let mut local_body = local_body.unwrap();
        let remote_body = remote_body.unwrap();

        let target_translation: Vec2;

        if game_tick.frame() > remote_body.last_update {
            // Remote Body is out of date, interpolate towards local Body...
            target_translation = local_body.translation;
        } else {
            // All is well, check for local discrepancies.
            let dist = local_body
                .translation
                .distance_squared(remote_body.translation);
            if dist > 1.0 {
                // Smoothly interpolate towards the correct position.
                target_translation =
                    Vec2::lerp(local_body.translation, remote_body.translation, 0.3);
                bevy::log::debug!("Interpolating ({})", dist);
            } else {
                target_translation = remote_body.translation;
            }
        }
        local_body.translation = target_translation;
        transform.rotation = local_body.rotation;
        transform.translation = Vec3::new(target_translation.x, target_translation.y, 1.0);
    }
}

fn update_velocity(game_tick: Res<GameTick>, time: Res<Time>, mut query: Query<&mut Body<Local>>) {
    for mut body in query.iter_mut() {
        let delta_pos = body.translation - body.last_translation;
        let delta_vel = delta_pos / time.delta_seconds();
        body.velocity = delta_vel;
        body.last_translation = body.translation;
        body.last_update = game_tick.frame();
    }
}

fn wrap_around(mut query: Query<&mut Body<Local>>) {
    for mut local in query.iter_mut() {
        if local.translation.x > WORLD_SIZE_X {
            local.translation.x = 0.1;
        } else if local.translation.x < 0.0 {
            local.translation.x = WORLD_SIZE_X;
        }

        if local.translation.y > WORLD_SIZE_Y {
            local.translation.y = 0.1;
        } else if local.translation.y < 0.0 {
            local.translation.y = WORLD_SIZE_Y;
        }
    }
}

// ===============================================================
// ========================== COMPONENTS =========================
// ===============================================================

#[derive(Debug, Default, Clone)]
pub struct Body<T> {
    pub translation: Vec2,
    pub rotation: Quat,
    pub scale: Vec2,

    last_update: u64,
    last_translation: Vec2,
    velocity: Vec2,

    phantom: std::marker::PhantomData<T>,
}

#[allow(dead_code)]
impl<T> Body<T>
where
    T: 'static + Default + Clone,
{
    pub fn velocity(&self) -> Vec2 {
        self.velocity
    }

    pub fn set_last_update(&mut self, frame: u64) {
        self.last_update = frame;
    }

    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }
}
