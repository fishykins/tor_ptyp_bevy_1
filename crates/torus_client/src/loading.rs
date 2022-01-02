use bevy::prelude::*;
use torus_core::flow::AppState;
use bevy_asset_loader::{AssetLoader, AssetCollection};

#[derive(Default)]
pub(crate) struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<AssetsLoading>()
        .add_startup_system(setup.system())
        .add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::on_update(AppState::Loading).with_system(check_assets_ready.system()),
        );
    }
}

#[derive(Default)]
struct AssetsLoading(Vec<HandleUntyped>);

fn setup(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    let doddy: Handle<Texture> = server.load("textures/doddy.png");
    loading.0.push(doddy.clone_untyped());
}

fn check_assets_ready(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    server: Res<AssetServer>,
    loading: Res<AssetsLoading>,
) {
    use bevy::asset::LoadState;

    match server.get_group_load_state(loading.0.iter().map(|h| h.id)) {
        LoadState::Failed => {
            // one of our assets had an error
            bevy::log::error!("One of the assets failed to load");
            commands.remove_resource::<AssetsLoading>();
            state.set(AppState::InGame).unwrap();
        }
        LoadState::Loaded => {
            bevy::log::info!("All assets are ready- transitioning to InGame");
            commands.remove_resource::<AssetsLoading>();
            state.set(AppState::InGame).unwrap();
        }
        _ => {
            // NotLoaded/Loading: not fully ready yet
        }
    }
}


// use self::textures::TextureAssets;
// use bevy_asset_loader::AssetLoader;

// #[derive(Default)]
// pub(crate) struct AssetsPlugin;

// impl Plugin for AssetsPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         AssetLoader::new(AppState::Loading)
//             .continue_to_state(AppState::InGame)
//             //.with_collection::<FontAssets>()
//             //.with_collection::<AudioAssets>()
//             .with_collection::<TextureAssets>()
//             .build(app);
//     }
// }