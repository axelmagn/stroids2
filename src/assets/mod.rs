use bevy::prelude::{AddAsset, Plugin};

pub mod kenney_assets;
pub mod keyed_texture_atlas;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<kenney_assets::TextureAtlasXml>();
        app.add_asset::<keyed_texture_atlas::KeyedTextureAtlas>();
        app.init_asset_loader::<kenney_assets::TextureAtlasXmlLoader>();
    }
}
