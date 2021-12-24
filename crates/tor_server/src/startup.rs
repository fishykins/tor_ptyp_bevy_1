use bevy::log::info;
use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;
use std::net::SocketAddr;

const SERVER_PORT: u16 = 14191;




pub fn startup(mut net: ResMut<NetworkResource>) {
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);
    info!("Starting server");
    net.listen(socket_address, None, None);
}
