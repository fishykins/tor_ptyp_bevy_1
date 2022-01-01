use std::ops::AddAssign;

use bevy::prelude::*;
use num::{Zero, Signed, clamp};

use super::{Binding, DefaultWeight};

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Control<W = DefaultWeight> {
    pub bindings: Vec<Binding<W>>,
}

impl<W> Control<W>
where
    W: Copy + AddAssign + Zero + Signed + PartialOrd,
{
    pub fn new(bindings: Vec<Binding<W>>) -> Self {
        Self { bindings }
    }

    pub fn active(&self, input: &Res<Input<KeyCode>>) -> bool {
        for binding in &self.bindings {
            if binding.active(input) {
                return true;
            }
        }
        false
    }

    pub fn axis(&self, input: &Res<Input<KeyCode>>) -> W {
        let mut value = W::zero();
        for binding in &self.bindings {
            if let Some(weight) = binding.active_weight(input) {
                value += weight;
            }
        }
        return clamp(value, W::zero() - W::one(), W::one());
    }
}