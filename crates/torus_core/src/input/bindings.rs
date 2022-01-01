use bevy::prelude::*;
use num::One;

use super::DefaultWeight;

#[derive(Clone, Debug, PartialEq)]
pub struct Binding<T = DefaultWeight> {
    keycode: KeyCode,
    weight: T,
}

impl<T> From<KeyCode> for Binding<T>
where
    T: One,
{
    fn from(keycode: KeyCode) -> Self {
        Self {
            keycode,
            weight: T::one(),
        }
    }
}

impl<W> Binding<W>
where
    W: Clone + Copy,
{
    pub fn new(keycode: KeyCode, weight: W) -> Self {
        Self { keycode, weight }
    }

    pub fn active(&self, input: &Res<Input<KeyCode>>) -> bool {
        return input.pressed(self.keycode);
    }

    pub fn active_weight(&self, input: &Res<Input<KeyCode>>) -> Option<W> {
        if self.active(input) {
            return Some(self.weight);
        }
        return None;
    }

    pub fn set_weight(&mut self, weight: W) {
        self.weight = weight;
    }
}