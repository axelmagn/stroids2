//! Module for loading assets provided by kenney

use std::io::{Error as IOError, ErrorKind as IOErrorKind};
use std::{collections::HashMap, str::from_utf8};

use bevy::render::texture::{CompressedImageFormats, ImageType};
use bevy::sprite::TextureAtlasSprite;
use bevy::{
    asset::{AssetLoader, Error as AssetError, LoadedAsset},
    prelude::{Handle, Image, Rect},
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

impl KeyedTextureAtlas {
    pub fn get_sprite(&self, key: &str) -> Option<TextureAtlasSprite> {
        let idx = self.keys.get(key)?;
        Some(TextureAtlasSprite::new(*idx))
    }
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
            load_context.set_labeled_asset("config", LoadedAsset::new(config.clone()));

            // load image path specified by config
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

            // create texture atlas to hold named sprites
            let mut atlas = TextureAtlas::new_empty(img_handle, img_size);
            let mut keys = HashMap::new();
            for subtexture in config.sub_textures {
                let rect = subtexture.rect();
                let idx = atlas.add_texture(rect);
                keys.insert(subtexture.name, idx);
            }
            let atlas_handle = load_context.set_labeled_asset("atlas", LoadedAsset::new(atlas));
            let keyed_atlas = KeyedTextureAtlas {
                keys,
                atlas: atlas_handle,
            };
            load_context.set_default_asset(LoadedAsset::new(keyed_atlas));

            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["xml"]
    }
}
