use bevy::prelude::*;
use torus_core::flow::AppState;

pub fn start_game(mut state: ResMut<State<AppState>>) {
    state.set(AppState::InGame).unwrap();
}