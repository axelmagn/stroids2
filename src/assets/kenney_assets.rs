//! Module for loading assets provided by kenney

use std::str::from_utf8;

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    prelude::{AddAsset, Plugin, Rect},
    reflect::TypeUuid,
};
use serde::Deserialize;

/// A texture atlas corresponding to an image file located at `image_path`
/// Note: this name overlaps with bevy::TextureAtlas
#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "10ee7006-ad73-4111-bf4f-4a1ccf0e6996"]
#[serde(rename = "TextureAtlas")]
#[serde(rename_all = "camelCase")]
pub struct TextureAtlasXml {
    pub image_path: String,
    #[serde(rename = "$value")]
    pub sub_textures: Vec<SubTexture>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubTexture {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl SubTexture {
    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.x + self.width, self.y + self.height)
    }
}

/// Asset loader for TextureAtlas
#[derive(Default)]
pub struct TextureAtlasXmlLoader;

impl AssetLoader for TextureAtlasXmlLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let content = from_utf8(bytes)?;
            let texture_atlas = serde_xml_rs::from_str::<TextureAtlasXml>(content)?;
            load_context.set_default_asset(LoadedAsset::new(texture_atlas));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xml"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_TEXTURE_ATLAS_STR: &'static str = include_str!("test_fixtures/textureAtlas.xml");

    #[test]
    fn test_texture_atlas_parse() {
        let texture_atlas: TextureAtlasXml =
            serde_xml_rs::from_str(TEST_TEXTURE_ATLAS_STR).unwrap();
        assert_eq!(texture_atlas.image_path, "simpleSpace_sheet.png");

        assert_eq!(texture_atlas.sub_textures[0].name, "effect_purple.png");
        assert_eq!(texture_atlas.sub_textures[0].x, 156.);
        assert_eq!(texture_atlas.sub_textures[0].y, 32.);
        assert_eq!(texture_atlas.sub_textures[0].width, 32.);
        assert_eq!(texture_atlas.sub_textures[0].height, 64.);

        assert_eq!(texture_atlas.sub_textures[1].name, "enemy_A.png");
        assert_eq!(texture_atlas.sub_textures[1].x, 0.);
        assert_eq!(texture_atlas.sub_textures[1].y, 420.);
        assert_eq!(texture_atlas.sub_textures[1].width, 48.);
        assert_eq!(texture_atlas.sub_textures[1].height, 48.);

        assert_eq!(texture_atlas.sub_textures[2].name, "icon_crossLarge.png");
        assert_eq!(texture_atlas.sub_textures[2].x, 100.);
        assert_eq!(texture_atlas.sub_textures[2].y, 236.);
        assert_eq!(texture_atlas.sub_textures[2].width, 48.);
        assert_eq!(texture_atlas.sub_textures[2].height, 48.);
    }
}
