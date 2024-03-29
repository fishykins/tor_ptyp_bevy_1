use std::time::Duration;

use crate::{agents::spawn_players, events::ClientEvent, network::NetworkPlugin};
use bevy::{
    app::{ScheduleRunnerPlugin, ScheduleRunnerSettings},
    diagnostic::DiagnosticsPlugin,
    log::{Level, LogPlugin, LogSettings},
    prelude::*,
};
use torus_core::{
    agents::move_agents,
    flow::{AppState, GameTick, Session},
    network::Local,
    physics::{physics_update, apply_transforms_system},
};

pub fn run(s: Session) {
    let mut app = App::new();

    let mut log_setting = LogSettings::default();
    log_setting.level = Level::INFO;

    // Establish State/Stage relationship.
    AppState::insert(&mut app, AppState::InGame);

    // Resources
    app.insert_resource(s.clone())
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / s.tickrate,
        )))
        .insert_resource(GameTick::default())
        .insert_resource(log_setting);

    // Global resources
    app.insert_resource(GameTick::default())
        .add_event::<ClientEvent>();

    // Plugins
    app.add_plugins(MinimalPlugins)
        .add_plugin(ScheduleRunnerPlugin::default())
        .add_plugin(DiagnosticsPlugin::default())
        .add_plugin(LogPlugin::default())
        .add_plugin(NetworkPlugin::default());

    // Systems
    app.add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(spawn_players.system())
            .with_system(move_agents.system())
            .label("input"),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(physics_update::<Local>)
            .label("physics")
            .after("input"),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(apply_transforms_system)
            .label("transform")
            .after("physics"),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(GameTick::next.system())
            .label("tick")
            .after("physics"),
    );
    app.run();
}
