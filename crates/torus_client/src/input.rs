use std::ops::{Deref, DerefMut};
use bevy_inspector_egui::{WorldInspectorParams};
use torus_core::{
    agents::Controller,
    flow::AppState,
    input::{Binding, InputMap, InputPlugin as CoreInputPlugin},
};

use bevy::prelude::*;

use crate::MainCamera;
// ===========================================================================
// ================================ PLUGIN ===================================
// ===========================================================================

#[derive(Default)]
pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CoreInputPlugin::<ControlScheme, f32>::default())
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(update_controller.system())
                    .with_system(toggle_inspector.system())
                    .label("input")
                    .before("physics"),
            )
            .add_startup_system(startup.system());
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum ControlScheme {
    None,
    Walk,
    Strafe,
    Console,
}

impl Default for ControlScheme {
    fn default() -> Self {
        ControlScheme::None
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
        .add_control(ControlScheme::Console)
        .bind(ControlScheme::Walk, Binding::new(KeyCode::W, 1.0))
        .bind(ControlScheme::Walk, Binding::new(KeyCode::S, -1.0))
        .bind(ControlScheme::Strafe, Binding::new(KeyCode::D, 1.0))
        .bind(ControlScheme::Strafe, Binding::new(KeyCode::A, -1.0))
        .bind(ControlScheme::Console, Binding::new(KeyCode::F1, 1.0));
}

fn update_controller(
    input_map: Res<InputMap<ControlScheme, f32>>,
    windows: Res<Windows>,
    cameras: Query<&Transform, With<MainCamera>>,
    mut query: Query<&mut Controller>,
) {
    if query.iter_mut().next().is_none() {
        return;
    }
    let mut controller = query.iter_mut().next().unwrap();
    let camera = cameras.single();
    let window = windows.get_primary().unwrap();

    let input = input_map.deref();
    let forward = input.active_value(&ControlScheme::Walk);
    let lateral = input.active_value(&ControlScheme::Strafe);

    if let Some(pos) = window.cursor_position() {
        // apply the camera transform
        let pos_world = camera.compute_matrix() * pos.extend(0.0).extend(1.0);
        controller.target_look = Some(Vec2::new(pos_world.x, pos_world.y));
    } else {
        controller.target_look = None;
    }

    if forward.is_some() || lateral.is_some() {
        controller.translation = Some(Vec2::new(lateral.unwrap_or(0.0), forward.unwrap_or(0.0)));
    } else {
        controller.translation = None;
    }
}

fn toggle_inspector(
    input_map: Res<InputMap<ControlScheme, f32>>,
    mut inspector_windows: ResMut<WorldInspectorParams>,
) {
    if input_map.pressed(&ControlScheme::Console) {
        inspector_windows.enabled = !inspector_windows.enabled;
    }
}