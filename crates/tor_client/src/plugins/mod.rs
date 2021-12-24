use bevy::app::AppBuilder;
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use tor_core::GameState;

pub mod audio;
pub mod input;
pub mod loading;
pub mod player;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(loading::LoadingPlugin)
            .add_plugin(input::InputPlugin)
            .add_plugin(audio::InternalAudioPlugin)
            .add_plugin(player::PlayerPlugin);

        #[cfg(debug_assertions)]
        {
            app.add_plugin(FrameTimeDiagnosticsPlugin::default())
                .add_plugin(LogDiagnosticsPlugin::default());
        }
    }
}
