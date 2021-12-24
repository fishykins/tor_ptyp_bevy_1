// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use argh::FromArgs;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::DefaultPlugins;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};
use std::net::SocketAddr;

const SERVER_PORT: u16 = 14191;

#[derive(FromArgs)]
/// A server application to host the game "Torus"
struct Flags {
    /// enable diagnostics in the console
    #[argh(switch, short = 'd')]
    debug: bool,
}

fn main() {
    let args: Flags = argh::from_env();
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Torus".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(NetworkingPlugin::default())
        .add_startup_system(startup.system())
        .add_system(handle_packets.system())
        .add_system(send_packets.system());

        if args.debug {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    app.run();
}

fn startup(mut net: ResMut<NetworkResource>) {
    let ip_address =
        bevy_networking_turbulence::find_my_ip_address().expect("can't find ip address");
    let socket_address = SocketAddr::new(ip_address, SERVER_PORT);
    info!("Starting client");
    net.connect(socket_address);
}

fn send_packets(mut net: ResMut<NetworkResource>, time: Res<Time>) {
    if (time.seconds_since_startup() * 60.) as i64 % 60 == 0 {
        info!("PING");
        net.broadcast(Packet::from("PING"));
    }
}

fn handle_packets(
    mut net: ResMut<NetworkResource>,
    time: Res<Time>,
    mut reader: EventReader<NetworkEvent>,
) {
    for event in reader.iter() {
        match event {
            NetworkEvent::Packet(handle, packet) => {
                let message = String::from_utf8_lossy(packet);
                info!("Got packet on [{}]: {}", handle, message);
                if message == "PING" {
                    let message = format!("PONG @ {}", time.seconds_since_startup());
                    match net.send(*handle, Packet::from(message)) {
                        Ok(()) => {
                            info!("Sent PONG");
                        }
                        Err(error) => {
                            info!("PONG send error: {}", error);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
