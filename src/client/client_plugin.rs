use bevy::prelude::*;
use super::systems::*;

#[derive(Default)]
pub(crate) struct ClientPlugin {}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(net_startup.system())
            .add_startup_system(scene_startup.system())
            .insert_resource(ServerIds::default())
            .add_system(handle_packets.system())
            .add_system(handle_messages.system());
    }
}


