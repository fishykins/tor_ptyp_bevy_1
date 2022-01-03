use torus_core::network::messages::AgentData;

#[derive(Clone)]
pub enum AgentEvent {
    Spawn(u32, AgentData),
}
