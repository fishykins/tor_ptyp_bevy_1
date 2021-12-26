use std::net::SocketAddr;

use bevy::prelude::{ResMut, info};
use bevy_networking_turbulence::NetworkResource;

use crate::core::resources::Session;

pub(crate) fn network_init(mut net: ResMut<NetworkResource>, session: ResMut<Session>) {
    info!("Starting server...");
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, session.port);
    net.listen(socket_address, None, None);
    info!("Listening on {}...", socket_address);
}