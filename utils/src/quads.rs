use std::ops::{Index, IndexMut};

use sfml::{
    graphics::{Color, FloatRect, IntRect, RcSprite, Sprite, Vertex},
    system::{Vector2, Vector2f},
};
use tracing::error;

use super::sfml_util_functions::{
    bottom_left_rect_coords, bottom_right_rect_coords, top_right_rect_coords,
};

#[derive(Clone, Default, Debug)]
pub struct Quad(pub [Vertex; 4]);

const VERTEX_DEFAULT_COLOR: Color = Color::WHITE;
impl Quad {
    /// Returns a new quad with the same texture coordinates as the input quad.
    /// Adjusts the position of the quad to the outer rect.
    ///
    /// Returns empty vector if provided vertex array len != 4
    ///
    /// UB if the quad is not a rectangle
    pub fn with_positions_from_rect(&self, rect: FloatRect) -> Quad {
        if self.0.len() != 4 {
            error!(
                "vertex_array.len() != 4. vertex_array_len() is {}",
                self.0.len()
            );
            return Default::default();
        }
        let rect_size_pos = bottom_right_rect_coords(rect);
        Quad([
            Vertex::new(rect.position(), VERTEX_DEFAULT_COLOR, self[0].tex_coords),
            Vertex::new(
                Vector2::new(rect_size_pos.x, rect.top),
                VERTEX_DEFAULT_COLOR,
                self[1].tex_coords,
            ),
            Vertex::new(
                Vector2::new(rect_size_pos.x, rect_size_pos.y),
                VERTEX_DEFAULT_COLOR,
                self[2].tex_coords,
            ),
            Vertex::new(
                Vector2::new(rect.left, rect_size_pos.y),
                VERTEX_DEFAULT_COLOR,
                self[3].tex_coords,
            ),
        ])
    }

    pub fn into_rect(&self) -> FloatRect {
        FloatRect::from_vecs(self[0].position, self[2].position - self[0].position)
    }

    pub fn mut_quad_positions_to_rect(&mut self, rect: FloatRect) {
        self[0].position = rect.position();
        self[1].position = Vector2f::new(rect.left + rect.width, rect.top);
        self[2].position = Vector2f::new(rect.left + rect.width, rect.top + rect.height);
        self[3].position = Vector2f::new(rect.left, rect.top + rect.height);
    }

    pub fn set_quad_to_one_color(&mut self, color: Color) {
        for vertex in &mut self.0 {
            vertex.color = color;
        }
    }
}

impl Index<usize> for Quad {
    type Output = Vertex;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Quad {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl From<RcSprite> for Quad {
    fn from(sprite: RcSprite) -> Self {
        let gl = sprite.global_bounds();
        let gl_size_pos = bottom_right_rect_coords(gl);
        let tx_rect = sprite.texture_rect().as_other();
        let tx_rect_size_pos = bottom_right_rect_coords(tx_rect);

        Quad([
            Vertex::new(gl.position(), VERTEX_DEFAULT_COLOR, tx_rect.position()),
            Vertex::new(
                Vector2::new(gl_size_pos.x, gl.top),
                VERTEX_DEFAULT_COLOR,
                Vector2::new(tx_rect_size_pos.x, tx_rect.top),
            ),
            Vertex::new(
                Vector2::new(gl_size_pos.x, gl_size_pos.y),
                VERTEX_DEFAULT_COLOR,
                Vector2::new(tx_rect_size_pos.x, tx_rect_size_pos.y),
            ),
            Vertex::new(
                Vector2::new(gl.left, gl_size_pos.y),
                VERTEX_DEFAULT_COLOR,
                Vector2::new(tx_rect.left, tx_rect_size_pos.y),
            ),
        ])
    }
}

impl<'a> From<Sprite<'a>> for Quad {
    fn from(sprite: Sprite) -> Self {
        let gl = sprite.global_bounds();
        let gl_size_pos = bottom_left_rect_coords(gl);
        let tx_rect = sprite.texture_rect().as_other();
        let tx_rect_size_pos = bottom_right_rect_coords(tx_rect);

        Quad([
            Vertex::new(gl.position(), VERTEX_DEFAULT_COLOR, tx_rect.position()),
            Vertex::new(
                Vector2::new(gl_size_pos.x, gl.top),
                VERTEX_DEFAULT_COLOR,
                Vector2::new(tx_rect_size_pos.x, tx_rect.top),
            ),
            Vertex::new(
                Vector2::new(gl_size_pos.x, gl_size_pos.y),
                VERTEX_DEFAULT_COLOR,
                Vector2::new(tx_rect_size_pos.x, tx_rect_size_pos.y),
            ),
            Vertex::new(
                Vector2::new(gl.left, gl_size_pos.y),
                VERTEX_DEFAULT_COLOR,
                Vector2::new(tx_rect.left, tx_rect_size_pos.y),
            ),
        ])
    }
}

impl From<FloatRect> for Quad {
    fn from(rect: FloatRect) -> Self {
        Quad([
            Vertex::with_pos(rect.position()),
            Vertex::with_pos(top_right_rect_coords(rect)),
            Vertex::with_pos(bottom_right_rect_coords(rect)),
            Vertex::with_pos(bottom_left_rect_coords(rect)),
        ])
    }
}

impl From<IntRect> for Quad {
    fn from(rect: IntRect) -> Self {
        Self::from(rect.as_other::<f32>())
    }
}
