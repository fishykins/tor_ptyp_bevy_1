use bevy::{core::Time, prelude::*};
use bevy_inspector_egui::Inspectable;
use crate::{physics::Body, control::Controller, network};

const BASE_SPEED: f32 = 200.0;
const RUN_MULTIPLIER: f32 = 1.5;
const ACCELERATION: f32 = 0.1;
const STOPPING_DISTANCE: f32 = 200.0;
const TURN_SPEED: f32 = 4.0;
const TURN_LIMMIT: f32 = 0.75;


#[derive(Debug, Clone, Copy, Inspectable, Reflect)]
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
            speed: BASE_SPEED,
            run_multiplier: RUN_MULTIPLIER,
            acceleration: ACCELERATION,
            stopping_distance: STOPPING_DISTANCE,
            turn_speed: TURN_SPEED,
            turn_limmit: TURN_LIMMIT,
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