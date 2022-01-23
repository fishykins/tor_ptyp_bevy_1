mod animal;
mod spool;
mod ingot;

pub use spool::Spool;
pub use animal::Animal;
pub use ingot::Ingot;

use crate::items::{DefaultFloat, Physical};

#[derive(Debug)]
pub enum Component {
    Animal(Animal),
    Spool(Spool),
    Ingot(Ingot),
}

impl Component {
    pub fn volume(&self) -> DefaultFloat {
        match self {
            Component::Animal(a) => a.volume(),
            Component::Spool(s) => s.volume(),
            Component::Ingot(i) => i.volume(),
        }
    }

    pub fn weight(&self) -> DefaultFloat {
        match self {
            Component::Animal(a) => a.mass(),
            Component::Spool(s) => s.mass(),
            Component::Ingot(i) => i.mass(),
        }
    }
}