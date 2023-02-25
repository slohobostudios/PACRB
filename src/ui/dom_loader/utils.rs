use crate::{ui::utils::positioning::UIPosition, utils::sfml_util_functions::*};
use minidom::Element;
use serde::ser::StdError;
use sfml::{graphics::Color, system::Vector2};
use std::{error::Error, str::FromStr};

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

pub fn get_scale_or_default(ele: &Element, default_scale: f32) -> f32 {
    ele.attr("scale")
        .unwrap_or("not_number")
        .parse::<f32>()
        .unwrap_or(default_scale)
}

pub fn get_ui_position(ele: &Element) -> Result<UIPosition, Box<dyn Error>> {
    Ok(UIPosition::from_str(
        ele.attr("position").ok_or("Position undefined")?,
    )?)
}

pub fn get_font_size_or_default(ele: &Element, default_font_size: u32) -> u32 {
    ele.attr("font_size")
        .unwrap_or("Not Number")
        .parse::<u32>()
        .unwrap_or(default_font_size)
}

#[track_caller]
pub fn get_color_attribute_or_default(ele: &Element, default_color: Color) -> Color {
    let Some(color_string) = ele.attr("color") else {
        return default_color;
    };
    color_from_str(color_string).unwrap_or(default_color)
}

pub fn get_size_or_default<T: std::default::Default + FromStr>(
    ele: &Element,
    default_size: Vector2<T>,
) -> Vector2<T>
where
    <T as FromStr>::Err: 'static + StdError,
{
    vector2_from_str::<T>(ele.attr("size").unwrap_or(Default::default())).unwrap_or(default_size)
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
