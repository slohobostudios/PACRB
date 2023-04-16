use serde::Deserialize;
use sfml::system::Vector2;
use tracing::error;

use super::{
    frame_tag::FrameTag,
    slice::Slice,
    utils::{string_as_f32, Vector2SizeDef},
};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    #[serde(with = "Vector2SizeDef")]
    pub size: Vector2<u16>,
    #[serde(deserialize_with = "string_as_f32")]
    pub scale: f32,
    #[serde(rename = "frameTags")]
    pub frame_tags: Vec<FrameTag>,
    pub slices: Vec<Slice>,
}

impl Meta {
    pub fn fetch_frame_tag_with_name(&self, name: &str) -> FrameTag {
        self.frame_tags
            .iter()
            .cloned()
            .find(|frame_tag| frame_tag.name == name)
            .unwrap_or_else(|| {
                error!("No frame_tag with name {}", name);
                Default::default()
            })
    }
    pub fn fetch_slice_with_name(&self, name: &str) -> Slice {
        self.slices
            .iter()
            .cloned()
            .find(|slice| slice.name == name)
            .unwrap_or_else(|| {
                error!("No slice with name {}", name);
                Default::default()
            })
    }
}
