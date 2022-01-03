use bevy::{math::{Vec2, Quat}, reflect::Reflect};
use serde::{Serialize, Deserialize};
use bevy_inspector_egui::Inspectable;

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq, Inspectable, Reflect)]
pub struct Controller {
    pub translation: Option<Vec2>,
    pub target_look: Option<Vec2>,
    pub target_direction: Option<Quat>,
}