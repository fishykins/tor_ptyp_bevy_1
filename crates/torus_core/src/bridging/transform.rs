use crate::{
    flow::GameTick,
    network::{Local, Remote},
    physics::Rigidbody,
};
use bevy::prelude::*;

const INTERPOLATION_THRESHOLD: f32 = 2.0;
const INTERPOLATION_SPEED: f32 = 0.1;

/// A system that will update a transform, based on data in both remote and local bodies.
pub fn apply_transforms_system(
    game_tick: Res<GameTick>,
    mut query: Query<(
        &mut Transform,
        Option<&mut Rigidbody<Local>>,
        Option<&Rigidbody<Remote>>,
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
            transform.translation = Vec3::new(local_body.position.x, local_body.position.y, 1.0);
            transform.rotation = Quat::from_axis_angle(Vec3::Z, local_body.rotation);
            continue;
        }

        if remote_body.is_some() && local_body.is_none() {
            // Only has a remote body so update the transform.
            let remote_body = remote_body.unwrap();
            transform.translation = Vec3::new(remote_body.position.x, remote_body.position.y, 1.0);
            transform.rotation = Quat::from_axis_angle(Vec3::Z, remote_body.rotation);
            continue;
        }

        // We have both a local and remote body so we need to do some work.
        let mut local_body = local_body.unwrap();
        let remote_body = remote_body.unwrap();

        let target_translation: Vec2;

        if game_tick.frame() > remote_body.last_updated() + 1 {
            // Remote Body is out of date, interpolate towards local Body...
            target_translation = local_body.position;
            bevy::log::warn!("Remote body is out of date, interpolating towards local body.");
        } else {
            // All is well, check for local discrepancies.
            let dist = local_body.position.distance_squared(remote_body.position);
            if dist > INTERPOLATION_THRESHOLD {
                // Smoothly interpolate towards the correct position.
                target_translation = Vec2::lerp(local_body.position, remote_body.position, INTERPOLATION_SPEED);
                bevy::log::debug!("Interpolating ({})", dist);
            } else {
                target_translation = remote_body.position;
            }
        }
        local_body.position = target_translation;
        transform.rotation = Quat::from_axis_angle(Vec3::Z, local_body.rotation);
        transform.translation = Vec3::new(target_translation.x, target_translation.y, 1.0);
    }
}
