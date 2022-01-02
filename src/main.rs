// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use argh::FromArgs;
use torus_core::flow::Session;
use torus_client;
use torus_server;

#[derive(FromArgs)]
/// A server application to host the game "Torus"
pub struct Flags {
    /// number of ticks per second
    #[argh(option, short = 't', default = "60.0")]
    pub tickrate: f64,

    /// port to listen on
    #[argh(option, short = 'p', default = "14200")]
    pub port: u16,

    /// server remote address
    #[argh(option, short = 'a')]
    pub address: Option<String>,

    /// enable diagnostics in the console
    #[argh(switch, short = 'd')]
    pub debug: bool,

    /// is server
    #[argh(switch, short = 's')]
    pub server: bool,
}

fn main() {
    // We are only going to parse startup arguments, the rest we will leave up to the official Torus plugin.
    let args: Flags = argh::from_env();
    let s = Session::new(
        args.server,
        !args.server,
        args.address,
        args.port,
        args.debug,
        args.tickrate,
    );

    if s.is_server() {
        println!("Starting Torus server...");
        torus_server::run(s);
    } else {
        println!("Starting Torus client...");
        torus_client::run(s);
    }
}
