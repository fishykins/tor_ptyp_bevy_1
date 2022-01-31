use std::{cmp::{Ordering, PartialOrd}, fmt::{Formatter, Display}};

use bevy::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Component)]
/// A component that indicates the parent [Entity] has volume, and by extension mass and density.
pub struct Matter {
    pub density: f32,
    pub volume: f32,
}

impl Matter {
    /// Creates Matter with given density and volume.
    pub fn new(density: f32, volume: f32) -> Self {
        Self {
            density,
            volume,
        }
    }

    /// Creates Matter with a volume of 1.0.
    pub fn new_with_mass(mass: f32) -> Self {
        Self {
            density: mass,
            volume: 1.0,
        }
    }

    /// Creates Matter with evenly distributed volume nad density.
    pub fn new_with_distributed_mass(mass: f32) -> Self {
        Self {
            density: mass.sqrt(),
            volume: mass.sqrt(),
        }
    }

    /// Calculates the mass of the given volume.
    pub fn mass(&self) -> f32 {
        return self.density * self.volume;
    }
}

impl PartialOrd for Matter {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.mass().partial_cmp(&other.mass())
    }
}

impl Display for Matter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Matter(density: {}, volume: {}, mass: {})", self.density, self.volume, self.mass())
    }
}
