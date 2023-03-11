use super::{element_loader::element_loader, utils::*};
use crate::elements::background::{
    fixed_size_repeatable_3x3_background::FixedSizeRepeatable3x3Background,
    traits::BackgroundElement,
};
use minidom::Element as MinidomElement;
use sfml::graphics::Color;
use std::error::Error;
use utils::{resource_manager::ResourceManager, simple_error::SimpleError};

/// # Usage
///
/// ## Required
/// - type "Fixed3x3RepeatableBackground"
/// - asset (String)
/// - frame_id (usize)
///
/// ## Optional
/// - scale (f32)
/// - size (Vector2)
/// - position (UIPosition)
fn fixed_size_repeatable_3x3_background(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<FixedSizeRepeatable3x3Background, Box<dyn Error>> {
    Ok(FixedSizeRepeatable3x3Background::new(
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
        get_size_or_default(minidom_element, Default::default()),
        get_scale_or_default(minidom_element, default_scale),
    ))
}

const FIXED_3X3_REPEATABLE_BACKGROUND: &'static str = "Fixed3x3RepeatableBackground";
const BACKGROUND_TYPES: [&'static str; 1] = [FIXED_3X3_REPEATABLE_BACKGROUND];
pub fn background_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Box<dyn BackgroundElement>, Box<dyn Error>> {
    match minidom_element.attr("type") {
        Some(FIXED_3X3_REPEATABLE_BACKGROUND) => Ok(Box::new(fixed_size_repeatable_3x3_background(
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
