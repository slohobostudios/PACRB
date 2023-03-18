use super::{element_loader::element_loader, utils::*};
use crate::elements::button::{
    boolean_image_button::BooleanImageButton, image_button::ImageButton,
    tiling_text_button::TilingButton, traits::Button,
};
use minidom::Element as MinidomElement;
use sfml::{graphics::Color, system::Vector2f};
use std::error::Error;
use tracing::error;
use utils::{
    resource_manager::ResourceManager, sfml_util_functions::vector2_from_str,
    simple_error::SimpleError,
};

/// # Usage
///
/// ## Required:
/// - type: [`IMAGE_BUTTON`]
/// - asset ([`String`])
/// - frame_id ([`usize`])
/// - hover_frame_id ([`usize`])
/// - click_frame_id ([`usize`])
///
/// ## Optional:
/// - position ([`UIPosition`])
/// - scale ([`f32`])
/// - event_id ([`u16`])
/// - sync_id ([`u16`])
fn image_button_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
) -> Result<ImageButton, Box<dyn Error>> {
    Ok(ImageButton::new(
        resource_manager,
        get_ui_position(&minidom_element).unwrap_or_default(),
        &get_asset_id(&minidom_element)?,
        minidom_element
            .attr("frame_id")
            .ok_or("No frame_id defined")?
            .parse::<usize>()?,
        minidom_element
            .attr("hover_frame_id")
            .ok_or("no hover_frame_id defined")?
            .parse::<usize>()?,
        minidom_element
            .attr("click_frame_id")
            .ok_or("no click_frame_id defined")?
            .parse::<usize>()?,
        get_scale(&minidom_element).unwrap_or(default_scale),
        get_event_id_or_default(&minidom_element),
        get_sync_id_or_default(&minidom_element),
    ))
}

/// # Usage
///
/// ## Required:
/// - type [`TILING_BUTTON`]
/// - asset ([`String`])
/// - frame_id ([`usize`])
/// - hover_frame_id ([`usize`])
/// - click_frame_id ([`usize`])
///
/// ## Optional:
/// - position ([`UIPosition`])
/// - scale ([`f32`])
/// - event_id ([`u16`])
/// - sync_id ([`u16`])
/// - size ([`Vector2f`])
///
/// ## Children:
/// Any dom node
fn tiling_button_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<TilingButton, Box<dyn Error>> {
    Ok(TilingButton::new(
        resource_manager,
        get_ui_position(&minidom_element).unwrap_or_default(),
        &get_asset_id(&minidom_element)?,
        get_generic_attribute::<usize>(&minidom_element, "frame_id").ok_or("ui::parse::loader::button_loader::tiling_text_button_loader: Unable to parse frame_id")?,
        get_generic_attribute::<usize>(&minidom_element, "hover_frame_id").ok_or("ui::parse::loader::button_loader::tiling_text_button_loader: Unable to parse hover_frame_id")?,
        get_generic_attribute::<usize>(&minidom_element, "click_frame_id").ok_or("ui::parse::loader::button_loader::tiling_text_button_loader: Unable to parse click_frame_id")?,
        element_loader(resource_manager, minidom_element.children().nth(0).ok_or("ui::parse::loader::button_loader::tiling_text_button_loader: No child element for TilingButton to wrap")?, default_scale, default_font_size, default_color)
        ,
        &vector2_from_str(&minidom_element.attr("size").unwrap_or("x:1,y:1"))
            .unwrap_or_else(|e| {
                error!(
                    "{:#?}",
                    e
                );
                Vector2f::new(1., 1.)
            })
            .as_other(),
        get_scale(&minidom_element).unwrap_or(default_scale),
        get_event_id_or_default(&minidom_element),
        get_sync_id_or_default(&minidom_element),
    ))
}

/// # Usage
///
/// ## Required:
/// - type [`BOOLEAN_IMAGE_BUTTON`]
/// - asset ([`String`])
/// - truth_frame_id ([`usize`])
/// - truth_hover_frame_id ([`usize`])
/// - truth_click_frame_id ([`usize`])
/// - false_frame_id ([`usize`])
/// - false_hover_frame_id ([`usize`])
/// - false_click_frame_id ([`usize`])
///
/// ## Optional:
/// - position ([`UIPosition`])
/// - scale ([`f32`])
/// - event_id ([`u16`])
/// - sync_id ([`u16`])
fn boolean_image_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
) -> Result<BooleanImageButton, Box<dyn Error>> {
    Ok(BooleanImageButton::new(
        resource_manager,
        get_ui_position(&minidom_element).unwrap_or_default(),
        get_scale(&minidom_element).unwrap_or(default_scale),
        false,
        &get_asset_id(&minidom_element)?,
        get_generic_attribute::<usize>(&minidom_element, "truth_frame_id").ok_or(
            "ui::pages::loader:button_loader::boolean_image_loader: Unable to parse truth_frame_id",
        )?,
        get_generic_attribute::<usize>(&minidom_element, "truth_hover_frame_id").ok_or(
            "ui::pages::loader:button_loader::boolean_image_loader: Unable to parse truth_hover_frame_id",
        )?,
        get_generic_attribute::<usize>(&minidom_element, "truth_click_frame_id").ok_or(
            "ui::pages::loader:button_loader::boolean_image_loader: Unable to parse truth_click_frame_id",
        )?,
        get_generic_attribute::<usize>(&minidom_element, "false_frame_id").ok_or(
            "ui::pages::loader:button_loader::boolean_image_loader: Unable to parse false_frame_id",
        )?,
        get_generic_attribute::<usize>(&minidom_element, "false_hover_frame_id").ok_or(
            "ui::pages::loader:button_loader::boolean_image_loader: Unable to parse false_hover_frame_id",
        )?,
        get_generic_attribute::<usize>(&minidom_element, "false_click_frame_id").ok_or(
            "ui::pages::loader:button_loader::boolean_image_loader: Unable to parse false_click_frame_id",
        )?,
        get_event_id_or_default(&minidom_element),
        get_sync_id_or_default(&minidom_element),
    ))
}

const TILING_BUTTON: &'static str = "TilingButton";
const IMAGE_BUTTON: &'static str = "ImageButton";
const BOOLEAN_IMAGE_BUTTON: &'static str = "BooleanImageButton";
const BUTTON_STYLES: [&'static str; 3] = [TILING_BUTTON, IMAGE_BUTTON, BOOLEAN_IMAGE_BUTTON];
pub fn button_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Box<dyn Button>, Box<dyn Error>> {
    match minidom_element.attr("type") {
        Some(TILING_BUTTON) => Ok(Box::new(tiling_button_loader(
            &resource_manager,
            &minidom_element,
            default_scale,
            default_font_size,
            default_color,
        )?)),
        Some(IMAGE_BUTTON) => Ok(Box::new(image_button_loader(
            &resource_manager,
            &minidom_element,
            default_scale,
        )?)),
        Some(BOOLEAN_IMAGE_BUTTON) => Ok(Box::new(boolean_image_loader(
            &resource_manager,
            &minidom_element,
            default_scale,
        )?)),
        string => Err(Box::new(SimpleError::new(format!(
            "Unable to parse type: {:#?} for valid button type. List of valid button types: {:#?}",
            string, BUTTON_STYLES
        )))),
    }
}
