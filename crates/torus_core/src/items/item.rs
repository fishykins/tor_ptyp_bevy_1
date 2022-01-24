use crate::items::{DefaultFloat, attire::*, components::*, containers::*};

/// A wrapper enum for all item types.
#[derive(Debug)]
pub enum Item {
    Attire(Attire),
    Component(Component),
    Container(Container),
    None,
}

impl Item {
    pub fn as_container(&self) -> Option<&Container> {
        match self {
            Item::Container(i) => Some(i),
            _ => None
        }
    }

    pub fn weight(&self) -> DefaultFloat {
        match self {
            Item::Attire(_) => todo!(),
            Item::Component(c) => c.weight(),
            Item::Container(_) => todo!(),
            _ => 0.0,
        }
    }

    pub fn volume(&self) -> DefaultFloat {
        match self {
            Item::Attire(_) => todo!(),
            Item::Component(c) => c.volume(),
            Item::Container(_) => todo!(),
            _ => 0.0,
        }
    }
}

/// Something that has spacial values such as weight and volume
pub trait Physical {
    /// Rigid objects have a set size, but some items can be fluid and therefor do not have a size.
    fn volume(&self) -> DefaultFloat;
    fn mass(&self) -> DefaultFloat;
}

/// Functions for converting to and from the Item enum
pub trait Wrappable {
    fn wrap(self) -> Item;
    fn try_unwrap(item: &Item) -> Option<&Self>;
    fn try_unwrap_mut(item: &mut Item) -> Option<&mut Self>;
}

