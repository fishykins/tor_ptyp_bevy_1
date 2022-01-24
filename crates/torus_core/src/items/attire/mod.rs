mod layer;

pub use layer::Layer;

use crate::items::{attire::AttireSlots::*, Item, Wrappable};

/// All the different implimentors of ['atire']
#[derive(Debug)]
pub enum Attire {
    Layer(Layer),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AttireSlots {
    Feet,
    Legs,
    Waiste,
    Baselayer,
    Midlayer,
    Jacket,
    Head,
    Hands,
}

impl AttireSlots {
    pub fn all() -> Vec<AttireSlots> {
        vec![Feet, Legs, Waiste, Baselayer, Midlayer, Jacket, Head, Hands]
    }
}

impl Attire {
    pub fn slots(&self) -> Vec<AttireSlots> {
        match self {
            Attire::Layer(layer) => layer.slots(),
        }
    }
}

impl Wrappable for Attire {
    fn wrap(self) -> Item {
        Item::Attire(self)
    }

    fn try_unwrap(item: &Item) -> Option<&Self> {
        match item {
            Item::Attire(slef) => Some(slef),
            _ => None,
        }
    }

    fn try_unwrap_mut(item: &mut Item) -> Option<&mut Self> {
        match item {
            Item::Attire(slef) => Some(slef),
            _ => None,
        }
    }
}
