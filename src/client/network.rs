use std::{net::{SocketAddr, IpAddr}, str::FromStr};

use bevy::{log, prelude::*};
use bevy_networking_turbulence::{NetworkEvent, NetworkResource};

use crate::core::{
    components::Controller,
    network::{ClientId, ClientMessage, ClientRequest, ServerMessage, ServerResponse},
    Session,
};

// ===============================================================
// ====================== CLIENT NETWORKING ======================
// ===============================================================

/// This plugin handles network initialization and events.
#[derive(Default)]
pub(crate) struct ClientNetworkPlugin;

impl Plugin for ClientNetworkPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(startup.system())
            .add_system_to_stage(CoreStage::PreUpdate, handle_events.system())
            .add_system_to_stage(CoreStage::PreUpdate, handle_messages.system())
            .add_system_to_stage(CoreStage::PostUpdate, relay_controls.system())
            .insert_resource(ClientId::default());
    }
}

// ===============================================================
// ========================= SYSTEMS =============================
// ===============================================================

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

fn handle_events(mut net: ResMut<NetworkResource>, mut network_events: EventReader<NetworkEvent>) {
    for event in network_events.iter() {
        match event {
            NetworkEvent::Connected(handle) => match net.connections.get_mut(handle) {
                Some(_connection) => {
                    log::info!("Connected.");

                    match net.send_message(*handle, ClientRequest::Join) {
                        Ok(msg) => match msg {
                            Some(msg) => {
                                log::error!("Unable to send handshake: {:?}", msg);
                            }
                            None => {}
                        },
                        Err(err) => {
                            log::error!("Unable to send handshake: {:?}", err);
                        }
                    };
                }
                None => panic!("Got packet for non-existing connection [{}]", handle),
            },
            _ => {}
        }
    }
}

/// Handles explicit messages sent from server..
fn handle_messages(mut net: ResMut<NetworkResource>, mut commands: Commands) {
    for (_, connection) in net.connections.iter_mut() {
        let channels = connection.channels().unwrap();
        while let Some(msg) = channels.recv::<ServerResponse>() {
            match msg {
                ServerResponse::Id(id) => {
                    log::info!("Server has allocated {}", id);
                    commands.insert_resource(ClientId::new(id));
                }
                ServerResponse::Spawn(spawn) => {
                    log::info!("Spawning data: {}", spawn);
                }
            }
        }
        while let Some(msg) = channels.recv::<ServerMessage>() {
            match msg {
                ServerMessage::GoonState(goons) => {
                    //log::info!("Got goon state: {:?}", goons);
                    commands.insert_resource(goons);
                }
            }
        }
    }
}

fn relay_controls(mut net: ResMut<NetworkResource>, controller: Query<&Controller>) {
    for controller in controller.iter() {
        net.broadcast_message(ClientMessage::Input(controller.clone()));
        //log::info!("Sending input: {:?}", controller);
    }
}

// ===============================================================
// ========================= RESOURCES ===========================
// ===============================================================
