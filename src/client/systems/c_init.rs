use std::net::SocketAddr;

use bevy::{prelude::*, render::camera::WindowOrigin};
use bevy_networking_turbulence::NetworkResource;

use crate::core::resources::Session;

pub(crate) fn net_startup(mut net: ResMut<NetworkResource>, session: ResMut<Session>) {
    info!("Starting client...");
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, session.port);
    net.connect(socket_address);
    info!("Connecting to {}...", socket_address);
}

pub(crate) fn scene_startup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera);
}