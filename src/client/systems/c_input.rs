#[allow(dead_code)]
use bevy::prelude::*;

use bevy_advanced_input::{
    config::InputConfig,
    input_id::InputId,
    user_input::{InputAxisType, MouseAxisType, UserInputHandle, UserInputSet},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum InputType {
    Editor,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum Bindings {
    Hotkeys(HotkeysInput),
    Movement(MovementInput),
    Camera(CameraInput),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)] 
pub(crate) enum MovementInput {
    Forward,
    Right,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum CameraInput {
    Yaw,
    Pitch,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub(crate) enum HotkeysInput {
    Test,
}

pub(crate) fn handle_input(
    input_bindings: Res<UserInputHandle<InputType, Bindings>>,
    query: Query<&InputId>,
) {
    query.for_each_mut(|input_component| {
        if let Some(input_handle) = input_bindings.to_handle(input_component) {
            if let Some(value) =
                input_handle.get_axis_value(Bindings::Movement(MovementInput::Right))
            {
                println!("Right: {}", value);
            }

            if let Some(value) =
                input_handle.get_axis_value(Bindings::Movement(MovementInput::Forward))
            {
                println!("Forward: {}", value);
            }

            if let Some(value) = input_handle.get_axis_value(Bindings::Camera(CameraInput::Yaw)) {
                println!("Yaw: {}", value);
            }
            if let Some(value) = input_handle.get_axis_value(Bindings::Camera(CameraInput::Pitch)) {
                println!("Pitch: {}", value);
            }
            if let Some(value) = input_handle.get_key_state(Bindings::Hotkeys(HotkeysInput::Test)) {
                println!("Test: {:?}", value);
            }
        }
    });
}

pub(crate) fn input_startup(mut input_bindings: ResMut<UserInputHandle<InputType, Bindings>>) {
    let mut config: InputConfig<Bindings> = InputConfig::new();
    config.rebind_default_value(InputAxisType::KeyboardButton(KeyCode::S), -1.0);
    config.rebind_default_value(InputAxisType::KeyboardButton(KeyCode::A), -1.0);

    let mut set = UserInputSet::new();

    set.begin_key(Bindings::Hotkeys(HotkeysInput::Test))
        .add(&[
            InputAxisType::KeyboardButton(KeyCode::Q),
            InputAxisType::KeyboardButton(KeyCode::W),
        ])
        .enable_repeat_all_for_reactivation();

    set.begin_axis(Bindings::Movement(MovementInput::Forward))
        .add(InputAxisType::KeyboardButton(KeyCode::W))
        .add(InputAxisType::KeyboardButton(KeyCode::S))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::LeftStickY));

    set.begin_axis(Bindings::Movement(MovementInput::Right))
        .add(InputAxisType::KeyboardButton(KeyCode::A))
        .add(InputAxisType::KeyboardButton(KeyCode::D))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::LeftStickX));


    set.begin_axis(Bindings::Camera(CameraInput::Yaw))
        .add(InputAxisType::MouseAxisDiff(MouseAxisType::X))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::RightStickX));

    set.begin_axis(Bindings::Camera(CameraInput::Pitch))
        .add(InputAxisType::MouseAxisDiff(MouseAxisType::Y))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::RightStickY));

    input_bindings.add_input(InputType::Editor, set);
    input_bindings.apply_config(&config);
}

pub(crate) fn spawn_input_controller(
    mut commands: Commands,
    mut input_bindings: ResMut<UserInputHandle<InputType, Bindings>>,
) {
    commands.spawn().insert(input_bindings.create_input_id(InputType::Editor));
}