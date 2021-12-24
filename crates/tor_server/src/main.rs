// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use argh::FromArgs;
use bevy::app::ScheduleRunnerSettings;
use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::{info, LogPlugin};
use bevy::prelude::*;
use bevy::MinimalPlugins;
use bevy_networking_turbulence::{NetworkEvent, NetworkResource, NetworkingPlugin, Packet};
use std::time::Duration;

mod startup;

#[derive(FromArgs)]
/// A server application to host the game "Torus"
struct Flags {
    /// tickrate of the server
    #[argh(option, default = "60.0")]
    tickrate: f64,

    /// enable diagnostics in the console
    #[argh(switch, short = 'd')]
    debug: bool,
}

fn main() {
    let args: Flags = argh::from_env();
    let mut app = App::build();
    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / args.tickrate,
    )));
    app.add_plugins(MinimalPlugins);
    app.add_plugin(LogPlugin::default());
    app.add_plugin(NetworkingPlugin::default());
    app.add_startup_system(startup::startup.system());
    app.add_system(handle_packets.system());

    if args.debug {
        app.add_plugin(DiagnosticsPlugin::default());
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(LogDiagnosticsPlugin::default());
    }

    app.run();
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
