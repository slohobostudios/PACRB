use super::utils::*;
use crate::{
    elements::background::{
        repeatable_3x3_background::Repeatable3x3Background, traits::BackgroundElement,
    },
    utils::positioning::UIPosition,
};
use minidom::Element as MinidomElement;
use sfml::graphics::Color;
use std::error::Error;
use utils::{resource_manager::ResourceManager, simple_error::SimpleError};

/// # Usage
///
/// ## Required
/// - type [`REPEATABLE_3X3_BACKGROUND`]
/// - asset ([`String`])
/// - frame_id ([`usize`])
///
/// ## Optional
/// - scale ([`f32`])
/// - size ([`Vector2`])
/// - position ([`UIPosition`])
/// - padding ([`UIPosition`])
///
/// ## Notes
/// padding can be used instead of size if you only have one child element
fn fixed_size_repeatable_3x3_background(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Repeatable3x3Background, Box<dyn Error>> {
    Ok(Repeatable3x3Background::new(
        resource_manager,
        collect_children_as_vector(
            resource_manager,
            minidom_element,
            default_scale,
            default_font_size,
            default_color,
        ),
        get_ui_position(minidom_element).unwrap_or_default(),
        &get_asset_id(minidom_element)?,
        minidom_element
            .attr("frame_id")
            .ok_or("no frame_id defined")?
            .parse::<u16>()?,
        get_generic_attribute::<UIPosition>(minidom_element, "padding").unwrap_or_default(),
        get_size(minidom_element).ok(),
        get_scale(minidom_element).unwrap_or(default_scale),
    ))
}

const REPEATABLE_3X3_BACKGROUND: &'static str = "Repeatable3x3Background";
const BACKGROUND_TYPES: [&'static str; 1] = [REPEATABLE_3X3_BACKGROUND];
pub fn background_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Box<dyn BackgroundElement>, Box<dyn Error>> {
    match minidom_element.attr("type") {
        Some(REPEATABLE_3X3_BACKGROUND) => Ok(Box::new(fixed_size_repeatable_3x3_background(
            &resource_manager,
            &minidom_element,
            default_scale,
            default_font_size,
            default_color,
        )?)),
        string => Err(Box::new(SimpleError::new(
            format!("Unable to parse type: {:#?} for valid background type. List of viable background types: {:#?}", string, BACKGROUND_TYPES),
        ))),
    }
}
