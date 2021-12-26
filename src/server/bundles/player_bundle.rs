use bevy::{prelude::{Bundle, Transform}, math::{Vec2, Vec3}};
use crate::core::components::{Controller, Agent};


#[derive(Bundle, Clone)]
pub struct PlayerBundle {
    pub agent: Agent,
    pub controller: Controller,
    pub transform: Transform,
}

impl PlayerBundle {
    pub fn new(handle: u32, pos: Vec2) -> Self {
        Self {
            agent: Agent { owner: handle },
            controller: Controller::default(),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 1.0)),
        }
    }
}