use sfml::graphics::{
    Color, IntRect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
};
use utils::center_of_rect;

use crate::pallete_builder::hsv_color::HSV;

#[derive(Debug, Clone)]
pub struct FullCell {
    color_rect: RectangleShape<'static>,
    color: HSV,
}

impl FullCell {
    pub fn new(global_bounds: IntRect) -> Self {
        let position = center_of_rect!(i32, global_bounds).as_other();
        let mut color_rect = RectangleShape::with_size(global_bounds.size().as_other());
        color_rect.set_fill_color(Color::TRANSPARENT);
        color_rect.set_outline_color(Color::TRANSPARENT);
        color_rect.set_position(position);
        color_rect.set_origin(color_rect.global_bounds().size() / 2.);
        Self {
            color_rect,
            color: Default::default(),
        }
    }
    pub fn set_color(&mut self, hsv_color: HSV) {
        self.color = hsv_color;
        self.color_rect.set_fill_color(self.color.into());
    }
    pub fn render(&self, window: &mut RenderWindow) {
        window.draw(&self.color_rect);
    }

    pub fn current_color(&self) -> HSV {
        self.color
    }
}

impl PartialEq for FullCell {
    fn eq(&self, other: &Self) -> bool {
        self.color_rect.global_bounds() == other.color_rect.global_bounds()
            && self.color == other.color
    }
}
