use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

#[derive(Default, Clone, Inspectable, Reflect)]
pub struct Body<T> where T: 'static + Sync + Send + Default + Inspectable + Reflect {
    pub translation: Vec2,
    pub rotation: Quat,
    pub scale: Vec2,
    last_update: u64,
    #[inspectable(ignore)]
    #[reflect(ignore)]
    phantom: std::marker::PhantomData<T>,
}

impl<T> Body<T> where T: 'static + Sync + Send + Default + Inspectable + Reflect {
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
