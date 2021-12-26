use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum InputType {
    Editor,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Bindings {
    Hotkeys(Hotkeys),
    Movement(Movement),
    Camera(Camera),
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize)] 
pub enum Movement {
    Forward,
    Right,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Camera {
    Yaw,
    Pitch,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum Hotkeys {
    Test,
}

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)] 
pub struct MovementWrapper(pub Movement, pub f32);