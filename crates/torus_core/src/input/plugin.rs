use crate::flow::AppState;
use bevy::prelude::*;
use num::{Signed, Zero};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::AddAssign;

use super::InputMap;

pub struct InputPlugin<'a, T, W>(
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
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_update(AppState::InGame)
                    .with_system(InputMap::<T, W>::update.system()),
            );
    }
}
