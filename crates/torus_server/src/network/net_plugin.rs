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
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CoreNetworkPlugin::default())
            .add_startup_system(startup.system())
            //.add_system_set(SystemSet::on_enter(AppState::InGame).with_system(startup.system()))
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_update(AppState::InGame)
                    .with_system(handle_client_broadcasts.system())
                    .with_system(handle_events.system())
                    .with_system(handle_requests.system()),
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::InGame).with_system(broadcast_client_data.system()),
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
