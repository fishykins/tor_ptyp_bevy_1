use crate::{flow::GameTick, physics::Body, network::{Remote, Local}};
use bevy::prelude::*;

/// A system that will update a transform, based on data in both remote and local bodies.
pub fn apply_transforms_system(
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

        if game_tick.frame() > remote_body.last_updated() {
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