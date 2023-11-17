use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub(crate) struct SceneAssets {
    pub(crate) apple: Handle<Image>,
}

pub(crate) struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        apple: asset_server.load("pumpkin.png"),
    }
}
