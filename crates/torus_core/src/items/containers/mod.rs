use crate::items::{DefaultFloat, Item};

mod pocket;

pub use pocket::Pocket;

/// Base container trait, used to define somthing that can hold entities.
pub trait Holder {
    fn contents(&self) -> Vec<&Item>;
    fn item(&self, item: usize) -> Option<&Item>;
    fn item_mut(&mut self, index: usize) -> Option<&mut Item>;
}

/// Something that can hold items and be interacted with by the player.
pub trait DynamicHolder: Holder {
    fn try_add(&mut self, item: Item) -> Result<usize, Item>;
    fn try_remove(&mut self, item: usize) -> Result<Item, &'static str>;
    /// Maximum capacity of the holder
    fn max_capacity(&self) -> DefaultFloat;
    /// Space left in the holder
    fn capacity(&self) -> DefaultFloat;
    /// Current fill of the holder
    fn fill(&self) -> DefaultFloat;
    /// total weight of the contents of the holder (not including the holder itself)
    fn weight(&self) -> DefaultFloat;
}

#[derive(Debug)]
pub enum Container {
    Pocket(Pocket),
}
