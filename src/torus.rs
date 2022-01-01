use bevy::app::{ScheduleRunnerPlugin, ScheduleRunnerSettings};
use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::MinimalPlugins;

use torus_core::flow::Session;
use torus_server::ServerPlugin;
use std::time::Duration;

#[derive(Default)]
/// The main plugin for Torus. This is the entry point for the Torus application, and will determine which plugins to load based on the session type.
pub struct TorusPlugin;

impl Plugin for TorusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let session = app.world().get_resource::<Session>().unwrap().clone();

        app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / session.tickrate,
        )));

        // No GUI for headless sessions, so we can use the minimal plugins
        if session.is_headless() {
            app.add_plugins(MinimalPlugins)
                .add_plugin(ScheduleRunnerPlugin::default())
                .add_plugin(DiagnosticsPlugin::default())
                .add_plugin(LogPlugin::default());
        }
        // App has an interface, so run window and rendering proccesses
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

        // True if either dedicated server, or client who is hosting.
        if session.is_server() {
            println!("Starting server...");
            app.add_plugin(ServerPlugin::default());
        }

        if session.is_client() {
            println!("Starting client...");
        }

        if session.debug {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
