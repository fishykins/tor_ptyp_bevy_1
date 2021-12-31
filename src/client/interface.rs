use bevy::{prelude::*, render::camera::WindowOrigin};

// ===============================================================
// ====================== CLIENT NETWORKING ======================
// ===============================================================

#[derive(Default)]
pub(crate) struct ClientInterfacePlugin;

impl Plugin for ClientInterfacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system_to_stage(CoreStage::PreUpdate, mouse_update.system())
            .insert_resource(MousePosition::default());
    }
}

// ===============================================================
// ========================= SYSTEMS =============================
// ===============================================================

pub struct MainCamera;

#[derive(Default)]
pub struct MousePosition(pub Vec2);

fn startup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera).insert(MainCamera);
}

fn mouse_update(
    // need to get window dimensions
    wnds: Res<Windows>,
    mut mouse_position: ResMut<MousePosition>,
    // query to get camera transform
    q_camera: Query<&Transform, With<MainCamera>>,
) {
    // get the primary window
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is in the primary window
    if let Some(pos) = wnd.cursor_position() {

        let camera_transform = q_camera.single().unwrap();
        // apply the camera transform
        let pos_world = camera_transform.compute_matrix() * pos.extend(0.0).extend(1.0);
        mouse_position.0 = Vec2::new(pos_world.x, pos_world.y);
    }
}
