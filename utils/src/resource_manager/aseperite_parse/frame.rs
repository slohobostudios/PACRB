use serde::Deserialize;
use sfml::{graphics::Rect, system::Vector2};

use super::utils::{RectDef, Vector2SizeDef};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Frame {
    #[serde(rename = "filename")]
    pub file_name: String,
    #[serde(with = "RectDef")]
    pub frame: Rect<u16>,
    pub rotated: bool,
    pub trimmed: bool,
    #[serde(with = "RectDef", rename = "spriteSourceSize")]
    pub sprite_source_size: Rect<u16>,
    #[serde(with = "Vector2SizeDef", rename = "sourceSize")]
    pub source_size: Vector2<u16>,
    pub duration: u16,
}
