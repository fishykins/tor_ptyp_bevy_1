use bevy::{core::Time, prelude::*};
use bevy_inspector_egui::Inspectable;
use crate::{physics::Rigidbody, control::Controller, network};

const BASE_SPEED: f32 = 1000.0;
const TURN_SPEED: f32 = 4.0;
const MASS: f32 = 64.0;


#[derive(Debug, Clone, Copy, Inspectable, Reflect)]
pub struct Biped {
    pub speed: f32,
    pub turn_speed: f32,
    pub mass: f32,
}

impl Default for Biped {
    fn default() -> Self {
        Self {
            speed: BASE_SPEED,
            turn_speed: TURN_SPEED,
            mass: MASS,
        }
    }
}

impl Biped {
    #[allow(dead_code)]
    pub fn movement_system(_time: Res<Time>, mut query: Query<(&mut Rigidbody<network::Local>, &Biped, &Controller)>) {
        for (mut _body, _biped, controller) in query.iter_mut() {
            // Sort out the direction
            if let Some(translation) = controller.translation {
                let direction = translation.x.atan2(translation.y);
                bevy::log::info!("direction: {:?}", direction);
            }
        }
    }
}