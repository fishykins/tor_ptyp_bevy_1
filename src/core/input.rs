use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Serialize, Deserialize, Debug)]
pub enum ControlScheme {
    Player,
    UI,
}