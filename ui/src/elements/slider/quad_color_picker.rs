use sfml::{
    graphics::{Color, IntRect, PrimitiveType, RenderStates, RenderTarget, RenderTexture, Vertex},
    system::{Vector2, Vector2f, Vector2i, Vector2u},
    window::Event as SFMLEvent,
};
use utils::{
    arithmetic_util_functions::i32_from_u32,
    quads::Quad,
    sfml_util_functions::{bottom_right_rect_coords, vector2i_from_vector2u},
};

use crate::{
    elements::{
        traits::{
            cast_actionable_element, cast_element, ActionableElement, Element as ElementTrait,
        },
        Element,
    },
    events::{Event, EventId, Events},
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};

use super::traits::{QuadColorPickerTrait, Slider};

/// This struct NEEDS to be defined on the heap.
/// It stores and internal array that if defined on the stack, can cause stack oveflow.
#[derive(Debug, Clone, Default)]
pub struct QuadColorPicker {
    position: UIPosition,
    global_bounds: IntRect,
    size: Vector2u,
    quad: Quad,
    event_id: u16,
    sync_id: u16,
    current_selection_relative_coords: Vector2f,
    is_hover: bool,
    is_dragging: bool,
    hover_element: Element,
}

impl QuadColorPicker {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        hover_element: Element,
        position: UIPosition,
        size: Vector2u,
        top_left_color: Color,
        top_right_color: Color,
        bottom_left_color: Color,
        bottom_right_color: Color,
        event_id: u16,
        sync_id: u16,
    ) -> Self {
        Self {
            hover_element,
            position,
            global_bounds: IntRect::from_vecs(Default::default(), vector2i_from_vector2u(size)),
            is_dragging: false,
            quad: Quad([
                Vertex {
                    color: top_left_color,
                    ..Default::default()
                },
                Vertex {
                    color: top_right_color,
                    ..Default::default()
                },
                Vertex {
                    color: bottom_right_color,
                    ..Default::default()
                },
                Vertex {
                    color: bottom_left_color,
                    ..Default::default()
                },
            ]),
            current_selection_relative_coords: Default::default(),
            event_id,
            sync_id,
            size,
            is_hover: false,
        }
    }
}

impl QuadColorPickerTrait for QuadColorPicker {
    fn set_top_left_color(&mut self, color: Color) {
        self.quad[0].color = color;
    }

    fn set_top_right_color(&mut self, color: Color) {
        self.quad[1].color = color;
    }

    fn set_bottom_right_color(&mut self, color: Color) {
        self.quad[2].color = color;
    }

    fn set_bottom_left_color(&mut self, color: Color) {
        self.quad[3].color = color;
    }

    fn top_left_color(&self) -> Color {
        self.quad[0].color
    }

    fn top_right_color(&self) -> Color {
        self.quad[1].color
    }

    fn bottom_right_color(&self) -> Color {
        self.quad[2].color
    }

    fn bottom_left_color(&self) -> Color {
        self.quad[3].color
    }
}

impl ElementTrait for QuadColorPicker {
    cast_element!();
    fn update_size(&mut self) {
        self.global_bounds.width = i32_from_u32(self.size.x);
        self.global_bounds.height = i32_from_u32(self.size.y);
        self.hover_element.update_size();

        Quad::mut_quad_positions_to_rect(&mut self.quad, self.global_bounds.as_other());
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        self.hover_element.update_position(self.global_bounds);
        Quad::mut_quad_positions_to_rect(&mut self.quad, self.global_bounds.as_other());
    }

    fn render(&mut self, window: &mut RenderTexture) {
        let rs = RenderStates::default();
        window.draw_primitives(&self.quad.0, PrimitiveType::QUADS, &rs);
        self.hover_element.render(window);
    }

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        Slider::event_handler(self, ui_settings, event)
    }

    fn box_clone(&self) -> Box<dyn ElementTrait> {
        Box::new(self.clone())
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }

    fn sync_id(&self) -> u16 {
        self.sync_id
    }
}

impl ActionableElement for QuadColorPicker {
    cast_actionable_element!();
    fn set_hover(&mut self, mouse_pos: Vector2i) {
        self.is_hover = self.global_bounds.contains(mouse_pos);
    }
    fn is_hover(&self) -> bool {
        self.is_hover
    }
    fn bind_pressed(&mut self, mouse_pos: Vector2i) {
        self.set_hover(mouse_pos);
        if self.is_hover {
            self.set_slider_position_by_cursor_coords(mouse_pos);
            self.is_dragging = true;
        }
    }
    fn bind_released(&mut self, mouse_pos: Vector2i) {
        self.is_hover = self.global_bounds.contains(mouse_pos);
        if self.is_hover {
            self.set_slider_position_by_cursor_coords(mouse_pos);
        }
        self.is_dragging = false;
    }
    fn triggered_event(&self) -> Event {
        Event::new(
            self.event_id,
            Events::Vector2fEvent(self.current_selection_relative_coords),
        )
    }
}

impl Slider for QuadColorPicker {
    fn slider_global_bounds(&mut self) -> IntRect {
        self.global_bounds
    }

    fn is_dragging(&self) -> bool {
        self.is_dragging
    }
    fn min_slider_value(&mut self) -> Vector2f {
        self.global_bounds.position().as_other()
    }

    fn max_slider_value(&mut self) -> Vector2f {
        bottom_right_rect_coords(self.global_bounds).as_other()
    }

    fn set_current_slider_value(&mut self, new_slider_value: Vector2f) {
        fn calc_slider_percentage(curr_selection: f32, size: f32) -> u16 {
            (curr_selection * f32::from(u16::MAX) / size) as u16
        }
        let mut slider_percentages = Vector2::new(
            calc_slider_percentage(self.current_selection_relative_coords.x, self.size.x as f32),
            calc_slider_percentage(self.current_selection_relative_coords.y, self.size.y as f32),
        );
        if new_slider_value.x >= self.min_slider_value().x
            && new_slider_value.x <= self.max_slider_value().x
        {
            self.current_selection_relative_coords.x =
                new_slider_value.x - self.global_bounds.left as f32;

            slider_percentages.x = calc_slider_percentage(
                self.current_selection_relative_coords.x,
                self.size.x as f32,
            );
        }
        if new_slider_value.y >= self.min_slider_value().y
            && new_slider_value.y <= self.max_slider_value().y
        {
            self.current_selection_relative_coords.y =
                new_slider_value.y - self.global_bounds.top as f32;

            slider_percentages.y = calc_slider_percentage(
                self.current_selection_relative_coords.y,
                self.size.y as f32,
            );
        }

        self.hover_element.set_ui_position(
            UIPosition {
                top: Some(i32::from(slider_percentages.y)),
                bottom: Some(i32::from(u16::MAX - slider_percentages.y)),
                left: Some(i32::from(slider_percentages.x)),
                right: Some(i32::from(u16::MAX - slider_percentages.x)),
            },
            self.global_bounds,
        )
    }

    fn box_clone(&self) -> Box<dyn Slider> {
        Box::new(self.clone())
    }
}
