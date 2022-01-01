pub mod protocols;
pub mod messages;
pub mod data;

mod plugin;
pub use plugin::NetworkPlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct Local;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default)]
pub struct Remote;