use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::*;

use crate::core::components::Controller;

fn keyboard_events(
    mut key_evr: EventReader<KeyboardInput>,
    mut controllers: Query<(Entity, &mut Controller)>,
) {
    use bevy::input::ElementState;

    for ev in key_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                //println!("Key press: {:?} ({})", ev.key_code, ev.scan_code);
                match ev.key_code {
                    Some(KeyCode::W) => {
                        for (_, mut controller) in controllers.iter_mut() {
                            controller.movement.transverse.y = 1.0;
                        }
                    }
                    Some(KeyCode::S) => {
                        for (_, mut controller) in controllers.iter_mut() {
                            controller.movement.transverse.y = -1.0;
                        }
                    }
                    Some(KeyCode::A) => {
                        for (_, mut controller) in controllers.iter_mut() {
                            controller.movement.transverse.x = -1.0;
                        }
                    }
                    Some(KeyCode::D) => {
                        for (_, mut controller) in controllers.iter_mut() {
                            controller.movement.transverse.x = 1.0;
                        }
                    }
                    _ => {}
                }
            }
            ElementState::Released => {
                //println!("Key release: {:?} ({})", ev.key_code, ev.scan_code);
                match ev.key_code {
                    Some(KeyCode::W) => {
                        for (_, mut controller) in controllers.iter_mut() {
                            controller.movement.transverse.y = 0.0;
                        }
                    }
                    Some(KeyCode::S) => {
                        for (_, mut controller) in controllers.iter_mut() {
                            controller.movement.transverse.y = 0.0;
                        }
                    }
                    Some(KeyCode::A) => {
                        for (_, mut controller) in controllers.iter_mut() {
                            controller.movement.transverse.x = 0.0;
                        }
                    }
                    Some(KeyCode::D) => {
                        for (_, mut controller) in controllers.iter_mut() {
                            controller.movement.transverse.x = 0.0;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn cursor_position(windows: Res<Windows>) {
    // Games typically only have one window (the primary window).
    // For multi-window applications, you need to use a specific window ID here.
    let window = windows.get_primary().unwrap();

    if let Some(_position) = window.cursor_position() {
        // cursor is inside the window, position given
    } else {
        // cursor is not inside the window
    }
}

fn mouse_button_events(mut mousebtn_evr: EventReader<MouseButtonInput>) {
    use bevy::input::ElementState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ElementState::Pressed => {
                println!("Mouse button press: {:?}", ev.button);
            }
            ElementState::Released => {
                println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(keyboard_events.system().before("relay"))
            .add_system(mouse_button_events.system().before("relay"))
            .add_system(cursor_position.system().before("relay"));
    }
}
