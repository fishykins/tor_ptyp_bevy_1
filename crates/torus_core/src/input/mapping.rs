use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;

use bevy::prelude::*;
use num::{Signed, Zero};

use super::{Binding, Control};

pub struct InputMap<T, W> {
    controls: HashMap<T, Control<W>>,
    active: HashMap<T, bool>,
    pressed: HashMap<T, bool>,
    released: HashMap<T, bool>,
    value: HashMap<T, W>,
}

impl<T, W> Default for InputMap<T, W> {
    fn default() -> Self {
        Self {
            controls: HashMap::new(),
            active: HashMap::new(),
            pressed: HashMap::new(),
            released: HashMap::new(),
            value: HashMap::new(),
        }
    }
}

impl<T, W> InputMap<T, W>
where
    T: Hash + Eq + Clone + Send + Sync,
    W: Default + Clone + Copy + Send + Sync + AddAssign + PartialOrd + Zero + Signed,
{
    pub fn add_control(&mut self, key: T) -> &mut Self {
        self.controls.insert(key, Default::default());
        self
    }

    pub fn bind<K: Into<T>, B: Into<Binding<W>>>(&mut self, control: K, binding: B) -> &mut Self {
        let key = control.into();
        if !self.controls.contains_key(&key) {
            self.add_control(key.clone());
        }
        if let Some(control) = self.controls.get_mut(&key) {
            control.bindings.push(binding.into());
        }
        self
    }

    pub fn update(input: Res<Input<KeyCode>>, mut map: ResMut<InputMap<T, W>>)
    where
        T: 'static + Debug,
        W: 'static + Debug,
    {
        let active = map
            .controls
            .iter()
            .map(|(key, control)| (key.clone(), control.active(&input)))
            .collect::<HashMap<T, bool>>();

        map.pressed = active
            .iter()
            .map(|(key, active)| {
                let pressed = *active && !map.active(key);
                (key.clone(), pressed)
            })
            .collect::<HashMap<T, bool>>();

        map.released = active
            .iter()
            .map(|(key, active)| {
                let released = !*active && map.active(key);
                (key.clone(), released)
            })
            .collect::<HashMap<T, bool>>();

        map.active = active;

        map.value = map
            .controls
            .iter()
            .map(|(key, control)| (key.clone(), control.axis(&input)))
            .collect::<HashMap<T, W>>();
    }

    pub fn active(&self, key: &T) -> bool {
        *self.active.get(key).unwrap_or(&false)
    }

    pub fn pressed(&self, key: &T) -> bool {
        *self.pressed.get(key).unwrap_or(&false)
    }

    pub fn released(&self, key: &T) -> bool {
        *self.released.get(key).unwrap_or(&false)
    }

    pub fn active_value(&self, key: &T) -> Option<W> {
        if !self.active(key) {
            return None;
        }
        if let Some(v) = self.value.get(key) {
            return Some(*v);
        }
        return None;
    }
}
