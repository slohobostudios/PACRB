use std::error::Error;

use minidom::Element;
use sfml::graphics::PrimitiveType;
use utils::simple_error::SimpleError;

use crate::elements::misc::primitive::Primitive;

use super::utils::{get_color_attribute, get_ui_position};

// Documentation imports
#[allow(unused_imports)]
use crate::utils::positioning::UIPosition;
#[allow(unused_imports)]
use sfml::graphics::Color;
#[allow(unused_imports)]
use utils::sfml_util_functions::vertex_array_from_string;

/// # Usage
///
/// ## Required:
/// - type ([`FILL`])
///
/// ## Optional:
/// - color ([`Color`])
fn fill_loader(ele: &Element) -> Primitive {
    Primitive::new(
        None,
        vec![],
        get_color_attribute(ele).unwrap_or_default(),
        PrimitiveType::TRIANGLE_FAN,
    )
}

/// # Usage
///
/// ## Required:
/// - type ([`POINTS`])
/// - vertices ([`String`]) see ([`vertex_array_from_string`]) for more details
///
/// ## Option:
/// - color ([`Color`])
/// - position ([`UIPosition`])
fn points_loader(ele: &Element) -> Primitive {
    Primitive::with_vertex_string(
        get_ui_position(ele).ok(),
        ele.attr("vertices").unwrap_or_default(),
        get_color_attribute(ele).unwrap_or_default(),
        PrimitiveType::POINTS,
    )
}

/// # Usage
///
/// ## Required:
/// - type ([`LINES`])
/// - vertices ([`String`]) see ([`vertex_array_from_string`]) for more details
///
/// ## Option:
/// - color ([`Color`])
/// - position ([`UIPosition`])
fn lines_loader(ele: &Element) -> Primitive {
    Primitive::with_vertex_string(
        get_ui_position(ele).ok(),
        ele.attr("vertices").unwrap_or_default(),
        get_color_attribute(ele).unwrap_or_default(),
        PrimitiveType::LINES,
    )
}

/// # Usage
///
/// ## Required:
/// - type ([`LINE_STRIP`])
/// - vertices ([`String`]) see ([`vertex_array_from_string`]) for more details
///
/// ## Option:
/// - color ([`Color`])
/// - position ([`UIPosition`])
fn line_strip_loader(ele: &Element) -> Primitive {
    Primitive::with_vertex_string(
        get_ui_position(ele).ok(),
        ele.attr("vertices").unwrap_or_default(),
        get_color_attribute(ele).unwrap_or_default(),
        PrimitiveType::LINE_STRIP,
    )
}

/// # Usage
///
/// ## Required:
/// - type ([`TRIANGLES`])
/// - vertices ([`String`]) see ([`vertex_array_from_string`]) for more details
///
/// ## Optional:
/// - color ([`Color`])
/// - position ([`UIPosition`])
fn triangles_loader(ele: &Element) -> Primitive {
    Primitive::with_vertex_string(
        get_ui_position(ele).ok(),
        ele.attr("vertices").unwrap_or_default(),
        get_color_attribute(ele).unwrap_or_default(),
        PrimitiveType::TRIANGLES,
    )
}

/// # Usage
///
/// ## Required:
/// - type ([`TRIANGLE_STRIP`])
/// - vertices ([`String`]) see ([`vertex_array_from_string`]) for more details
///
/// ## Optional:
/// - color ([`Color`])
/// - position ([`UIPosition`])
fn triangle_strip_loader(ele: &Element) -> Primitive {
    Primitive::with_vertex_string(
        get_ui_position(ele).ok(),
        ele.attr("vertices").unwrap_or_default(),
        get_color_attribute(ele).unwrap_or_default(),
        PrimitiveType::TRIANGLE_STRIP,
    )
}

/// # Usage
///
/// ## Required:
/// - type ([`TRIANGLE_FAN`])
/// - vertices ([`String`]) see ([`vertex_array_from_string`]) for more details
///
/// ## Optional:
/// - color ([`Color`])
/// - position ([`UIPosition`])
fn triangle_fan_loader(ele: &Element) -> Primitive {
    Primitive::with_vertex_string(
        get_ui_position(ele).ok(),
        ele.attr("vertices").unwrap_or_default(),
        get_color_attribute(ele).unwrap_or_default(),
        PrimitiveType::TRIANGLE_FAN,
    )
}

const FILL: &str = "Fill";
const POINTS: &str = "Points";
const LINES: &str = "Lines";
const LINE_STRIP: &str = "LineStrip";
const TRIANGLES: &str = "Triangles";
const TRIANGLE_STRIP: &str = "TriangleStrip";
const TRIANGLE_FAN: &str = "TriangleFan";
const PRIMITIVE_TYPES: [&str; 7] = [
    FILL,
    POINTS,
    LINES,
    LINE_STRIP,
    TRIANGLES,
    TRIANGLE_STRIP,
    TRIANGLE_FAN,
];
pub fn primitive_loader(ele: &Element) -> Result<Primitive, Box<dyn Error>> {
    match ele.attr("type") {
        Some(FILL) => Ok(fill_loader(ele)),
        Some(POINTS) => Ok(points_loader(ele)),
        Some(LINES) => Ok(lines_loader(ele)),
        Some(LINE_STRIP) => Ok(line_strip_loader(ele)),
        Some(TRIANGLES) => Ok(triangles_loader(ele)),
        Some(TRIANGLE_STRIP) => Ok(triangle_strip_loader(ele)),
        Some(TRIANGLE_FAN) => Ok(triangle_fan_loader(ele)),
        string => Err(Box::new(SimpleError::new(format!(
            "Unable to parse type: {:#?} for valid primitive type. List of viable primitive types: {:#?}", string, PRIMITIVE_TYPES
        ))))
    }
}
