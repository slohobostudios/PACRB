use serde::Deserialize;
use sfml::graphics::IntRect;

use super::utils::RectDef;

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct SliceKey {
    pub frame: usize,
    #[serde(with = "RectDef")]
    pub bounds: IntRect,
}

impl Default for SliceKey {
    fn default() -> Self {
        Self {
            frame: Default::default(),
            bounds: IntRect::new(0, 0, 16, 16),
        }
    }
}
