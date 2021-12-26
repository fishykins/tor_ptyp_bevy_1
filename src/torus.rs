use bevy::app::{ScheduleRunnerSettings, ScheduleRunnerPlugin};
use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::MinimalPlugins;

use super::client::ClientPlugin;
use super::network::NetworkPlugin;
use super::server::ServerPlugin;
use crate::core::{resources::Session, CorePlugin};
use std::time::Duration;

#[derive(Default)]
/// The main plugin for Torus. This is the entry point for the Torus application, and will determine which plugins to load based on the session type.
pub struct TorusPlugin;


impl Plugin for TorusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let session = app.world().get_resource::<Session>().unwrap().clone();
        println!("{}", session.to_string());

        app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / session.tickrate,
        )));

        // No GUI for headless sessions, so we can use the minimal plugins
        if session.is_headless() {
            println!("Setting up a headless session...");
            app.add_plugins(MinimalPlugins)
                .add_plugin(ScheduleRunnerPlugin::default())
                .add_plugin(DiagnosticsPlugin::default())
                .add_plugin(LogPlugin::default());
        }
        // App has a client, so run a window and rendering proccesses
        else {
            app.insert_resource(Msaa { samples: 4 })
                .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
                .insert_resource(WindowDescriptor {
                    width: 800.,
                    height: 600.,
                    title: "Torus".to_string(),
                    ..Default::default()
                })
                .add_plugins(DefaultPlugins);
        }

        
        app.add_plugin(NetworkPlugin::default());
        app.add_plugin(CorePlugin::default());

        // True if either dedicated server, or client who is hosting.
        if session.is_server() {
            println!("Setting up a server...");
            app.add_plugin(ServerPlugin::default());
        }

        if session.is_client() {
            println!("Setting up a client...");
            app.add_plugin(ClientPlugin::default());
        }


        if session.debug {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
