use bevy::prelude::*;
use torus_core::flow::AppState;

pub fn monitor_state(state: Res<State<AppState>>) {
    bevy::log::info!("AppState: {:?}", state.current());
}