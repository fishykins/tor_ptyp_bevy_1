mod rigidbody;

pub use rigidbody::*;

use std::f32::consts::PI;

pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let mut angle = b - a;
    if angle > PI {
        angle -= 2.0 * PI;
    } else if angle < -PI {
        angle += 2.0 * PI;
    }
    normalize_angle(a + angle * t)
}


pub fn angle_of_difference(a: f32, b: f32) -> f32 {
    normalize_angle(b - a)
}

pub fn normalize_angle(angle: f32) -> f32 {
    if angle > 2.0 * std::f32::consts::PI {
        return angle - std::f32::consts::PI * 2.0;
    } else if angle < 0.0 {
        return angle + std::f32::consts::PI * 2.0;
    }
    return angle;
}