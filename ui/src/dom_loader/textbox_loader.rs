use std::error::Error;

use minidom::Element as MinidomElement;
use sfml::graphics::Color;
use utils::{
    resource_manager::ResourceManager,
    sfml_util_functions::{color_from_str, invert_color},
    simple_error::SimpleError,
};

use crate::elements::textbox::{
    fixed_size_one_line_textbox::FixedSizeOneLineTextbox, traits::TextBox,
};

use super::utils::{
    get_color_attribute, get_event_id_or_default, get_font_size, get_size, get_sync_id_or_default,
    get_ui_position,
};

/// # Usage
///
/// ## Required
/// - type [`FIXED_SIZE_ONE_LINE_TEXTBOX`]
///
/// ## Optional
/// - position ([`UIPosition`](crate::utils::positioning::UIPosition))
/// - size ([`Vector2u`](sfml::system::Vector2u))
/// - font_size ([`u32`])
/// - color ([`Color`]) <- text color
/// - background_color ([`Color`])
/// - event_id ([`EventId`](crate::events::EventId))
/// - sync_id ([`EventId`](crate::events::EventId))
/// - INNER TEXT CHILD ELEMENT ([`String`])
fn fixed_size_one_line_textbox_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_font_size: u32,
    default_color: Color,
) -> Result<FixedSizeOneLineTextbox, Box<dyn Error>> {
    let text_color = get_color_attribute(minidom_element).unwrap_or(default_color);
    Ok(FixedSizeOneLineTextbox::new(
        resource_manager,
        get_ui_position(minidom_element).unwrap_or_default(),
        get_size(minidom_element).unwrap_or_default().x,
        get_font_size(minidom_element).unwrap_or(default_font_size),
        text_color,
        color_from_str(
            minidom_element
                .attr("background_color")
                .unwrap_or("This color doesn't exist : !"),
        )
        .unwrap_or(invert_color(text_color)),
        minidom_element.text().trim(),
        get_event_id_or_default(minidom_element),
        get_sync_id_or_default(minidom_element),
    ))
}

const FIXED_SIZE_ONE_LINE_TEXTBOX: &str = "FixedSizeOneLineTextbox";
const TEXTBOX_STYLES: [&str; 1] = [FIXED_SIZE_ONE_LINE_TEXTBOX];
pub fn textbox_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    _default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Box<dyn TextBox>, Box<dyn Error>> {
    match minidom_element.attr("type") {
        Some(FIXED_SIZE_ONE_LINE_TEXTBOX) => Ok(Box::new(fixed_size_one_line_textbox_loader(
            resource_manager,
            minidom_element,
            default_font_size,
            default_color,
        )?)),
        string => Err(Box::new(SimpleError::new(format!(
            "Unabe to parse type: {:#?} for valid textbox type. List of valid textbox types: {:#?}",
            string, TEXTBOX_STYLES
        )))),
    }
}
