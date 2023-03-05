use serde::Deserialize;
use sfml::system::Vector2;

use super::{frame_tag::FrameTag, slice::Slice};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Meta {
    pub app: String,
    pub version: String,
    pub image: String,
    pub format: String,
    pub size: Vector2<u16>,
    pub scale: f32,
    pub frame_tags: Vec<FrameTag>,
    pub slices: Vec<Slice>,
}
