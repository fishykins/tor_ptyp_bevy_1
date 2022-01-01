use bevy::math::{Vec2, Quat};
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Controller {
    pub translation: Option<Vec2>,
    pub target_look: Option<Vec2>,
    pub target_direction: Option<Quat>,
}