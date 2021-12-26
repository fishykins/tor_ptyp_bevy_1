#[allow(dead_code)]
use bevy::prelude::*;

use bevy_advanced_input::{
    config::InputConfig,
    input_id::InputId,
    user_input::{InputAxisType, MouseAxisType, UserInputHandle, UserInputSet},
};
use bevy_networking_turbulence::NetworkResource;
use crate::{core::input::*, network::ClientMessage};

pub(crate) fn handle_input(
    mut net: ResMut<NetworkResource>,
    input_bindings: Res<UserInputHandle<InputType, Bindings>>,
    query: Query<&InputId>,
) {
    query.for_each_mut(|input_component| {
        if let Some(input_handle) = input_bindings.to_handle(input_component) {
            if let Some(value) =
                input_handle.get_axis_value(Bindings::Movement(Movement::Right))
            {
                net.broadcast_message(ClientMessage::Input(MovementWrapper(
                    Movement::Right,
                    value,
                )));
            }

            if let Some(value) =
                input_handle.get_axis_value(Bindings::Movement(Movement::Forward))
            {
                net.broadcast_message(ClientMessage::Input(MovementWrapper(
                    Movement::Forward,
                    value,
                )));
            }

            if let Some(value) = input_handle.get_key_state(Bindings::Hotkeys(Hotkeys::Test)) {
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

    set.begin_key(Bindings::Hotkeys(Hotkeys::Test))
        .add(&[
            InputAxisType::KeyboardButton(KeyCode::Q),
            InputAxisType::KeyboardButton(KeyCode::W),
        ])
        .enable_repeat_all_for_reactivation();

    set.begin_axis(Bindings::Movement(Movement::Forward))
        .add(InputAxisType::KeyboardButton(KeyCode::W))
        .add(InputAxisType::KeyboardButton(KeyCode::S))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::LeftStickY));

    set.begin_axis(Bindings::Movement(Movement::Right))
        .add(InputAxisType::KeyboardButton(KeyCode::A))
        .add(InputAxisType::KeyboardButton(KeyCode::D))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::LeftStickX));


    set.begin_axis(Bindings::Camera(Camera::Yaw))
        .add(InputAxisType::MouseAxisDiff(MouseAxisType::X))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::RightStickX));

    set.begin_axis(Bindings::Camera(Camera::Pitch))
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