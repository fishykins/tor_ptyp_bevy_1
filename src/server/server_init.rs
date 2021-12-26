use std::net::SocketAddr;

use bevy::prelude::{ResMut, info};
use bevy_networking_turbulence::NetworkResource;

use crate::core::resources::Session;



pub(crate) fn startup(mut net: ResMut<NetworkResource>, session: ResMut<Session>) {
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, session.port);
    info!("Starting server");
    net.listen(socket_address, None, None);
}