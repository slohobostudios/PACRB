use crate::elements::missing_texture::MissingTexture;

use super::utils::*;
use minidom::Element as MinidomElement;
use sfml::system::Vector2;
use utils::resource_manager::ResourceManager;

/// # Usage
///
/// ## Optional
/// - position ([`UIPosition`])
/// - size ([`Vector2`])
pub fn missing_texture_loader(
    resource_manager: &ResourceManager,
    ele: &MinidomElement,
) -> MissingTexture {
    MissingTexture::new(
        resource_manager,
        get_ui_position(ele).unwrap_or_default(),
        get_size(ele).unwrap_or(Vector2::new(32, 32)),
    )
}
