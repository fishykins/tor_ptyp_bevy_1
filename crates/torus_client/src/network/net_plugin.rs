use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr,
};

use super::broadcasts::{broadcast_client_data, handle_server_broadcasts};
use super::events::handle_events;
use super::messages::handle_messages;
use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;
use torus_core::{
    flow::{AppState, Session},
    network::NetworkPlugin as CoreNetworkPlugin,
};

#[derive(Clone, Default)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CoreNetworkPlugin::default())
            //.add_startup_system(startup.system())
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(startup.system()))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(handle_messages.system())
                    .with_system(handle_events.system())
                    .with_system(handle_server_broadcasts.system())
                    .label("receive")
                    .before("simulation")
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(broadcast_client_data.system())
                    .label("broadcast")
                    .after("simulation")
            );
    }
}

fn startup(mut net: ResMut<NetworkResource>, session: Res<Session>) {
    info!("Starting client...");
    let ip_address: IpAddr;

    if session.address.is_none() {
        ip_address =
            bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    } else {
        ip_address = IpAddr::from_str(&session.clone().address.unwrap()).unwrap();
    }
    let socket_address = SocketAddr::new(ip_address, session.port);
    net.connect(socket_address);
    info!("Connecting to {}...", socket_address);
}
