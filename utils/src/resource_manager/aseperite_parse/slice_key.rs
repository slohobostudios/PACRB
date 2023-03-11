use serde::Deserialize;
use sfml::graphics::IntRect;

use super::utils::RectDef;

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct SliceKey {
    pub frame: usize,
    #[serde(with = "RectDef")]
    pub bounds: IntRect,
}
