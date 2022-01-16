use bevy::{math::{Vec2, Quat}, reflect::Reflect, prelude::Component};
use serde::{Serialize, Deserialize};
use bevy_inspector_egui::Inspectable;

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Inspectable, Reflect, Component)]
pub struct Controller {
    pub translation: Option<Vec2>,
    pub target_look: Option<Vec2>,
    pub target_direction: Option<Quat>,
}