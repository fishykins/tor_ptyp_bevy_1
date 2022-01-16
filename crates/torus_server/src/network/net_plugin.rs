use std::net::SocketAddr;

use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;
use torus_core::{
    flow::{AppState, Session},
    network::NetworkPlugin as CoreNetworkPlugin,
};

use super::{broadcast_client_data, handle_client_broadcasts, handle_events, handle_requests};

#[derive(Debug, Default)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CoreNetworkPlugin::default())
            .add_startup_system(startup.system())
            //.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(startup.system()))
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(handle_client_broadcasts)
                    .with_system(handle_events)
                    .with_system(handle_requests)
                    .label("receive")
                    .before("physics"),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                    .with_system(broadcast_client_data)
                    .after("physics"),
            );
    }
}

fn startup(mut net: ResMut<NetworkResource>, session: ResMut<Session>) {
    info!("Starting server...");
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, session.port);
    net.listen(socket_address, None, None);
    info!("Listening on {}", socket_address);
}
