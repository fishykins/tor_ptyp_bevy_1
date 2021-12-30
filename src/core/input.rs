use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;

use bevy::prelude::*;
use num::{clamp, One, Signed, Zero};

// ===========================================================================
// ================================ INPUT ====================================
// ===========================================================================

#[derive(Default)]
pub(crate) struct InputPlugin<'a, T, W>(
    std::marker::PhantomData<&'a T>,
    std::marker::PhantomData<&'a W>,
);

impl<'a, T, W> Plugin for InputPlugin<'a, T, W>
where
    InputMap<T, W>: Default,
    T: Hash + Eq + Clone + Send + Sync + Debug,
    W: Clone + Copy + Send + Sync + Debug + Default + AddAssign<W> + Zero + PartialOrd + Signed,
    'a: 'static,
{
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<InputMap<T, W>>()
            .add_system_to_stage(CoreStage::First, InputMap::<T, W>::update.system());
    }
}

// ===========================================================================
// ============================== BINDINGS ===================================
// ===========================================================================
pub type DefaultWeight = f32;

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

// ===========================================================================
// =============================== CONTROL ===================================
// ===========================================================================
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Control<W = DefaultWeight> {
    bindings: Vec<Binding<W>>,
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

// ===========================================================================
// ============================== INPUT MAP ==================================
// ===========================================================================
pub struct InputMap<T, W> {
    controls: HashMap<T, Control<W>>,
    active: HashMap<T, bool>,
    value: HashMap<T, W>,
}

impl<T, W> Default for InputMap<T, W> {
    fn default() -> Self {
        Self {
            controls: HashMap::new(),
            active: HashMap::new(),
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

    fn update(input: Res<Input<KeyCode>>, mut map: ResMut<InputMap<T, W>>)
    where
        T: 'static + Debug,
        W: 'static + Debug,
    {
        map.active = map
            .controls
            .iter()
            .map(|(key, control)| (key.clone(), control.active(&input)))
            .collect::<HashMap<T, bool>>();

        map.value = map
            .controls
            .iter()
            .map(|(key, control)| (key.clone(), control.axis(&input)))
            .collect::<HashMap<T, W>>();
    }

    pub fn active(&self, key: &T) -> bool {
        *self.active.get(key).unwrap_or(&false)
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
