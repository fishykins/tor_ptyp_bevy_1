mod layer;

pub use layer::Layer;

use crate::items::{atire::AtireSlots::*, Item, Wrappable};

/// All the different implimentors of ['atire']
#[derive(Debug)]
pub enum Atire {
    Layer(Layer),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AtireSlots {
    Feet,
    Legs,
    Waiste,
    Baselayer,
    Midlayer,
    Jacket,
    Head,
    Hands,
}

impl AtireSlots {
    pub fn all() -> Vec<AtireSlots> {
        vec![Feet, Legs, Waiste, Baselayer, Midlayer, Jacket, Head, Hands]
    }
}

impl Atire {
    pub fn slots(&self) -> Vec<AtireSlots> {
        match self {
            Atire::Layer(layer) => layer.slots(),
        }
    }
}

impl Wrappable for Atire {
    fn wrap(self) -> Item {
        Item::Atire(self)
    }

    fn try_unwrap(item: &Item) -> Option<&Self> {
        match item {
            Item::Atire(slef) => Some(slef),
            _ => None,
        }
    }

    fn try_unwrap_mut(item: &mut Item) -> Option<&mut Self> {
        match item {
            Item::Atire(slef) => Some(slef),
            _ => None,
        }
    }
}
