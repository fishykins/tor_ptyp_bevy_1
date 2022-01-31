mod rigidbody;
mod update;
mod transform;
mod collider;
mod collider_shape;
mod matter;

use std::f32::consts::PI;

pub use collider_shape::ColliderShape;
pub use transform::*;
pub use rigidbody::*;
pub use update::*;
pub use collider::*;
pub use matter::*;

pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let mut angle = b - a;
    if angle > PI {
        angle -= 2.0 * PI;
    } else if angle < -PI {
        angle += 2.0 * PI;
    }
    normalize_angle(a + angle * t)
}

/// Calculates the smallest difference between two angles.
/// Assumes that values are normalized.
pub fn angle_of_difference(a: f32, b: f32) -> f32 {
    let mut angle = b - a;
    if angle > PI {
        angle -= PI;
    } else if angle < 0.0 {
        angle += PI;
    }
    angle
}

/// Normalizes the angle to the range of 0 -> 2 * PI.
pub fn normalize_angle(angle: f32) -> f32 {
    if angle > 2.0 * std::f32::consts::PI {
        return angle - std::f32::consts::PI * 2.0;
    } else if angle < 0.0 {
        return angle + std::f32::consts::PI * 2.0;
    }
    return angle;
}
