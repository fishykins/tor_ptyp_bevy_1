use bevy::prelude::Plugin;

#[derive(Default)]
/// A plugin that contains universal assets for the Torus engine.
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, _app: &mut bevy::prelude::AppBuilder) {

    }
}