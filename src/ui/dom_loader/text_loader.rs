use super::utils::*;
use crate::{assets::resource_manager::ResourceManager, ui::elements::text::Text};
use minidom::Element;
use sfml::graphics::Color;

/// # Usage
///
/// ## Required:
///
/// ## Optional:
/// - child element (String)
/// - position (UIPosition)
/// - font_size (u32)
/// - color (Color)
pub fn text_loader(
    resource_manager: &ResourceManager,
    ele: &Element,
    default_font_size: u32,
    default_color: Color,
) -> Text {
    Text::new(
        &resource_manager,
        get_ui_position(&ele).unwrap_or_else(|_| Default::default()),
        ele.text().trim(),
        get_font_size_or_default(&ele, default_font_size),
        get_color_attribute_or_default(&ele, default_color),
    )
}
