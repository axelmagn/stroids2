use bevy::prelude::{AddAsset, Plugin};

pub mod kenney_assets;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<kenney_assets::KeyedTextureAtlasConfig>();
        app.init_asset_loader::<kenney_assets::TextureAtlasXmlLoader>();
    }
}
