use bevy::{prelude::*, render::camera::WindowOrigin};

// ===============================================================
// ====================== CLIENT NETWORKING ======================
// ===============================================================

#[derive(Default)]
pub(crate) struct ClientInterfacePlugin;

impl Plugin for ClientInterfacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}

// ===============================================================
// ========================= SYSTEMS =============================
// ===============================================================

fn startup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera);
}
