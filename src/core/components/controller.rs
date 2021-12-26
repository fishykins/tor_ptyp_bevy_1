use crate::core::player;

#[allow(dead_code)]
#[derive(Debug, Clone, Default)]
/// A component that holds entity movement/input data.
pub struct Controller {
    pub movement: player::Movement,
}