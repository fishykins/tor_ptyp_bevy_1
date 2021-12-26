use super::systems::*;
use bevy::prelude::*;
use bevy_advanced_input::plugin::InputBindingPlugin;

#[derive(Default)]
pub(crate) struct InputPlugin {}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InputBindingPlugin::<InputType, Bindings>::default())
            .add_startup_system(input_startup.system())
            .add_startup_system(spawn_input_controller.system())
            .add_system(handle_input.system().after("raw_input").label("input"));
    }
}
