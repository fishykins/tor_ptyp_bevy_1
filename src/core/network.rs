use bevy::math::Vec2;
use bevy::prelude::*;
use bevy_networking_turbulence::{
    ConnectionChannelsBuilder, MessageChannelMode, MessageChannelSettings, NetworkResource,
    NetworkingPlugin as TurbulenceNetPlugin, ReliableChannelSettings,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::time::Duration;

use super::components::ServerData;

// ===============================================================
// ===================== CORE NETWORKING =========================
// ===============================================================

#[derive(Default)]
/// A plugin that handles basic universal network events.
pub struct CoreNetworkPlugin;

impl Plugin for CoreNetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.add_plugin(TurbulenceNetPlugin::default())
            .add_startup_system(setup.system());
    }
}

// ===============================================================
// ======================== SYSTEMS ==============================
// ===============================================================

/// Universal initialization for network systems.
pub fn setup(mut net: ResMut<NetworkResource>) {
    net.set_channels_builder(|builder: &mut ConnectionChannelsBuilder| {
        builder
            .register::<ClientMessage>(CLIENT_MESSAGE_SETTINGS)
            .unwrap();
        builder
            .register::<ClientRequest>(CLIENT_REQUEST_SETTINGS)
            .unwrap();
        builder
            .register::<ServerMessage>(SERVER_MESSAGE_SETTINGS)
            .unwrap();
        builder
            .register::<ServerResponse>(SERVER_RESPONSE_SETTINGS)
            .unwrap();
    });
}

// ===============================================================
// ======================= RESOURCES =============================
// ===============================================================

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct ClientId(pub ServerData<u32>);

impl ClientId {
    pub fn new(id: u32) -> Self {
        ClientId(ServerData::Acquired(id))
    }
}

impl Display for ClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            ServerData::Acquired(id) => write!(f, "ClientId({})", id),
            ServerData::Waiting(ticks) => write!(f, "Waiting({})", ticks),
            ServerData::None => write!(f, "None"),
        }
    }
}

/// Client FYI broadcasts, such as input data.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientMessage {}

/// Client requests that require server response.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ClientRequest {
    Join,
    Spawn,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerMessage {}

/// A response to a specific client's request.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerResponse {
    Id(u32),
    Spawn(Vec2),
}

#[derive(Default)]
pub struct Broadcast {
    pub frame: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AgentStateMessage {
    pub frame: u32,
    pub agents: Vec<(u32, Vec2)>,
}

// ===============================================================
// ======================== CHANNELS =============================
// ===============================================================

pub const CLIENT_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
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

pub const CLIENT_REQUEST_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 1,
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

pub const SERVER_MESSAGE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 8,
    channel_mode: MessageChannelMode::Unreliable,
    message_buffer_size: 8,
    packet_buffer_size: 8,
};

pub const SERVER_RESPONSE_SETTINGS: MessageChannelSettings = MessageChannelSettings {
    channel: 9,
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
