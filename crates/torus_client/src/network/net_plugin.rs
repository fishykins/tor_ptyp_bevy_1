use std::{net::{SocketAddr, IpAddr}, str::FromStr};

use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;
use torus_core::{
    flow::{AppState, Session},
    network::NetworkPlugin as CoreNetworkPlugin,
};
use super::events::handle_events;
use super::messages::handle_messages;
use super::broadcasts::broadcast_client_data;

#[derive(Clone, Default)]
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CoreNetworkPlugin::default())
            //.add_startup_system(startup.system())
            .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(startup.system()))
            .add_system_set_to_stage(
                CoreStage::PreUpdate,
                SystemSet::on_update(AppState::InGame)
                    .with_system(handle_messages.system())
                    .with_system(handle_events.system())
            )
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::on_update(AppState::InGame).with_system(broadcast_client_data.system()),
            );
    }
}

fn startup(mut net: ResMut<NetworkResource>, session: Res<Session>) {
    info!("Starting client...");
    let ip_address: IpAddr;

    if session.address.is_none() {
        ip_address = bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    } else {
        ip_address = IpAddr::from_str(&session.clone().address.unwrap()).unwrap();
    }
    let socket_address = SocketAddr::new(ip_address, session.port);
    net.connect(socket_address);
    info!("Connecting to {}...", socket_address);
}