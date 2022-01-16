use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use torus_core::flow::AppState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
    fn build(&self, mut app: &mut App) {
        AssetLoader::new(AppState::Loading)
            .continue_to_state(AppState::InGame)
            .with_collection::<Images>()
            .with_collection::<Audio>()
            .build(&mut app);
    }
}

#[derive(AssetCollection)]
pub struct Audio {
    //#[asset(path = "walking.ogg")]
//walking: Handle<AudioSource>
}

#[derive(AssetCollection)]
pub struct Images {
    #[asset(path = "images/doddy.png")]
    pub player: Handle<Image>,
}
