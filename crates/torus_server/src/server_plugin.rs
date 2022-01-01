use crate::{agents::spawn_players, events::ClientEvent, network::NetworkPlugin};
use bevy::prelude::*;
use torus_core::flow::{AppState, GameTick};

#[derive(Default, Clone, Debug)]
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // Establish State/Stage relationship.
        AppState::insert(app, AppState::InGame);

        // Global resources
        app.insert_resource(GameTick::default())
            .add_event::<ClientEvent>();

        // Plugins
        app.add_plugin(NetworkPlugin::default());

        // Systems
        app.add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::on_update(AppState::InGame).with_system(spawn_players.system()),
        )
        .add_system_set_to_stage(
            CoreStage::Last,
            SystemSet::on_update(AppState::InGame).with_system(GameTick::next.system()),
        );

        //app.add_system(super::debug::monitor_state.system());
    }
}
