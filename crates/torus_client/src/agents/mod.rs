mod agent_plugin;
mod events;
mod spawn;
mod movement;

//pub use spawn::*;
pub use agent_plugin::AgentPlugin;
pub use events::AgentEvent;
pub use movement::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Player;
