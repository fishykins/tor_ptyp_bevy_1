use std::marker::PhantomData;

use bevy::prelude::*;

pub trait ItemType {}

/// An entity tagged with this component can be picked up by an agent, stored in containers and generally abused.
#[derive(Debug, Default, Component)]
pub struct Item<T> where T: ItemType {
    pub name : String,
    _phantom: PhantomData<T>,
}