use std::error::Error;

use minidom::Element as MinidomElement;
use sfml::graphics::Color;
use utils::{resource_manager::ResourceManager, simple_error::SimpleError};

use crate::{
    dom_loader::utils::{
        get_asset_id, get_color_attribute, get_event_id_or_default, get_font_size,
        get_generic_attribute, get_scale, get_sync_id_or_default, get_ui_position,
    },
    elements::listbox::{traits::ListBox, up_down_scroll_listbox::UpDownScrollListBox},
    utils::positioning::UIPosition,
};

/// # Usage
///
/// ## Required
/// - type [`UP_DOWN_SCROLL_LISTBOX`]
/// - asset ([`String`])
/// - frame_id ([`usize`])
/// - hover_frame_id ([`usize`])
/// - click_frame_id ([`usize`])
/// - options ([`Vec<String>`])
///
/// ## Optional
/// - position ([`UIPosition`](crate::utils::positioning::UIPosition))
/// - padding([`UIPosition`])
/// - scale ([`f32`])
/// - event_id ([`u16`])
/// - sync_id ([`u16`])
/// - number_of_buttons ([`usize`])
/// - font_size([`u32`])
/// - color([`Color`])
pub fn up_down_scroll_listbox_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<UpDownScrollListBox, Box<dyn Error>> {
    Ok(
        UpDownScrollListBox::new(
            resource_manager,& get_asset_id(minidom_element)?, get_ui_position(minidom_element).unwrap_or_default(),
            get_generic_attribute::<usize>(minidom_element, "frame_id").ok_or("ui::parse::loader::button_loader::tiling_text_button_loader: Unable to parse frame_id")?,
            get_generic_attribute::<usize>(minidom_element, "hover_frame_id").ok_or("ui::parse::loader::button_loader::tiling_text_button_loader: Unable to parse hover_frame_id")?,
            get_generic_attribute::<usize>(minidom_element, "click_frame_id").ok_or("ui::parse::loader::button_loader::tiling_text_button_loader: Unable to parse click_frame_id")?,
            minidom_element.attr("options").ok_or("No options provided")?.split(',').map(|v| v.to_string()).collect::<Vec<_>>(),
            get_generic_attribute::<usize>(minidom_element, "number_of_buttons").unwrap_or(1),
            get_generic_attribute::<UIPosition>(minidom_element, "padding"),
            get_scale(minidom_element).unwrap_or(default_scale),
            get_font_size(minidom_element).unwrap_or(default_font_size),
            get_color_attribute(minidom_element).unwrap_or(default_color),
            get_event_id_or_default(minidom_element),
            get_sync_id_or_default(minidom_element)
        )
    )
}

const UP_DOWN_SCROLL_LISTBOX: &str = "UpDownScrollListbox";
const LISTBOX_STYLES: [&str; 1] = [UP_DOWN_SCROLL_LISTBOX];
pub fn listbox_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Box<dyn ListBox>, Box<dyn Error>> {
    match minidom_element.attr("type") {
        Some(UP_DOWN_SCROLL_LISTBOX) => Ok(Box::new(up_down_scroll_listbox_loader(
            resource_manager,
            minidom_element,
            default_scale,
            default_font_size,
            default_color,
        )?)),
        string => Err(Box::new(SimpleError::new(format!(
            "Unable to parse type: {:#?} for valid listbox type. List of valid listbox types: {:#?}",
            string,
            LISTBOX_STYLES
        ))))
    }
}
