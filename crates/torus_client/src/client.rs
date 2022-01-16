use std::time::Duration;

use crate::{agents::spawn_agents, assets::*, input::InputPlugin, network::NetworkPlugin, camera_update, world::spawn_deco};
use bevy::{
    app::ScheduleRunnerSettings,
    log::{Level, LogSettings},
    prelude::*,
    render::camera::WindowOrigin,
};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use bevy_prototype_lyon::plugin::ShapePlugin;
use torus_core::{
    agents::{move_agents, AgentEvent},
    flow::{AppState, GameTick, Session},
    network::{data::ClientId, Local},
    physics::{physics_update, transform_update},
};

pub fn run(s: Session) {
    // Debug settings
    let mut log_setting = LogSettings::default();
    log_setting.level = Level::INFO;

    // Build the app
    let mut app = App::new();

    AppState::insert(&mut app, AppState::Loading);

    // Resources
    app.insert_resource(s.clone())
        .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
            1.0 / s.tickrate,
        )))
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Torus client".to_string(),
            ..Default::default()
        })
        .insert_resource(GameTick::default())
        .insert_resource(ClientId::default())
        .insert_resource(log_setting);

    // Plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(AssetPlugin)
        .add_plugin(NetworkPlugin)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InputPlugin)
        .add_plugin(ShapePlugin);

    // Events
    app.add_event::<AgentEvent>();

    // Register inspectables
    app.register_inspectable::<torus_core::agents::Agent>()
        .register_inspectable::<torus_core::agents::Controller>()
        .register_inspectable::<torus_core::physics::Rigidbody<torus_core::network::Local>>()
        .register_inspectable::<torus_core::physics::Rigidbody<torus_core::network::Remote>>()
        .register_inspectable::<torus_core::agents::Biped>();

    // Systems
    app.add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(startup)
            .label("startup"),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(move_agents)
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
            .with_system(transform_update)
            .with_system(camera_update)
            .label("transform")
            .after("physics"),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(spawn_agents)
            .label("spawning"),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(GameTick::next)
            .label("tick")
            .after("broadcast"),
    );

    app.run();
}

#[derive(Component, Debug)]
pub struct MainCamera;

fn startup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera).insert(MainCamera);
    for _ in 0..20 {
        spawn_deco(&mut commands);
    }
}
