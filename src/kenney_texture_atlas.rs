//! Module for parsing sprite sheets provided by kenney.nl

use std::{collections::HashMap, str::from_utf8};

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::TypeUuid,
};
use serde::Deserialize;

/// A texture atlas corresponding to an image file located at `image_path`
#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "10ee7006-ad73-4111-bf4f-4a1ccf0e6996"]
pub struct KenneyTextureAtlas {
    image_path: String,
    sub_textures: HashMap<String, SubTexture>,
}

#[derive(Debug, Deserialize)]
struct SubTexture {
    name: String,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

/// Asset loader for TextureAtlas
pub struct KenneyTextureAtlasLoader;

impl AssetLoader for KenneyTextureAtlasLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let content = from_utf8(bytes)?;
            let texture_atlas = serde_xml_rs::from_str::<KenneyTextureAtlas>(content)?;
            load_context.set_default_asset(LoadedAsset::new(texture_atlas));
            Ok(())
        })
        // TODO: look into load_context.set_labeled_asset and see if this can be used to load the png as well
        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &["xml"]
    }
}
