use bevy::prelude::*;

#[derive(Default, Clone)]
pub struct Body<T> {
    pub translation: Vec2,
    pub rotation: Quat,
    pub scale: Vec2,
    last_update: u64,
    phantom: std::marker::PhantomData<T>,
}

impl<T> Body<T> where T: 'static + Default {
    pub fn from_translation(translation: Vec2) -> Self {
        Self {
            translation,
            ..Default::default()
        }
    }

    pub fn set_last_update(&mut self, frame: u64) {
        self.last_update = frame;
    }

    pub fn last_updated(&self) -> u64 {
        self.last_update
    }
}
