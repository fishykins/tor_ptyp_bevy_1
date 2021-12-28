use serde::{Serialize, Deserialize};

use super::input::Movement;

// ===============================================================
// ========================= HELPERS =============================
// ===============================================================

/// Something that is held client side and is sent from the server. 
/// This helps keep track of where we are at when it comes to acquiring the wrapped item.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ServerData<T> where T: Serialize + Clone + PartialEq + Default {
    Acquired(T),
    Waiting(u32),
    None,
}

impl<T> Default for ServerData<T> where T: Serialize + Clone + PartialEq + Default {
    fn default() -> Self {
        ServerData::None
    }
}

// ===============================================================
// ========================== COMPONENTS =========================
// ===============================================================

/// Any entity that has movement logic, be it AI or player, should have this component.
#[derive(Default, Clone, Debug)]
pub struct Controller{
    pub movement: Movement,
}