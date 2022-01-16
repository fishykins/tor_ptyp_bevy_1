use bevy::{reflect::Reflect, prelude::Component};
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Clone, Default, Inspectable, Reflect, Component)]
pub struct Agent {
    #[inspectable(label = "Owner", read_only)]
    pub owner: u32,
}

impl Agent {
    pub fn new(owner: u32) -> Self {
        Self { owner }
    }
}
