use bevy::{prelude::*};
use bevy_asset_loader::{AssetCollection, AssetLoader};

use crate::core::AppState;

// ===============================================================
// ======================= CLIENT ASSETS =========================
// ===============================================================

#[derive(Default)]
pub(crate) struct ClientAssetsPlugin;

impl Plugin for ClientAssetsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        AssetLoader::new(AppState::Loading, AppState::InGame)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<TextureAssets>()
            .build(app);
    }
}



#[derive(AssetCollection)]
pub struct FontAssets {
    // #[asset(path = "fonts/FiraSans-Bold.ttf")]
    // pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    // #[asset(path = "sounds/background.ogg")]
    // pub background: Handle<AudioSource>,
    // #[asset(path = "sounds/shot.ogg")]
    // pub tower_shots: Handle<AudioSource>,
    // #[asset(path = "sounds/enemybreach.ogg")]
    // pub enemy_breach: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/doddy.png")]
    pub doddy: Handle<Texture>,
}