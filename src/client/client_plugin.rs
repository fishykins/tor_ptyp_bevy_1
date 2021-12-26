
use bevy::prelude::*;
use super::systems::*;

#[derive(Default)]
pub(crate) struct ClientPlugin {}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system(handle_messages.system());
    }
}


