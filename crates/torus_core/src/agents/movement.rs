use crate::agents::Biped;
use crate::control::Controller;
use crate::network::Local;
use crate::physics::{Rigidbody, lerp_angle};
use bevy::prelude::*;

pub fn move_agents(
    time: Res<Time>,
    mut agents: Query<(&mut Rigidbody<Local>, &Controller, &Biped)>,
) {
    for (mut body, controller, biped) in agents.iter_mut() {
        if let Some(wasd) = controller.translation {
            let direction = -wasd.x.atan2(wasd.y);
            body.rotation = lerp_angle(
                body.rotation,
                direction,
                biped.turn_speed * time.delta_seconds(),
            );

            let v = wasd.normalize().length();
            if v != 0.0 {
                body.add_forward_force(v * biped.speed);
            }
        }

        if body.position.is_nan() {
            body.position = Vec2::new(10.0, 10.0);
            body.linear_velocity = Vec2::new(0.0, 0.0);
            bevy::log::warn!("NaN position");
        }
    }
}
