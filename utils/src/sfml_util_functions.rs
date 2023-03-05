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

// Converts hash #FFFFFF or #FFFFFFFF to SFML::Color
pub fn try_from_color_hash_string_to_sfml_color(
    color_hash: &str,
) -> Result<Color, Box<(dyn Error + 'static)>> {
    let color_hash = if color_hash.chars().next() == Some('#') {
        color_hash
            .chars()
            .skip(1)
            .take(color_hash.len() - 1)
            .collect::<String>()
    } else {
        color_hash.to_string()
    }
    .trim()
    .to_string();

    if color_hash.len() != 8 && color_hash.len() != 6 {
        return Err(Box::new(SimpleError::new(format!(
            "Hex string {:#?} is not 8 or 6 characters long!",
            color_hash
        ))));
    }

    let color_hash = if color_hash.len() == 8 {
        color_hash
    } else {
        color_hash + "ff"
    };

    Ok(Color::from(u32::from_str_radix(&color_hash, 16)?))
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
