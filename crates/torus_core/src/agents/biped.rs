use bevy::{core::Time, prelude::*};

use crate::{physics::Body, control::Controller, network};


#[derive(Debug, Clone, Copy)]
pub struct Biped {
    pub speed: f32,
    pub run_multiplier: f32,
    pub acceleration: f32,
    pub stopping_distance: f32,
    pub turn_speed: f32,
    pub turn_limmit: f32,
}

impl Default for Biped {
    fn default() -> Self {
        Self {
            speed: 200.0,
            run_multiplier: 1.5,
            acceleration: 0.1,
            stopping_distance: 200.0,
            turn_speed: 4.0,
            turn_limmit: 0.75,
        }
    }
}

impl Biped {
    #[allow(dead_code)]
    pub fn movement_system(_time: Res<Time>, mut query: Query<(&mut Body<network::Local>, &Biped, &Controller)>) {
        for (mut _body, _biped, controller) in query.iter_mut() {
            // Sort out the direction
            if let Some(translation) = controller.translation {
                let direction = translation.x.atan2(translation.y);
                bevy::log::info!("direction: {:?}", direction);
            }
        }
    }
}