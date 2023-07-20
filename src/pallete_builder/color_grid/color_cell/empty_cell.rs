use std::time::{Duration, Instant};

use sfml::{
    graphics::{Color, IntRect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable},
    system::Vector2,
};
use utils::center_of_rect;

const OUTLINE_THICKNESS: f32 = 1f32;
const PLUS_THICKNESS: f32 = 1f32;
const PLUS_LENGTH: f32 = 16f32;
const BASE_OUTLINE_COLOR: Color = Color::rgba(0xf7, 0xe5, 0xe4, 0x00);
const DURATION_BETWEEN_FRAMES: Duration = Duration::from_millis(6);
const MIN_ALPHA_VALUE: u8 = 64;
const MAX_ALPHA_VALUE: u8 = 250;
#[derive(Debug, Clone)]
pub struct EmptyCell {
    outline: RectangleShape<'static>,
    plus: Vector2<RectangleShape<'static>>,
    increment_direction: i8,
    last_animation_frame: Instant,
    color: Color,
    pub is_hover: bool,
}

impl EmptyCell {
    pub fn new(mut global_bounds: IntRect) -> Self {
        let position = center_of_rect!(i32, global_bounds);
        let position = Vector2::new(
            position.x + OUTLINE_THICKNESS as i32,
            position.y + OUTLINE_THICKNESS as i32,
        )
        .as_other();
        global_bounds.width -= (OUTLINE_THICKNESS * 2.) as i32;
        global_bounds.height -= (OUTLINE_THICKNESS * 2.) as i32;
        let mut outline = RectangleShape::with_size(global_bounds.size().as_other());
        outline.set_fill_color(Color::TRANSPARENT);
        outline.set_outline_color(BASE_OUTLINE_COLOR);
        outline.set_outline_thickness(OUTLINE_THICKNESS);
        outline.set_position(position);
        outline.set_origin(outline.global_bounds().size() / 2.);
        let mut plus = Vector2::new(
            RectangleShape::with_size(Vector2::new(PLUS_THICKNESS, PLUS_LENGTH)),
            RectangleShape::with_size(Vector2::new(PLUS_LENGTH, PLUS_THICKNESS)),
        );
        plus.x.set_fill_color(BASE_OUTLINE_COLOR);
        plus.x.set_position(position);
        plus.x.set_origin(plus.x.global_bounds().size() / 2.);
        plus.y.set_fill_color(BASE_OUTLINE_COLOR);
        plus.y.set_position(position);
        plus.y.set_origin(plus.y.global_bounds().size() / 2.);

        Self {
            outline,
            plus,
            increment_direction: 1,
            last_animation_frame: Instant::now(),
            color: BASE_OUTLINE_COLOR,
            is_hover: false,
        }
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;

        self.outline.set_outline_color(self.color);
        self.plus.x.set_fill_color(self.color);
        self.plus.y.set_fill_color(self.color);
    }

    pub fn update(&mut self) {
        if !self.is_hover {
            if self.color.a == u8::MIN {
                return;
            }

            if self.last_animation_frame.elapsed() > DURATION_BETWEEN_FRAMES
                && self.color.a > u8::MIN
            {
                self.last_animation_frame = Instant::now();
                self.set_color(Color::rgba(
                    self.color.r,
                    self.color.g,
                    self.color.b,
                    self.color.a - 1,
                ));
            }

            return;
        }

        self.color = if self.last_animation_frame.elapsed() > DURATION_BETWEEN_FRAMES {
            self.last_animation_frame = Instant::now();
            let color = if self.increment_direction.is_positive() && self.color.a != u8::MAX {
                Color::rgba(self.color.r, self.color.g, self.color.b, self.color.a + 1)
            } else if self.increment_direction.is_negative() && self.color.a != u8::MIN {
                Color::rgba(self.color.r, self.color.g, self.color.b, self.color.a - 1)
            } else {
                self.color
            };
            if self.color.a <= MIN_ALPHA_VALUE {
                self.increment_direction = 1;
            } else if self.color.a >= MAX_ALPHA_VALUE {
                self.increment_direction = -1;
            }
            color
        } else {
            self.color
        };
        self.set_color(self.color);
    }

    pub fn render(&self, window: &mut RenderWindow) {
        if self.color.a != 0 {
            window.draw(&self.outline);
            window.draw(&self.plus.x);
            window.draw(&self.plus.y);
        }
    }
}

impl PartialEq for EmptyCell {
    fn eq(&self, other: &Self) -> bool {
        self.outline.global_bounds() == other.outline.global_bounds()
            && self.plus.x.global_bounds() == other.plus.x.global_bounds()
            && self.plus.y.global_bounds() == other.plus.y.global_bounds()
    }
}
