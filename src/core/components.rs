use bevy::math::Vec2;
use serde::{Deserialize, Serialize};

// ===============================================================
// ========================= HELPERS =============================
// ===============================================================

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

// ===============================================================
// ========================== COMPONENTS =========================
// ===============================================================
/// Any entity that has movement logic, be it AI or player, should have this component.
#[derive(Default, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Controller {
    pub forward: f32,
    pub lateral: f32,
    pub target: Vec2,
}

/// A component that implies ownership of an entity.
#[derive(Debug, Clone)]
pub struct Goon {
    owner: u32,
}

impl Goon {
    pub fn new(owner: u32) -> Self {
        Goon { owner }
    }
    pub fn owner(&self) -> u32 {
        self.owner
    }
}
