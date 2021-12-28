use bevy::prelude::*;

// ===============================================================
// ========================= BRIDGING ============================
// ===============================================================

/// This plugin is responsible for bundling client
/// data and sending it to the server.
#[derive(Default)]
pub(crate) struct ClientBridgePlugin;

impl Plugin for ClientBridgePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}

// ===============================================================
// ========================= SYSTEMS =============================
// ===============================================================

fn startup(mut _commands: Commands) {

}
