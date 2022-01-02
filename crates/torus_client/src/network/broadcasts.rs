use bevy::prelude::*;
use bevy_networking_turbulence::NetworkResource;
use torus_core::{control::Controller, network::messages::ClientMessage};

pub fn broadcast_client_data(mut net: ResMut<NetworkResource>, controller: Query<&Controller>) {
    for controller in controller.iter() {
        net.broadcast_message(ClientMessage::Input(controller.clone()));
    }
}
