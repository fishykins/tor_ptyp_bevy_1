use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;

use crate::core::resources::Session;

#[derive(Default)]
pub(crate) struct ClientPlugin {}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
    }
}

pub fn startup(mut net: ResMut<NetworkResource>, session: ResMut<Session>) {
    info!("Starting client...");
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, session.port);
    net.connect(socket_address);
    info!("Connecting to {}...", socket_address);
}
