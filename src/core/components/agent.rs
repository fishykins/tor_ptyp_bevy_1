#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// A component that represents a player-owned entity. 
pub struct Agent {
    pub owner: u32
}