use crate::elements::{grouping::grid::Grid, Element};

use super::{element_loader::element_loader, utils::*};
use minidom::Element as MinidomElement;
use sfml::{graphics::Color, system::Vector2};
use std::error::Error;
use tracing::error;
use utils::{
    resource_manager::ResourceManager, sfml_util_functions::vector2_from_str,
    simple_error::SimpleError,
};

fn grid_elements_loader(
    resource_manager: &ResourceManager,
    ele: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Vec<Vec<Element>>, Box<dyn Error>> {
    let children: Vec<&MinidomElement> = ele.children().collect();
    let sqrt_num_of_children = (children.len() as f32).sqrt().ceil();
    let grid_layout: Vector2<usize> = vector2_from_str(
        ele.attr("grid_layout")
            .unwrap_or(format!("x:{},y:{}", sqrt_num_of_children, sqrt_num_of_children).as_str()),
    )
    .unwrap_or_else(|e| {
        error!("Error parsing vector2 from string. {:#?}", e);
        Vector2::new(sqrt_num_of_children, sqrt_num_of_children)
    })
    .as_other();

    if grid_layout.x * grid_layout.y < children.len() {
        return Err(Box::new(SimpleError::new(format!(
            "There are more grid elements ({}) than the grid_layout ({},{}) will allow",
            children.len(),
            grid_layout.x,
            grid_layout.y
        ))));
    }

    let mut grid: Vec<Vec<Element>> = Vec::new();
    for x in 0..grid_layout.x {
        grid.push(Vec::new());
        for _ in 0..grid_layout.y {
            grid[x].push(Element::Empty);
        }
        grid[x].shrink_to_fit();
    }
    grid.shrink_to_fit();

    #[allow(clippy::needless_range_loop)]
    for x in 0..grid_layout.x {
        for y in 0..grid_layout.y {
            grid[x][y] = element_loader(
                resource_manager,
                children
                    .get(y * grid_layout.x + x)
                    .unwrap_or(&&empty_element()),
                default_scale,
                default_font_size,
                default_color,
            );
        }
    }

    Ok(grid)
}

/// # Usage
///
/// ## Optional
/// - size ([`Vector2`](sfml::system::Vector2))
/// - position ([`UIPosition`](crate::utils::positioning::UIPosition))
/// - pagination_size ([`Vector2`](sfml::system::Vector2))
pub fn grid_loader(
    resource_manager: &ResourceManager,
    ele: &MinidomElement,
    default_scale: f32,
    default_font_size: u32,
    default_color: Color,
) -> Result<Grid, Box<dyn Error>> {
    let default_scale = get_scale(ele).unwrap_or(default_scale);
    let default_font_size = get_font_size(ele).unwrap_or(default_font_size);
    let default_color = get_color_attribute(ele).unwrap_or(default_color);

    Ok(Grid::new(
        resource_manager,
        grid_elements_loader(
            resource_manager,
            ele,
            default_scale,
            default_font_size,
            default_color,
        )?,
        get_ui_position(ele).unwrap_or_default(),
        vector2_from_str::<u16>(ele.attr("pagination_size").unwrap_or_default())
            .unwrap_or_default(),
        get_size(ele).unwrap_or_default(),
    ))
}
