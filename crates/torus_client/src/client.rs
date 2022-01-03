use std::time::Duration;

use crate::{agents::AgentPlugin, network::NetworkPlugin, input::InputPlugin};
use bevy::{
    app::ScheduleRunnerSettings,
    log::{Level, LogSettings},
    prelude::*,
    render::camera::WindowOrigin,
};
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};
use torus_core::{
    flow::{AppState, GameTick, Session},
    network::data::ClientId,
};

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/doddy.png")]
    pub doddy: Handle<Texture>,
}

pub fn run(s: Session) {
    // Debug settings
    let mut log_setting = LogSettings::default();
    log_setting.level = Level::DEBUG;

    // Build the app
    let mut app = App::build();

    AppState::insert(&mut app, AppState::Loading);

    AssetLoader::new(AppState::Loading)
        .continue_to_state(AppState::InGame)
        .with_collection::<TextureAssets>()
        .build(&mut app);

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
            title: "Torus".to_string(),
            ..Default::default()
        })
        .insert_resource(GameTick::default())
        .insert_resource(ClientId::default())
        .insert_resource(log_setting);

    // Plugins
    app.add_plugins(DefaultPlugins)
        .add_plugin(AgentPlugin::default())
        .add_plugin(NetworkPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InputPlugin::default());

    // Register inspectables
    app.register_inspectable::<torus_core::agents::Agent>()
    .register_inspectable::<torus_core::control::Controller>()
    .register_inspectable::<torus_core::physics::Body<torus_core::network::Local>>()
    .register_inspectable::<torus_core::physics::Body<torus_core::network::Remote>>()
        .register_inspectable::<torus_core::agents::Biped>();

    // Systems
    app.add_system_set(
        SystemSet::on_enter(AppState::InGame)
            .with_system(startup.system())
            .label("startup"),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(monitor_state.system())
            .label("simulation"),
    )
    .add_system_set(
        SystemSet::on_update(AppState::InGame)
            .with_system(GameTick::next.system())
            .after("broadcasts"),
    );

    app.run();
}

pub struct MainCamera;

fn startup(mut commands: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.window_origin = WindowOrigin::BottomLeft;
    commands.spawn_bundle(camera).insert(MainCamera);
}

fn monitor_state(_state: ResMut<State<AppState>>) {
    //bevy::log::debug!("{:?}", state.current());
}
