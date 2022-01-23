use crate::items::{Item, Physical, Wrappable, DefaultFloat, Material, PI};

use super::Component;


#[derive(Debug)]
pub struct Spool{
    name: String,
    description: String,
    material: Material,
    length: DefaultFloat,
    diameter: DefaultFloat,
}

impl Spool {
    pub fn new(name: String, description: String, material: Material, length: DefaultFloat, diameter: DefaultFloat) -> Self {
        Spool {
            name,
            description,
            material,
            length,
            diameter,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn material(&self) -> &Material {
        &self.material
    }

    pub fn length(&self) -> &DefaultFloat {
        &self.length
    }

    pub fn diameter(&self) -> &DefaultFloat {
        &self.diameter
    }

    /// Cuts a length of the spool off and returns it. If not enough length is available, returns the maximum possible cut.
    pub fn cut(&mut self, length: DefaultFloat) -> DefaultFloat {
        let mut cut_length = length;
        if cut_length > self.length {
            cut_length = self.length;
        }
        self.length -= cut_length;
        return cut_length
    }

    /// Tries to cut the desired amount of length from the spool. Returns true if the amount was cut, false if not.
    pub fn try_cut(&mut self, length: DefaultFloat) -> bool {
        if length > self.length {
            return false
        }
        self.length -= length;
        return true
    }
}

impl Physical for Spool {
    fn volume(&self) -> DefaultFloat {
        let r = self.diameter / 2.0;
        return PI * r * r * self.length;
    }

    fn mass(&self) -> DefaultFloat {
        return self.volume() * self.material.density;
    }
}

impl Wrappable for Spool {
    fn wrap(self) -> Item {
        Item::Component(Component::Spool(self))
    }

    fn try_unwrap(item: &Item) -> Option<&Self> {
        match item {
            Item::Component(Component::Spool(d)) => Some(d),
            _ => None,
        }
    }

    fn try_unwrap_mut(_: &mut Item) -> Option<&mut Self> {
        todo!()
    }
}
