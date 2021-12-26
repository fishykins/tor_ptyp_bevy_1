use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_networking_turbulence::{NetworkResource, Packet};

use crate::core::resources::Session;

#[derive(Default)]
pub(crate) struct ClientPlugin {}

impl Plugin for ClientPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system());
        app.add_system(send_packets.system());
    }
}

pub fn startup(mut net: ResMut<NetworkResource>, session: ResMut<Session>) {
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, session.port);
    info!("Starting client");
    net.connect(socket_address);
}

fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>) {
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        net.broadcast(Packet::from("PING"));
    }
}
