mod agent_plugin;
mod events;
mod spawn;

//pub use spawn::*;
pub use agent_plugin::AgentPlugin;
pub use events::AgentEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Player;
