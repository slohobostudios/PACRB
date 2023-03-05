use serde::Deserialize;
use sfml::graphics::IntRect;

#[derive(Debug, Clone, Copy, Default, Deserialize)]
pub struct SliceKey {
    pub frame: usize,
    pub bounds: IntRect,
}
