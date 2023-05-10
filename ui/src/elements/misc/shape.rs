use sfml::graphics::{vertex_array_bounds, Color, IntRect, RenderStates, Vertex};
use utils::{resource_manager::ResourceManager, sfml_util_functions::vertex_array_from_string};

use crate::{
    elements::traits::{cast_element, Element},
    utils::positioning::UIPosition,
};

#[derive(Debug, Clone)]
pub struct Shape {
    global_bounds: IntRect,
    /// position is optional because of the "rectangle strategy".
    /// If both position and number of vertices is zero, we
    /// will create a rect that is the size of the object
    /// encompassing the shape. This will be based on
    /// relative_rect.
    position: Option<UIPosition>,
    vertices: Vec<Vertex>,
    rectangle_strategy: bool,
    color: Color,
    rerender: bool,
}

impl Shape {
    pub fn new(position: Option<UIPosition>, vertices: Vec<Vertex>, color: Color) -> Self {
        Self {
            rectangle_strategy: vertices.is_empty() && position.is_none(),
            global_bounds: vertex_array_bounds(&vertices).as_other(),
            position,
            vertices,
            color,
            rerender: true,
        }
    }

    pub fn with_vertex_string(position: Option<UIPosition>, vertices: &str, color: Color) -> Self {
        Self::new(position, vertex_array_from_string(vertices), color)
    }
}

impl Element for Shape {
    cast_element!();

    fn global_bounds(&self) -> IntRect {
        todo!()
    }

    fn update_size(&mut self) {
        self.global_bounds = vertex_array_bounds(&self.vertices).as_other();
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        let new_gb = self
            .position
            .unwrap_or_default()
            .center_with_size(relative_rect, self.global_bounds.size());

        let offset = self.global_bounds.size() - new_gb.size();
        let offset = offset.as_other();

        for vertex in &mut self.vertices {
            vertex.position -= offset;
        }

        self.update_size();
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = Some(ui_position);
        self.update_position(relative_rect);
    }

    fn render(&mut self, render_texture: &mut sfml::graphics::RenderTexture) {
        let mut rs = RenderStates::default();
    }
}
