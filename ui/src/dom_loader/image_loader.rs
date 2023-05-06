use std::error::Error;

use minidom::Element;
use utils::{resource_manager::ResourceManager, simple_error::SimpleError};

use crate::elements::image::Image;

use super::utils::{get_scale, get_ui_position};

const ICONS_HASH: &str = "icons.png";
/// # Usage
///
/// ## Required:
/// - type ([`ICON`])
/// - name ([`String`])
pub fn icon_loader(
    resource_manager: &ResourceManager,
    ele: &Element,
    default_scale: f32,
) -> Result<Image, Box<dyn Error>> {
    let name = ele.attr("name").ok_or("no name defined")?;
    let asset = resource_manager.fetch_asset(ICONS_HASH);
    let frame_tag = &asset.fetch_frame_tag(name);
    Ok(Image::new(
        resource_manager,
        get_ui_position(ele).unwrap_or_default(),
        ICONS_HASH,
        usize::from(frame_tag.from),
        get_scale(ele).unwrap_or(default_scale),
    ))
}

const ICON: &str = "Icon";
const IMAGE_TYPES: [&str; 1] = [ICON];
pub fn image_loader(
    resource_manager: &ResourceManager,
    ele: &Element,
    default_scale: f32,
) -> Result<Image, Box<dyn Error>> {
    match ele.attr("type") {
        Some(ICON) => icon_loader(resource_manager, ele, default_scale),
        string => Err(Box::new(SimpleError::new(format!(
            "Unable to parse type: {:#?} for valid image type. List of viable image types: {:#?}",
            string, IMAGE_TYPES
        )))),
    }
}
