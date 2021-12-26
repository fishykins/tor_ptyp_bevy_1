use bevy::prelude::*;
use bevy::app::ScheduleRunnerSettings;
use bevy::diagnostic::{DiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::log::LogPlugin;
use bevy::MinimalPlugins;

use crate::core::resources::Session;

use super::client::ClientPlugin;
use super::network::NetworkPlugin;
use super::server::ServerPlugin;
use std::time::Duration;

#[derive(Default)]
/// The main plugin for Torus. This is the entry point for the Torus application, and will determine which plugins to load based on the session type.
pub struct TorusPlugin {
    pub session: Session,
}

impl TorusPlugin {
    pub fn new(session: Session) -> Self {
        Self { session }
    }
}

impl Plugin for TorusPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / self.session.tickrate,
        )));
        app.init_resource::<Session>().insert_resource(self.session.clone());
    
        // No GUI for headless sessions, so we can use the minimal plugins
        if self.session.is_headless() {
            app.add_plugins(MinimalPlugins);
            app.add_plugin(DiagnosticsPlugin::default());
            app.add_plugin(LogPlugin::default());
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
    
        // True if either dedicated server, or client who is hosting.
        if self.session.is_server() {
            app.add_plugin(ServerPlugin::default());
        }
    
        if self.session.is_client() {
            app.add_plugin(ClientPlugin::default());
        }
    
        app.add_plugin(NetworkPlugin::default());
    
        if self.session.debug {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default());
            app.add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}