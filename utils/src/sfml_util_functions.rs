#[macro_export]
macro_rules! center_of_rect {
    ( $type:ty, $rect:expr ) => {
        sfml::system::Vector2 {
            x: $rect.left + $rect.width / 2 as $type,
            y: $rect.top + $rect.height / 2 as $type,
        }
    };
}

#[macro_export]
macro_rules! vector_to_rect_with_zeroed_origin {
    ( $type:ty, $vector:expr ) => {
        Rect {
            top: 0 as $type,
            left: 0 as $type,
            width: $vector.x,
            height: $vector.y,
        }
    };
}

use super::{simple_error::SimpleError, string_util_functions::get_tuple_list_from_string};
use sfml::{
    graphics::{Color, Rect},
    system::{Vector2i, Vector2u},
};
use std::{error::Error, ops::Add, str::FromStr};
use tracing::error;

pub fn try_from_color_hash_owned_string_to_sfml_color(s: String) -> Result<Color, Box<dyn Error>> {
    try_from_color_hash_string_to_sfml_color(&s)
}

// Converts hash #FFFFFF or #FFFFFFFF to SFML::Color
pub fn try_from_color_hash_string_to_sfml_color(hex: &str) -> Result<Color, Box<dyn Error>> {
    let error = || SimpleError::new(format!("hex string is not valid: {hex}"));
    let digits = hex.trim().strip_prefix('#').ok_or_else(error)?;
    let mut iter = digits.chars().map(|c| c.to_digit(16).map(|d| d as u8));
    let mut next_component = || Some(iter.next()?? << 4 | iter.next()??);
    let red = next_component().ok_or_else(error)?;
    let green = next_component().ok_or_else(error)?;
    let blue = next_component().ok_or_else(error)?;
    match digits.len() {
        6 => Ok(Color::rgb(red, green, blue)),
        8 => {
            let alpha = next_component().ok_or_else(error)?;
            Ok(Color::rgba(red, green, blue, alpha))
        }
        _ => Err(error().into()),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn parse_color_hex() {
        // rgb tests
        assert_eq!(
            try_from_color_hash_string_to_sfml_color("#FFFFFF").unwrap(),
            Color::rgb(255, 255, 255)
        );
        assert_eq!(
            try_from_color_hash_string_to_sfml_color("#000000").unwrap(),
            Color::rgb(0, 0, 0)
        );
        assert_eq!(
            try_from_color_hash_string_to_sfml_color("#3780B2").unwrap(),
            Color::rgb(55, 128, 178)
        );
        // rgba tests
        assert_eq!(
            try_from_color_hash_string_to_sfml_color("#FFFFFFFF").unwrap(),
            Color::rgba(255, 255, 255, 255)
        );
        assert_eq!(
            try_from_color_hash_string_to_sfml_color("#00000000").unwrap(),
            Color::rgba(0, 0, 0, 0)
        );
        assert_eq!(
            try_from_color_hash_string_to_sfml_color("#3780B2A1").unwrap(),
            Color::rgba(55, 128, 178, 161)
        );
    }
}
use sfml::{graphics::Vertex, system::Vector2};

use sfml::graphics::Transform;
/// Returns new vertex array with the applied transformation
pub fn get_vertex_array_with_applied_transformation(
    vertex_array: &Vec<Vertex>,
    tf: Transform,
) -> Vec<Vertex> {
    let mut vertex_array = vertex_array.clone();
    for vertex in &mut vertex_array {
        vertex.position = tf.transform_point(vertex.position);
    }
    vertex_array
}

#[track_caller]
pub fn vector2i_from_vector2u(vector: Vector2u) -> Vector2i {
    vector.try_into_other().unwrap_or_else(|err| {
        error!("{}", err);
        let i32_x = i32::try_from(vector.x).unwrap_or(i32::MAX);
        let i32_y = i32::try_from(vector.y).unwrap_or(i32::MAX);
        Vector2i::new(i32_x, i32_y)
    })
}

pub fn vector2u_from_vector2i(vector: Vector2i) -> Vector2u {
    vector.try_into_other().unwrap_or_else(|err| {
        error!("{}", err);
        let u32_x = u32::try_from(vector.x).unwrap_or(u32::MIN);
        let u32_y = u32::try_from(vector.y).unwrap_or(u32::MAX);
        Vector2u::new(u32_x, u32_y)
    })
}

use serde::ser::StdError;
#[track_caller]
pub fn vector2_from_str<T: std::default::Default + FromStr>(
    string: &str,
) -> Result<Vector2<T>, Box<dyn Error>>
where
    <T as FromStr>::Err: 'static + StdError,
{
    let mut vec: Vector2<T> = Default::default();
    for string_tuple in get_tuple_list_from_string(string) {
        let (val, amt) = string_tuple?;

        let amt = amt.parse::<T>()?;

        match val.to_lowercase().as_str() {
            "x" => vec.x = amt,
            "y" => vec.y = amt,
            _ => {
                return Err(Box::new(SimpleError::new(
                    "val: {} is not an attribute in sfml::system::Vector2".to_string(),
                )))
            }
        }
    }

    Ok(vec)
}

#[track_caller]
pub fn color_from_str(s: &str) -> Result<Color, Box<dyn Error>> {
    if s.contains(":") {
        let mut color: Color = Default::default();
        for tuple in get_tuple_list_from_string(s) {
            let Ok((s, val)) = tuple else {
                error!("Failed to parse color from element. {:#?}", tuple);
                return Ok(Default::default());
            };

            let Ok(val) = val.parse::<u8>() else {
                error!("Failed to parse color value in get_color_or_default: {}", val);
                return Ok(Default::default());
            };

            match s.to_lowercase().as_str() {
                "r" => color.r = val,
                "g" => color.g = val,
                "b" => color.b = val,
                "a" => color.a = val,
                _ => {
                    error!("Invalid color value: {}", s);
                    return Ok(Default::default());
                }
            }
        }

        Ok(color)
    } else if s.contains("#") {
        Ok(
            try_from_color_hash_string_to_sfml_color(s).unwrap_or_else(|err| {
                error!("{:#?}", err);
                Default::default()
            }),
        )
    } else {
        Ok(match s.to_uppercase().as_str() {
            "BLACK" => Color::BLACK,
            "WHITE" => Color::WHITE,
            "RED" => Color::RED,
            "GREEN" => Color::GREEN,
            "BLUE" => Color::BLUE,
            "YELLOW" => Color::YELLOW,
            "MAGENTA" => Color::MAGENTA,
            "CYAN" => Color::CYAN,
            "TRANSPARENT" | _ => Color::TRANSPARENT,
        })
    }
}

pub fn bottom_right_rect_coords<T: Add<Output = T>>(rect: Rect<T>) -> Vector2<T> {
    Vector2 {
        x: rect.left + rect.width,
        y: rect.top + rect.height,
    }
}

pub fn bottom_left_rect_coords<T: Add<Output = T>>(rect: Rect<T>) -> Vector2<T> {
    Vector2 {
        x: rect.left,
        y: rect.top + rect.height,
    }
}

pub fn top_right_rect_coords<T: Add<Output = T>>(rect: Rect<T>) -> Vector2<T> {
    Vector2 {
        x: rect.left + rect.width,
        y: rect.top,
    }
}
