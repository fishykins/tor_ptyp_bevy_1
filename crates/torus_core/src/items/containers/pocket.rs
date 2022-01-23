use std::convert::TryFrom;

use crate::items::{DefaultFloat, Item, Wrappable, containers::*};

/// A Pocket can hold items that have the combined weight of less than the max capacity.
#[derive(Debug)]
pub struct Pocket{
    container: Vec<Item>,
    empty_slots: Vec<usize>,
    capacity: DefaultFloat,
}

impl Pocket{
    pub fn new(capacity: DefaultFloat) -> Self {
        Self {
            container: Vec::new(),
            empty_slots: Vec::new(),
            capacity,
        }
    }
}

impl Holder for Pocket{
    fn contents(&self) -> Vec<&Item> {
        self.container.iter().collect()
    }

    fn item(&self, index: usize) -> Option<&Item> {
        self.container.get(index)
    }

    fn item_mut(&mut self, index: usize) -> Option<&mut Item> {
        self.container.get_mut(index)
    }
}

impl DynamicHolder for Pocket{
    fn try_add(&mut self, item: Item) -> Result<usize, Item> {
        if item.volume() <= self.capacity() {
            if let Some(index) = self.empty_slots.pop() {
                self.container[index] = item;
                return Ok(index)
            } else {
                self.container.push(item);
                return Ok(self.container.len() -1) 
            }
        }
        Err(item)
    }

    fn try_remove(&mut self, index: usize) -> Result<Item, &'static str> {
        if index >= self.container.len() {
            return Err("Index out of range");
        }
        let item = self.container.remove(index);
        self.container.insert(index, Item::None);
        Ok(item)
    }

    fn max_capacity(&self) -> DefaultFloat {
        self.capacity
    }

    fn capacity(&self) -> DefaultFloat {
        self.capacity - self.fill()
    }

    fn fill(&self) -> DefaultFloat {
        let mut sum: DefaultFloat = 0.0;
        for i in self.container.iter() {
            sum += i.volume();
        }
        sum
    }

    fn weight(&self) -> DefaultFloat {
        let mut sum: DefaultFloat = 0.0;
        for i in self.container.iter() {
            sum += i.weight();
        }
        sum
    }

    
}

impl Wrappable for Pocket {
    fn wrap(self) -> Item {
        Item::Container(Container::Pocket(self))
    }

    fn try_unwrap(item: &Item) -> Option<&Self> {
        match item {
            Item::Container(c) => {
                match c {
                    Container::Pocket(p) => Some(p),
                } 
            },
            _ => None
        }
    }

    fn try_unwrap_mut(item: &mut Item) -> Option<&mut Self> {
        match item {
            Item::Container(c) => {
                match c {
                    Container::Pocket(p) => Some(p),
                } 
            },
            _ => None
        }
    }
}

impl TryFrom<Item> for Pocket {
    type Error = &'static str;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        match item {
            Item::Container(c) => {
                match c {
                    Container::Pocket(p) => Ok(p),
                } 
            },
            _ => Err("Cannot cast from item to Pocket")
        }
    }
}