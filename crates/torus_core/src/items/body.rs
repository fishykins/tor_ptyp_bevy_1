use crate::items::atire::{Atire, AtireSlots};

pub struct Body {
    atire: Vec<Atire>,
}

impl Body {
    pub fn new() -> Self {
        Self { atire: Vec::new() }
    }

    pub fn atire(&self) -> &Vec<Atire> {
        &self.atire
    }

    pub fn empty_slots(&self) -> Vec<AtireSlots> {
        let mut empty = AtireSlots::all();
        for i in self.atire.iter() {
            let slots = i.slots();
            empty.retain(|x| !slots.contains(x));
        }
        return empty;
    }

    pub fn try_add(&mut self, atire: Atire) -> Result<usize, Atire> {
        // First, lets get the body parts we want to add
        if atire.slots().iter().all(|x| self.empty_slots().contains(x)) {
            self.atire.push(atire);
            return Ok(self.atire.len() - 1);
        }
        return Err(atire);
    }
}
