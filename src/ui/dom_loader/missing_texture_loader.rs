use super::utils::*;
use crate::{
    assets::resource_manager::ResourceManager, ui::elements::missing_texture::MissingTexture,
};
use minidom::Element as MinidomElement;
use sfml::system::Vector2;

/// # Usage
///
/// ## Optional
/// - position (UIPosition)
/// - size (Vector2)
pub fn missing_texture_loader(
    resource_manager: &ResourceManager,
    ele: &MinidomElement,
) -> MissingTexture {
    MissingTexture::new(
        resource_manager,
        get_ui_position(&ele).unwrap_or_default(),
        get_size_or_default(ele, Vector2::new(32, 32)),
    )
}
