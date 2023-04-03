use super::{element_loader::element_loader, utils::*};
use crate::elements::slider::{
    hue_color_picker::HueColorPicker,
    increment_decrement_pointer_slider::IncrementDecrementPointerSlider,
    quad_color_picker::QuadColorPicker, traits::Slider,
};
use minidom::Element as MinidomElement;
use sfml::graphics::Color;
use std::error::Error;
use tracing::warn;
use utils::{
    resource_manager::ResourceManager, sfml_util_functions::color_from_str,
    simple_error::SimpleError,
};

/// # Usage
///
/// ## Required
/// - type [`INCREMENT_DECREMENT_POINTER_SLIDER`]
/// - asset ([`String`])
/// - frame_id ([`usize`])
/// - hover_frame_id ([`usize`])
/// - click_frame_id ([`usize`])
/// - min ([`f32`])
/// - max ([`f32`])
/// - increment ([`f32`])
///
/// ## Optional
/// - sync_id ([`u16`])
/// - event_id ([`u16`])
/// - size ([`Vector2`])
/// - color ([`Color`])
/// - font_size ([`u32`])
/// - scale ([`f32`])
/// - position ([`UIPosition`])
fn increment_decrement_pointer_slider_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<IncrementDecrementPointerSlider, Box<dyn Error>> {
    Ok(IncrementDecrementPointerSlider::new(
        resource_manager,
        get_ui_position(minidom_element)?,
        get_scale(minidom_element).unwrap_or( default_scale),
        &get_asset_id(minidom_element)?,
        get_font_size(minidom_element).unwrap_or( default_font_size),
        get_color_attribute(minidom_element).unwrap_or(default_color),
        get_generic_attribute(minidom_element,"frame_id").ok_or("ui::dom_loader::slider_loader::increment_decrement_pointer_slider_loader: Failed to parse frame_id")?,
        get_generic_attribute(minidom_element,"hover_frame_id").ok_or("ui::dom_loader::slider_loader::increment_decrement_pointer_slider_loader: Failed to parse hover_frame_id")?,
        get_generic_attribute(minidom_element,"click_frame_id").ok_or("ui::dom_loader::slider_loader::increment_decrement_pointer_slider_loader: Failed to parse click_frame_id")?,
        get_size(minidom_element).unwrap_or_default().x,
        (
            get_generic_attribute::<f32>(minidom_element, "min").ok_or("ui::dom_loader::slider_loader::increment_decrement_pointer_slider_loader: Failed to parse min")?,
            get_generic_attribute::<f32>(minidom_element, "max").ok_or("ui::dom_loader::slider_loader::increment_decrement_pointer_slider_loader: Failed to parse max")?
        ),
        get_generic_attribute::<f32>(minidom_element, "increment").ok_or("ui::dom_loader::slider_loader::increment_decrement_pointer_slider_loader: Failed to parse increment")?,
        get_event_id_or_default(minidom_element),
        get_sync_id_or_default(minidom_element),

    ))
}

/// # Usage
///
/// ## Required:
/// - type [`QUAD_COLOR_PICKER`]
/// - top_left_color ([`Color`])
/// - top_right_color ([`Color`])
/// - bottom_right_color ([`Color`])
/// - bottom_left_color)
///
/// ## Optional:
/// - position ([`UIPosition`])
/// - scale ([`f32`])
/// - event_id ([`u16`])
/// - sync_id ([`u16`])
/// - size ([`Vector2f`])
fn quad_color_picker_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<QuadColorPicker, Box<dyn Error>> {
    let size = get_size(minidom_element).unwrap_or_default();
    if size == Default::default() {
        warn!("QuadColorPicker size is 0,0. Displaying nothing")
    }
    Ok(QuadColorPicker::new(
        element_loader(
            resource_manager,
            minidom_element
                .children().next()
                .ok_or("Inner slider icon element not found")?,
            get_scale(minidom_element).unwrap_or(default_scale),
            get_font_size(minidom_element).unwrap_or(default_font_size),
            get_color_attribute(minidom_element).unwrap_or(default_color),
        ),
        get_ui_position(minidom_element).unwrap_or_default(),
        size,
        color_from_str(minidom_element.attr("top_left_color").unwrap_or_default())
            .unwrap_or_default(),
        color_from_str(minidom_element.attr("top_right_color").unwrap_or_default())
            .unwrap_or_default(),
        color_from_str(
            minidom_element
                .attr("bottom_right_color")
                .unwrap_or_default(),
        )
        .unwrap_or_default(),
        color_from_str(
            minidom_element
                .attr("bottom_left_color")
                .unwrap_or_default(),
        )
        .unwrap_or_default(),
        get_event_id_or_default(minidom_element),
        get_sync_id_or_default(minidom_element),
    ))
}

/// # Usage
///
/// ## Required:
/// - type: [`HUE_COLOR_PICKER`]
/// - *CHILD_ELEMENT*
///
/// ## Optional:
/// - position ([`UIPosition`])
/// - size ([`Vector2`])
/// - font_size ([`u32`])
/// - event_id ([`u16`])
/// - sync_id ([`u16`])
/// - color ([`Color`])
pub fn hue_color_picker_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<HueColorPicker, Box<dyn Error>> {
    Ok(HueColorPicker::new(
        element_loader(
            resource_manager,
            minidom_element
                .children().next()
                .ok_or("Inner slider icon element not found")?,
            get_scale(minidom_element).unwrap_or(default_scale),
            get_font_size(minidom_element).unwrap_or(default_font_size),
            get_color_attribute(minidom_element).unwrap_or(default_color),
        ),
        get_ui_position(minidom_element).unwrap_or_default(),
        get_size(minidom_element).unwrap_or_default(),
        get_event_id_or_default(minidom_element),
        get_sync_id_or_default(minidom_element),
    ))
}

const INCREMENT_DECREMENT_POINTER_SLIDER: &str = "IncrementPointerSlider";
const QUAD_COLOR_PICKER: &str = "QuadColorPicker";
const HUE_COLOR_PICKER: &str = "HueColorPicker";
const SLIDER_STYLES: [&str; 3] = [
    INCREMENT_DECREMENT_POINTER_SLIDER,
    QUAD_COLOR_PICKER,
    HUE_COLOR_PICKER,
];
pub fn slider_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Box<dyn Slider>, Box<dyn Error>> {
    match minidom_element.attr("type") {
        Some(INCREMENT_DECREMENT_POINTER_SLIDER) => {
            Ok(Box::new(increment_decrement_pointer_slider_loader(
                resource_manager,
                minidom_element,
                default_scale,
                default_font_size,
                default_color,
            )?))
        }
        Some(QUAD_COLOR_PICKER) => Ok(Box::new(quad_color_picker_loader(
            resource_manager,
            minidom_element,
            default_scale,
            default_font_size,
            default_color,
        )?)),
        Some(HUE_COLOR_PICKER) => Ok(Box::new(hue_color_picker_loader(
            resource_manager,
            minidom_element,
            default_scale,
            default_font_size,
            default_color,
        )?)),
        string => Err(Box::new(SimpleError::new(format!(
            "Unable to parse type: {:#?} for valid slider type. List of valid slider types {:#?}",
            string, SLIDER_STYLES
        )))),
    }
}
