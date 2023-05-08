use crate::elements::{root_node::RootNode, traits::Element as ElementTrait};
use ::utils::resource_manager::ResourceManager;
use element_loader::*;
use sfml::graphics::{Color, IntRect};
use std::error::Error;
use tracing::{error, warn};

use self::utils::{get_color_attribute, get_font_size, get_scale};

mod background_loader;
mod button_loader;
mod div_loader;
mod element_loader;
mod grid_loader;
mod image_loader;
mod missing_texture_loader;
mod sets_loader;
mod slider_loader;
mod text_loader;
mod textbox_loader;
mod utils;

/// This function is how loading ui elements from an xml document will work. Returns empty document on failure.
///
/// # Args
/// - resource_manager: ResourceManager,
/// - relative_rect: Typically the screen size
/// - xml_doc: &str representing the xml doc
///
/// # Usage
///
/// ```ignore
/// let xml_doc = r##"
/// <RootNode scale="4" font_size="24" color="#f7e5e4" xmlns="https://www.loc.gov/marc/marcxml.html">
///   <Background
///     type="Fixed3x3RepeatableBackground"
///     asset="dark_blue_background.png"
///     position="b:15,r:15"
///     size="x:200,y:175"
///     frame_id="0">
///   </Background>
/// </RootNode>"##;
///
/// let dom_controller = dom_loader(&resource_manager, screen_size, xml_doc);
/// ```
pub fn dom_loader(
    resource_manager: &ResourceManager,
    relative_rect: IntRect,
    xml_doc: &str,
) -> RootNode {
    let mut root_node =
        try_page_loader(resource_manager, relative_rect, xml_doc).unwrap_or_else(|e| {
            error!("Error loading page: {:#?}", e);
            RootNode::new(resource_manager, Vec::new(), relative_rect)
        });

    root_node.update_size();
    root_node.update_position(relative_rect);

    root_node
}

const DEFAULT_SCALE: f32 = 4.;
const DEFAULT_FONT_SIZE: u32 = 16;
const DEFAULT_COLOR: Color = Color::TRANSPARENT;
fn try_page_loader(
    resource_manager: &ResourceManager,
    relative_rect: IntRect,
    xml_doc: &str,
) -> Result<RootNode, Box<dyn Error>> {
    let root_node = xml_doc.parse()?;
    let default_scale = get_scale(&root_node).unwrap_or_else(|err| {
        warn!(
            "No default scale in root node! Exact error: {:?} Setting to {}",
            err, DEFAULT_SCALE
        );
        DEFAULT_SCALE
    });
    let default_font_size = get_font_size(&root_node).unwrap_or_else(|err| {
        warn!(
            "No default font size in root node! Exact error: {:?} Setting to {}",
            err, DEFAULT_FONT_SIZE
        );
        DEFAULT_FONT_SIZE
    });
    let default_color = get_color_attribute(&root_node).unwrap_or_else(|err| {
        warn!(
            "No default color in root node! Exact error: {:?} Setting to Color::TRANSPARENT",
            err
        );
        DEFAULT_COLOR
    });

    let root_elements = root_node
        .children()
        .map(|child_node| {
            element_loader(
                resource_manager,
                child_node,
                default_scale,
                default_font_size,
                default_color,
            )
        })
        .collect();

    Ok(RootNode::new(
        resource_manager,
        root_elements,
        relative_rect,
    ))
}
