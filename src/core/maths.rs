use std::f32::consts::PI;

pub fn lerp_angle(a: f32, b: f32, t: f32) -> f32 {
    let mut angle = b - a;
    if angle > PI {
        angle -= 2.0 * PI;
    } else if angle < -PI {
        angle += 2.0 * PI;
    }
    a + angle * t
}


pub fn angle_of_difference(a: f32, b: f32) -> f32 {
    let mut angle = b - a;
    if angle > PI {
        angle -= 2.0 * PI;
    } else if angle < -PI {
        angle += 2.0 * PI;
    }
    angle
}