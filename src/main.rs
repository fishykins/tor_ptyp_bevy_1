// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod client;
mod core;
mod server;
mod torus;

use argh::FromArgs;
use bevy::prelude::*;
use torus::TorusPlugin;

use crate::core::Session;

#[derive(FromArgs)]
/// A server application to host the game "Torus"
pub struct Flags {
    /// number of ticks per second
    #[argh(option, short = 't', default = "60.0")]
    pub tickrate: f64,

    /// port to listen on
    #[argh(option, short = 'p', default = "14200")]
    pub port: u16,

    /// enable diagnostics in the console
    #[argh(switch, short = 'd')]
    pub debug: bool,

    /// is server
    #[argh(switch, short = 's')]
    pub server: bool,

    /// is client
    #[argh(switch, short = 'c')]
    pub client: bool,
}

fn main() {
    // We are only going to parse startup arguments, the rest we will leave up to the official Torus plugin.
    let args: Flags = argh::from_env();
    let s = Session::new(
        args.server,
        args.client,
        args.port,
        args.debug,
        args.tickrate,
    );

    let mut app = App::build();
    app.init_resource::<Session>()
        .insert_resource(s)
        .add_plugin(TorusPlugin::default());
    app.run();
}
