mod agent_plugin;
mod events;
//mod spawn;


//pub use spawn::*;
pub use events::{AgentEvent, handle_events};
pub use agent_plugin::AgentPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Player;