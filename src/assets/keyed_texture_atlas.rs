use std::collections::HashMap;

use bevy::{
    prelude::{Handle, Image, Vec2},
    reflect::TypeUuid,
    sprite::TextureAtlas,
};

use super::kenney_assets::TextureAtlasXml;

/// A texture atlas with named sprites
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "a21062d1-791a-4dc8-83e2-7521b6144c11"]
pub struct KeyedTextureAtlas {
    pub keys: HashMap<String, usize>,
    pub atlas: TextureAtlas,
}

impl KeyedTextureAtlas {
    pub fn new_empty(texture: Handle<Image>, dimensions: Vec2) -> Self {
        Self {
            keys: HashMap::new(),
            atlas: TextureAtlas::new_empty(texture, dimensions),
        }
    }

    pub fn from_xml(texture: Handle<Image>, atlas_xml: &TextureAtlasXml, dimensions: Vec2) -> Self {
        let mut out = Self::new_empty(texture, atlas_xml);
        atlas_xml.sub_textures.iter().for_each(|t| {
            let idx = out.atlas.add_texture(t.rect());
            out.keys.insert(t.name.clone(), idx);
        });
        out
    }
}
