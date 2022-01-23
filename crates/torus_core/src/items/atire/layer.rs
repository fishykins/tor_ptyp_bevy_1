use std::{convert::TryFrom};

use crate::items::{atire::*, containers::*, Item, Wrappable};

#[derive(Debug)]
pub struct Layer {
    name: String,
    slots: Vec<AtireSlots>,
    pockets: Vec<Pocket>,
}

impl Layer {
    pub fn new(name: String, slots: Vec<AtireSlots>, pockets: Vec<Pocket>) -> Self {
        Self {
            name,
            slots,
            pockets,
        }
    }
    pub fn slots(&self) -> Vec<AtireSlots> {
        self.slots.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn pocket(&self, i: usize) -> Option<&Pocket> {
        if i <= self.pockets.len() {
            return None;
        }
        Some(&self.pockets[i])
    }
    pub fn pocket_mut(&mut self, i: usize) -> Option<&mut Pocket> {
        if i >= self.pockets.len() {
            return None;
        }
        Some(&mut self.pockets[i])
    }
}

impl Wrappable for Layer {
    fn wrap(self) -> Item {
        Item::Atire(Atire::Layer(self))
    }

    fn try_unwrap(_: &Item) -> Option<&Self> {
        todo!()
    }

    fn try_unwrap_mut(_: &mut Item) -> Option<&mut Self> {
        todo!()
    }
}

impl TryFrom<Item> for Layer {
    type Error = &'static str;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        match item {
            Item::Atire(a) => match a {
                Atire::Layer(l) => Ok(l),
            },
            _ => Err("Failed to get Layer from Item"),
        }
    }
}
