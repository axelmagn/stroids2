//! Module for loading assets provided by kenney

use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::{collections::HashMap, str::from_utf8};

use bevy::render::render_resource::Texture;
use bevy::render::texture::{self, CompressedImageFormats, ImageType};
use bevy::utils::default;
use bevy::{
    asset::{AssetLoader, Error as AssetError, LoadedAsset},
    prelude::{AddAsset, Handle, Image, Plugin, Rect},
    reflect::TypeUuid,
    sprite::TextureAtlas,
    utils::BoxedFuture,
};
use serde::Deserialize;

/// A texture atlas with named sprites
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "a21062d1-791a-4dc8-83e2-7521b6144c11"]
pub struct KeyedTextureAtlas {
    pub keys: HashMap<String, usize>,
    pub atlas: Handle<TextureAtlas>,
}

#[derive(Debug, Deserialize, Clone, TypeUuid)]
#[uuid = "10ee7006-ad73-4111-bf4f-4a1ccf0e6996"]
#[serde(rename = "TextureAtlas")]
#[serde(rename_all = "camelCase")]
pub struct KeyedTextureAtlasConfig {
    pub image_path: String,
    #[serde(rename = "$value")]
    pub sub_textures: Vec<SubTexture>,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Default)]
pub struct KeyedTextureAtlasLoader;

impl AssetLoader for KeyedTextureAtlasLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> BoxedFuture<'a, Result<(), AssetError>> {
        Box::pin(async move {
            // parse xml
            let xml_content = from_utf8(bytes)?;
            let config = serde_xml_rs::from_str::<KeyedTextureAtlasConfig>(xml_content)?;
            let config_handle =
                load_context.set_labeled_asset("config", LoadedAsset::new(config.clone()));

            // load related img
            let base_dir = load_context.path().parent().ok_or(IOError::new(
                IOErrorKind::NotFound,
                String::from("could not find base_dir"),
            ))?;
            let img_path = base_dir.join(config.image_path);
            let img_bytes = load_context.read_asset_bytes(img_path.clone()).await?;
            let ext = img_path
                .extension()
                .and_then(|ext| ext.to_str())
                .ok_or(IOError::new(
                    IOErrorKind::NotFound,
                    String::from("could not find image file extension"),
                ))?;

            let img = Image::from_buffer(
                &img_bytes,
                ImageType::Extension(ext),
                CompressedImageFormats::default(),
                true,
            )?;
            let img_size = img.size();
            let img_handle = load_context.set_labeled_asset("image", LoadedAsset::new(img));

            // create texture atlas
            let atlas = TextureAtlas::new_empty(img_handle, img_size);

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xml"]
    }
}

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
            let texture_atlas = serde_xml_rs::from_str::<KeyedTextureAtlasConfig>(content)?;
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
        let texture_atlas: KeyedTextureAtlasConfig =
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
