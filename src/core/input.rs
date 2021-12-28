use bevy::math::Vec2;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ControlScheme {
    Player,
    UI,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug)]
pub enum ControlInput {
    Move(Movement),
    Action(ActionInput)
}

impl Default for ControlInput {
    fn default() -> Self {
        ControlInput::Move(Movement::default())
    }
}

#[derive(PartialEq, Eq, Clone, Serialize, Deserialize, Debug)]
pub enum ActionInput {
    None,
    Attack,
    Interact,
}

impl Default for ActionInput {
    fn default() -> Self {
        ActionInput::None
    }
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Default)]
pub struct Movement {
    pub transverse: Vec2,
    pub target: Vec2,
    pub mode: u8,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Debug, Default)]
pub struct ControlWrapper(pub ControlInput);