mod app_state;
mod session;

pub use app_state::AppState;
pub use session::{Session, SessionType};

use bevy::prelude::*;

#[derive(Debug, Default, Clone)]
pub struct GameTick {
    frame: u64,
}

impl GameTick {
    pub fn next(mut t: ResMut<GameTick>) {
        t.frame += 1;
    }

    pub fn frame(&self) -> u64 {
        self.frame
    }
}
