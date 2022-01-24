use crate::items::attire::{Attire, AttireSlots};

/// Represents a physical object in the game world that can be dressed.
pub struct Manakin {
    attire: Vec<Attire>,
}

impl Manakin {
    pub fn new() -> Self {
        Self { attire: Vec::new() }
    }

    pub fn attire(&self) -> &Vec<Attire> {
        &self.attire
    }

    pub fn empty_slots(&self) -> Vec<AttireSlots> {
        let mut empty = AttireSlots::all();
        for i in self.attire.iter() {
            let slots = i.slots();
            empty.retain(|x| !slots.contains(x));
        }
        return empty;
    }

    pub fn try_add(&mut self, atire: Attire) -> Result<usize, Attire> {
        // First, lets get the body parts we want to add
        if atire.slots().iter().all(|x| self.empty_slots().contains(x)) {
            self.attire.push(atire);
            return Ok(self.attire.len() - 1);
        }
        return Err(atire);
    }
}
