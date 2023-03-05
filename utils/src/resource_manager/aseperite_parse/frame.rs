use sfml::{graphics::Rect, system::Vector2};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Frame {
    pub file_name: String,
    pub frame: Rect<u16>,
    pub rotated: bool,
    pub trimmed: bool,
    pub sprite_source_size: Rect<u16>,
    pub source_size: Vector2<u16>,
    pub duration: u16,
}
