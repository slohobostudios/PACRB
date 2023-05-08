use minidom::Element as MinidomElement;
use sfml::graphics::Color;
use utils::resource_manager::ResourceManager;

use crate::{elements::grouping::sets::Sets, utils::positioning::UIPosition};

use super::utils::{
    collect_children_as_vector, get_generic_attribute, get_size, get_sync_id_or_default,
    get_ui_position,
};

/// # Usage
///
/// ## Required:
///
/// ## Optional:
/// - position ([`UIPosition`])
/// - padding ([`UIPosition`])
/// - scale ([`f32`])
/// - font_size ([`u32`])
/// - color ([`Color`])
pub fn sets_loader(
    resource_manager: &ResourceManager,
    minidom_element: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Sets {
    let elements = collect_children_as_vector(
        resource_manager,
        minidom_element,
        default_scale,
        default_font_size,
        default_color,
    );
    let mut sets = vec![];
    for element in elements {
        sets.push(vec![element]);
    }
    Sets::new(
        get_ui_position(minidom_element).unwrap_or_default(),
        sets,
        get_generic_attribute::<UIPosition>(minidom_element, "padding"),
        get_size(minidom_element).ok(),
        get_sync_id_or_default(minidom_element),
    )
}
