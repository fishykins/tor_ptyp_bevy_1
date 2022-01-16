mod spawn;

use bevy::prelude::Component;
pub use spawn::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Component)]
pub struct Player;
