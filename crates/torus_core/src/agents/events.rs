use crate::network::messages::AgentData;

#[derive(Clone)]
pub enum AgentEvent {
    Spawn(u32, AgentData),
}
