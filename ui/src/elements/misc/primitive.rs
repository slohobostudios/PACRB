use sfml::graphics::{
    vertex_array_bounds, Color, IntRect, PrimitiveType, RenderStates, RenderTarget, Vertex,
};
use utils::{quads::Quad, sfml_util_functions::vertex_array_from_string};

use crate::{
    elements::traits::{cast_element, Element},
    utils::positioning::UIPosition,
};

/// Usage:
///
/// Provide a [`PrimitiveType`], vertices, position, and vertices color.
/// It will draw whatever primitive you want through SFML.
///
/// Another feature is if you make position optional, and the number of
/// vertices provided is zero, it will fill up the ENTIRE space according
/// to it's outer element via relative_rect.
#[derive(Debug, Clone)]
pub struct Primitive {
    global_bounds: IntRect,
    /// position is optional because of the "rectangle strategy".
    /// If both position and number of vertices is zero, we
    /// will create a rect that is the size of the object
    /// encompassing the shape. This will be based on
    /// relative_rect.
    position: Option<UIPosition>,
    vertices: Vec<Vertex>,
    primitive_type: PrimitiveType,
    rel_rect_strategy: bool,
    rerender: bool,
}

impl Primitive {
    pub fn new(
        position: Option<UIPosition>,
        mut vertices: Vec<Vertex>,
        color: Color,
        mut primitive_type: PrimitiveType,
    ) -> Self {
        let rel_rect_strategy = vertices.is_empty() && position.is_none();
        if rel_rect_strategy {
            primitive_type = PrimitiveType::TRIANGLE_FAN;
            vertices = vec![Default::default(); 4];
        }
        for vertex in &mut vertices {
            vertex.color = color;
        }
        Self {
            rel_rect_strategy,
            global_bounds: vertex_array_bounds(&vertices).as_other(),
            position,
            vertices,
            primitive_type,
            rerender: true,
        }
    }

    pub fn with_vertex_string(
        position: Option<UIPosition>,
        vertices: &str,
        color: Color,
        primitive_type: PrimitiveType,
    ) -> Self {
        Self::new(
            position,
            vertex_array_from_string(vertices),
            color,
            primitive_type,
        )
    }
}

impl Element for Primitive {
    cast_element!();

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        self.global_bounds = vertex_array_bounds(&self.vertices).as_other();
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        if self.rel_rect_strategy {
            let quad = Quad::from(relative_rect);
            for i in 0..4 {
                self.vertices[i].position = quad[i].position;
            }
            self.update_size();
            return;
        }

        let new_gb = self
            .position
            .unwrap_or_default()
            .center_with_size(relative_rect, self.global_bounds.size());

        let offset = self.global_bounds.position() - new_gb.position();
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
        let rs = RenderStates::default();
        render_texture.draw_primitives(&self.vertices, self.primitive_type, &rs);
        self.rerender = false;
    }
}
