use minidom::Element as MinidomElement;
use sfml::graphics::Color;
use std::error::Error;
use utils::{resource_manager::ResourceManager, sfml_util_functions::vector2_from_str};

use crate::{elements::div::Div, utils::positioning::UIPosition};

use super::utils::{
    collect_children_as_vector, get_color_attribute, get_font_size, get_generic_attribute,
    get_scale, get_ui_position,
};

/// # Usage
///
/// ## Required:
///
/// ## Optional:
/// - position ([`UIPosition`])
/// - padding ([`UIPosition`])
/// - scale ([`f32`])
/// - font_size ([`u32`])
/// - color ([`Color`])
pub fn div_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Div, Box<dyn Error>> {
    Ok(Div::new(
        resource_manager,
        get_ui_position(minidom_element).unwrap_or_default(),
        collect_children_as_vector(
            resource_manager,
            minidom_element,
            get_scale(minidom_element).unwrap_or(default_scale),
            get_font_size(minidom_element).unwrap_or(default_font_size),
            get_color_attribute(minidom_element).unwrap_or(default_color),
        ),
        get_generic_attribute::<UIPosition>(minidom_element, "padding"),
        minidom_element
            .attr("size")
            .and_then(|size| vector2_from_str(size).ok()),
    ))
}
