use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use super::ColliderShape;


#[derive(Clone, Inspectable, Reflect, Component)]
pub struct Collider {
    pub shape: ColliderShape,
    pub offset: Vec2,
    pub debug: bool,
}

impl Default for Collider {
    fn default() -> Self {
        Self {
            shape: ColliderShape::Circle(0.5),
            offset: Vec2::ZERO,
            debug: false,
        }
    }
}