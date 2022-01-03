pub mod protocols;
pub mod messages;
pub mod data;

mod plugin;
use bevy::reflect::Reflect;
use bevy_inspector_egui::Inspectable;
pub use plugin::NetworkPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Inspectable, Reflect)]
pub struct Local;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, Inspectable, Reflect)]
pub struct Remote;