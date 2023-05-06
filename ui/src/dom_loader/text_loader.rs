use super::utils::*;
use crate::elements::text::Text;
use minidom::Element;
use sfml::graphics::Color;
use utils::resource_manager::ResourceManager;

/// # Usage
///
/// ## Required:
///
/// ## Optional:
/// - child element ([`String`])
/// - position ([`UIPosition`](crate::utils::positioning::UIPosition))
/// - font_size ([`u32`])
/// - color ([`Color`])
/// - disable_padding ([`bool`]) *Default = true*
pub fn text_loader(
    resource_manager: &ResourceManager,
    ele: &Element,
    default_font_size: u32,
    default_color: Color,
) -> Text {
    Text::new(
        resource_manager,
        get_ui_position(ele).unwrap_or_else(|_| Default::default()),
        ele.text().trim(),
        get_generic_attribute::<bool>(ele, "disable_padding").unwrap_or(true),
        get_font_size(ele).unwrap_or(default_font_size),
        get_color_attribute(ele).unwrap_or(default_color),
    )
}
