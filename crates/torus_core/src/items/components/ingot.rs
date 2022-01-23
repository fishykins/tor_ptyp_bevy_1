use crate::items::{DefaultFloat, Item, Material, Physical, Wrappable };

use super::Component;

#[derive(Debug)]
pub struct Ingot {
    name: String,
    description: String,
    material: Material,
    volume: DefaultFloat,
}

impl Ingot {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn material(&self) -> &Material {
        &self.material
    }
}

impl Physical for Ingot {
    fn volume(&self) -> DefaultFloat {
        return self.volume;
    }

    fn mass(&self) -> DefaultFloat {
        return self.material.density * self.volume;
    }
}

impl Wrappable for Ingot {
    fn wrap(self) -> Item {
        Item::Component(Component::Ingot(self))
    }

    fn try_unwrap(item: &Item) -> Option<&Self> {
        match item {
            Item::Component(Component::Ingot(d)) => Some(d),
            _ => None,
        }
    }

    fn try_unwrap_mut(_: &mut Item) -> Option<&mut Self> {
        todo!()
    }
}
