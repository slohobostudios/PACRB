use std::ops::{Add, Index, IndexMut};

use sfml::{
    graphics::{Color, FloatRect, IntRect, RcSprite, Rect, Sprite, Vertex},
    system::Vector2,
};
use tracing::error;

use super::sfml_util_functions::{
    bottom_left_rect_coords, bottom_right_rect_coords, top_right_rect_coords,
};

fn rect_corner_positions<T: Add + Add<Output = T> + Copy>(
    rect: Rect<T>,
) -> (Vector2<T>, Vector2<T>, Vector2<T>, Vector2<T>) {
    (
        rect.position(),
        top_right_rect_coords(rect),
        bottom_right_rect_coords(rect),
        bottom_left_rect_coords(rect),
    )
}

/// Quad is an abstraction to TriangleFan, but used as if it were a quad.
///
/// MUST USE [`PrimitiveType::TriangleFan`] when rendering.
///
/// Order of vertices goes like this:
///
/// 1----2
/// |\   |
/// | \  |
/// |  \ |
/// |   \|
/// 4----3
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
        let pos = rect_corner_positions(rect);
        Quad([
            Vertex::new(pos.0, VERTEX_DEFAULT_COLOR, self[0].tex_coords),
            Vertex::new(pos.1, VERTEX_DEFAULT_COLOR, self[1].tex_coords),
            Vertex::new(pos.2, VERTEX_DEFAULT_COLOR, self[2].tex_coords),
            Vertex::new(pos.3, VERTEX_DEFAULT_COLOR, self[3].tex_coords),
        ])
    }

    pub fn into_rect(&self) -> FloatRect {
        FloatRect::from_vecs(self[0].position, self[2].position - self[0].position)
    }

    pub fn mut_quad_positions_to_rect(&mut self, rect: FloatRect) {
        let pos = rect_corner_positions(rect);
        self[0].position = pos.0;
        self[1].position = pos.1;
        self[2].position = pos.2;
        self[3].position = pos.3;
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
        let pos = rect_corner_positions(sprite.global_bounds());
        let tx = rect_corner_positions::<f32>(sprite.texture_rect().as_other());

        Quad([
            Vertex::new(pos.0, VERTEX_DEFAULT_COLOR, tx.0),
            Vertex::new(pos.1, VERTEX_DEFAULT_COLOR, tx.1),
            Vertex::new(pos.3, VERTEX_DEFAULT_COLOR, tx.2),
            Vertex::new(pos.3, VERTEX_DEFAULT_COLOR, tx.3),
        ])
    }
}

impl<'a> From<Sprite<'a>> for Quad {
    fn from(sprite: Sprite) -> Self {
        let pos = rect_corner_positions(sprite.global_bounds());
        let tx = rect_corner_positions::<f32>(sprite.texture_rect().as_other());

        Quad([
            Vertex::new(pos.0, VERTEX_DEFAULT_COLOR, tx.0),
            Vertex::new(pos.1, VERTEX_DEFAULT_COLOR, tx.1),
            Vertex::new(pos.3, VERTEX_DEFAULT_COLOR, tx.2),
            Vertex::new(pos.3, VERTEX_DEFAULT_COLOR, tx.3),
        ])
    }
}

impl From<FloatRect> for Quad {
    fn from(rect: FloatRect) -> Self {
        let pos = rect_corner_positions(rect);
        Quad([
            Vertex::with_pos(pos.0),
            Vertex::with_pos(pos.1),
            Vertex::with_pos(pos.2),
            Vertex::with_pos(pos.3),
        ])
    }
}

impl From<IntRect> for Quad {
    fn from(rect: IntRect) -> Self {
        Self::from(rect.as_other::<f32>())
    }
}
