use crate::{elements::Element as UIElement, utils::positioning::UIPosition};
use minidom::Element;
use serde::ser::StdError;
use sfml::{graphics::Color, system::Vector2};
use std::{error::Error, str::FromStr};
use utils::{
    resource_manager::ResourceManager,
    sfml_util_functions::{color_from_str, vector2_from_str},
};

use super::element_loader;

pub fn get_generic_attribute<T: std::default::Default + std::str::FromStr>(
    ele: &Element,
    attr: &str,
) -> Option<T> {
    Some(ele.attr(attr)?.parse::<T>().unwrap_or_default())
}

pub fn empty_element() -> Element {
    Element::builder("Empty", "QuestHearth").build()
}

pub fn get_asset_id(ele: &Element) -> Result<String, Box<dyn Error>> {
    Ok(ele.attr("asset").ok_or("No asset defined")?.to_string())
}

pub fn get_scale(ele: &Element) -> Result<f32, Box<dyn Error>> {
    Ok(ele.attr("scale").unwrap_or("not_number").parse::<f32>()?)
}

pub fn get_ui_position(ele: &Element) -> Result<UIPosition, Box<dyn Error>> {
    Ok(UIPosition::from_str(
        ele.attr("position").ok_or("Position undefined")?,
    )?)
}

pub fn get_font_size(ele: &Element) -> Result<u32, Box<dyn Error>> {
    Ok(ele
        .attr("font_size")
        .unwrap_or("Not Number")
        .parse::<u32>()?)
}

#[track_caller]
pub fn get_color_attribute(ele: &Element) -> Result<Color, Box<dyn Error>> {
    color_from_str(
        ele.attr("color").ok_or("No color attribute")?,
    )
}

pub fn get_size<T: std::default::Default + FromStr>(
    ele: &Element,
) -> Result<Vector2<T>, Box<dyn Error>>
where
    <T as FromStr>::Err: 'static + StdError,
{
    vector2_from_str::<T>(
        ele.attr("size").ok_or("No size attribute")?,
    )
}

pub fn get_event_id_or_default(ele: &Element) -> u16 {
    ele.attr("event_id")
        .unwrap_or("0")
        .parse::<u16>()
        .unwrap_or_default()
}

pub fn get_sync_id_or_default(ele: &Element) -> u16 {
    ele.attr("sync_id")
        .unwrap_or("0")
        .parse::<u16>()
        .unwrap_or_default()
}

pub fn collect_children_as_vector(
    resource_manager: &ResourceManager,
    ele: &Element,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Vec<UIElement> {
    ele.children()
        .map(|child_node| {
            element_loader(
                resource_manager,
                child_node,
                default_scale,
                default_font_size,
                default_color,
            )
        })
        .collect()
}
