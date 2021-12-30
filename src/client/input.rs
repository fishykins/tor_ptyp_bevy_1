use std::ops::{Deref, DerefMut};

use bevy::prelude::*;

use crate::core::{
    components::Controller,
    input::{Binding, InputMap},
};

// ===========================================================================
// ================================ INPUT ====================================
// ===========================================================================

#[derive(Default)]
pub(crate) struct ClientInputPlugin;

impl Plugin for ClientInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(crate::core::input::InputPlugin::<ControlScheme, f32>::default());
        app.add_startup_system(startup.system())
            .add_system_to_stage(CoreStage::PreUpdate, update.system());
    }
}

// ===========================================================================
// =============================== SYSTEMS ===================================
// ===========================================================================

fn startup(mut input_map: ResMut<InputMap<ControlScheme, f32>>) {
    input_map
        .deref_mut()
        .add_control(ControlScheme::Walk)
        .add_control(ControlScheme::Strafe)
        .bind(ControlScheme::Walk, Binding::new(KeyCode::W, 1.0))
        .bind(ControlScheme::Walk, Binding::new(KeyCode::S, -1.0))
        .bind(ControlScheme::Strafe, Binding::new(KeyCode::D, 1.0))
        .bind(ControlScheme::Strafe, Binding::new(KeyCode::A, -1.0));
}

fn update(input_map: Res<InputMap<ControlScheme, f32>>, mut query: Query<&mut Controller>) {
    let input = input_map.deref();
    let forward = input.active_value(&ControlScheme::Walk);
    let lateral = input.active_value(&ControlScheme::Strafe);

    for mut controller in query.iter_mut() {
        if forward.is_some() {
            controller.forward = forward.unwrap();
        } else {
            controller.forward = 0.0;
        }
        if lateral.is_some() {
            controller.lateral = lateral.unwrap();
        } else {
            controller.lateral = 0.0;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ControlScheme {
    None,
    Walk,
    Strafe,
}

impl Default for ControlScheme {
    fn default() -> Self {
        ControlScheme::None
    }
}
