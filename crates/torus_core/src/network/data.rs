use std::fmt::Display;
use serde::{Deserialize, Serialize};

/// Something that is held client side and is sent from the server.
/// This helps keep track of where we are at when it comes to acquiring the wrapped item.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ServerData<T>
where
    T: Serialize + Clone + PartialEq + Default,
{
    Acquired(T),
    Waiting(u32),
    None,
}

impl<T> Default for ServerData<T>
where
    T: Serialize + Clone + PartialEq + Default,
{
    fn default() -> Self {
        ServerData::None
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ClientId(pub ServerData<u32>);

impl ClientId {
    pub fn new(id: u32) -> Self {
        ClientId(ServerData::Acquired(id))
    }
    pub fn is_equal(&self, i: u32) -> bool {
        self.0 == ServerData::Acquired(i)
    }
    pub fn allocated(&self) -> bool {
        match self.0 {
            ServerData::Acquired(_) => true,
            _ => false,
        }
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            ServerData::Acquired(id) => write!(f, "ClientId({})", id),
            ServerData::Waiting(ticks) => write!(f, "Waiting({})", ticks),
            ServerData::None => write!(f, "None"),
        }
    }
}

