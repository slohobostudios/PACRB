use crate::{
    assets::resource_manager::ResourceManager,
    ui::elements::{root_node::RootNode, traits::Element as ElementTrait},
};
use element_loader::*;
use sfml::graphics::{Color, IntRect};
use std::error::Error;
use tracing::error;
use utils::*;

mod background_loader;
mod button_loader;
mod element_loader;
mod grid_loader;
mod missing_texture_loader;
mod slider_loader;
mod text_loader;
mod utils;

/// This function is how loading ui elements from an xml document will work. Returns empty document on failure.
///
/// # Usage
///
/// ```no_run
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
        try_page_loader(&resource_manager, relative_rect, xml_doc).unwrap_or_else(|e| {
            error!("Error loading page: {:#?}", e);
            RootNode::new(resource_manager, Vec::new(), relative_rect)
        });

    root_node.update_size();
    root_node.update_position(relative_rect);

    root_node
}

fn try_page_loader(
    resource_manager: &ResourceManager,
    relative_rect: IntRect,
    xml_doc: &str,
) -> Result<RootNode, Box<dyn Error>> {
    let root_node = xml_doc.parse()?;
    let default_scale = get_scale_or_default(&root_node, 4.);
    let default_font_size = get_font_size_or_default(&root_node, 16);
    let default_color = get_color_attribute_or_default(&root_node, Color::TRANSPARENT);

    let root_elements = root_node
        .children()
        .map(|child_node| {
            element_loader(
                &resource_manager,
                &child_node,
                default_scale,
                default_font_size,
                default_color,
            )
        })
        .collect();

    Ok(RootNode::new(
        &resource_manager,
        root_elements,
        relative_rect,
    ))
}
