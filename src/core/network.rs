use crate::core::input::MovementWrapper;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_networking_turbulence::{
    ConnectionChannelsBuilder, MessageChannelMode, MessageChannelSettings, NetworkResource,
    NetworkingPlugin as TurbulenceNetPlugin, ReliableChannelSettings,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

// Housing for all the message types that can be sent from the client to the server.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {
    Handshake(String),
    Input(MovementWrapper),
}

#[derive(Default)]
/// A plugin that handles basic universal network events.
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_plugin(TurbulenceNetPlugin::default())
            .add_startup_system(setup.system());
        //.add_system(handle_packets.system());
    }
}

/// Universal initialization for network systems.
pub fn setup(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_STATE_MESSAGE_SETTINGS)
            .unwrap();
        builder
            .register::<GameStateMessage>(GAME_STATE_MESSAGE_SETTINGS)
            .unwrap();
    });
}

#[derive(Default)]
pub struct Broadcast {
    pub frame: u32,
}

pub const CLIENT_STATE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 0,
    channel_mode: MessageChannelMode::Reliable {
        reliability_settings: ReliableChannelSettings {
            bandwidth: 4096,
            recv_window_size: 1024,
            send_window_size: 1024,
            burst_bandwidth: 1024,
            init_send: 512,
            wakeup_time: Duration::from_millis(100),
            initial_rtt: Duration::from_millis(200),
            max_rtt: Duration::from_secs(2),
            rtt_update_factor: 0.1,
            rtt_resend_factor: 1.5,
        },
        max_message_len: 1024,
    },
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

pub const GAME_STATE_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameStateMessage {
    pub frame: u32,
    pub agents: Vec<(u32, Vec3)>,
}
