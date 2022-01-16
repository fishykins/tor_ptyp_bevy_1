use bevy::prelude::*;

use crate::agents::Player;

pub fn camera_update(
    mut query: QuerySet<(
        QueryState<&Transform, With<Player>>,
        QueryState<&mut Transform, With<Camera>>,
    )>
) {
    let player = query.q0().get_single();
    if !player.is_ok() {
        return;
    }
    let window_offset = Vec3::new(400.0, 300.0, 0.0);
    let pos = player.unwrap().translation.clone() - window_offset;

    let mut cam_q = query.q1();
    let mut cam = cam_q.single_mut();

    cam.translation = cam.translation.lerp(pos, 0.03);
}
