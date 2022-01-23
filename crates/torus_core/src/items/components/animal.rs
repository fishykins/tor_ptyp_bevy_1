use crate::items::{DefaultFloat, Item, Physical, Wrappable};

use super::Component;

#[derive(Debug)]
pub struct Animal {
    sound: String,
    weight: DefaultFloat,
    volume: DefaultFloat,
}

impl Animal {
    pub fn new(sound: String, weight: DefaultFloat, volume: DefaultFloat) -> Self {
        Self {
            sound,
            weight,
            volume,
        }
    }

    pub fn make_sound(&self) {
        println!("{}!", self.sound);
    }
}

impl Physical for Animal {
    fn volume(&self) -> DefaultFloat {
        self.volume
    }

    fn mass(&self) -> DefaultFloat {
        self.weight
    }
}

impl Wrappable for Animal {
    fn wrap(self) -> Item {
        Item::Component(Component::Animal(self))
    }

    fn try_unwrap(item: &Item) -> Option<&Self> {
        match item {
            Item::Component(Component::Animal(d)) => Some(d),
            _ => None,
        }
    }

    fn try_unwrap_mut(_: &mut Item) -> Option<&mut Self> {
        todo!()
    }
}
