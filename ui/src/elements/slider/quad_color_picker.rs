use sfml::{
    graphics::{Color, IntRect, PrimitiveType, RenderStates, RenderTarget, RenderTexture, Vertex},
    system::{Vector2, Vector2f, Vector2i, Vector2u},
    window::Event as SFMLEvent,
};
use utils::{
    arithmetic_util_functions::i32_from_u32,
    quads::Quad,
    resource_manager::ResourceManager,
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
    syncs::Syncs,
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};

use super::traits::Slider;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct QuadColorPickerSync {
    pub top_left_color: Option<Color>,
    pub top_right_color: Option<Color>,
    pub bottom_left_color: Option<Color>,
    pub bottom_right_color: Option<Color>,
    /// Calculates the position based off of a percentage.
    /// x: 0% and y: 0% is the top left corner, and x:100% and y: 100%
    /// is the bottom right corner.
    ///
    /// The caclulation is done via floating point values between 0-65535
    /// 65535 being 100% and 0 being 0%
    pub hover_element_position_percentage: Option<Vector2<u16>>,
}

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
    rerender: bool,
}

impl QuadColorPicker {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        hover_element: Element,
        position: UIPosition,
        size: Vector2u,
        top_left_color: Color,
        top_right_color: Color,
        bottom_right_color: Color,
        bottom_left_color: Color,
        event_id: u16,
        sync_id: u16,
    ) -> Self {
        let mut qcp = Self {
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
            rerender: true,
        };
        qcp.update_size();

        qcp
    }
    pub fn set_top_left_color(&mut self, color: Color) {
        self.quad[0].color = color;
        self.rerender = true;
    }

    pub fn set_top_right_color(&mut self, color: Color) {
        self.quad[1].color = color;
        self.rerender = true;
    }

    pub fn set_bottom_right_color(&mut self, color: Color) {
        self.quad[2].color = color;
        self.rerender = true;
    }

    pub fn set_bottom_left_color(&mut self, color: Color) {
        self.quad[3].color = color;
        self.rerender = true;
    }

    pub fn top_left_color(&self) -> Color {
        self.quad[0].color
    }

    pub fn top_right_color(&self) -> Color {
        self.quad[1].color
    }

    pub fn bottom_right_color(&self) -> Color {
        self.quad[2].color
    }

    pub fn bottom_left_color(&self) -> Color {
        self.quad[3].color
    }
}

impl ElementTrait for QuadColorPicker {
    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        Slider::event_handler(self, ui_settings, event)
    }
    fn update_size(&mut self) {
        self.global_bounds.width = i32_from_u32(self.size.x);
        self.global_bounds.height = i32_from_u32(self.size.y);
        self.hover_element.update_size();

        Quad::set_position_from_rect(&mut self.quad, self.global_bounds.as_other());
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        self.hover_element.update_position(self.global_bounds);
        Quad::set_position_from_rect(&mut self.quad, self.global_bounds.as_other());
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }

    fn update(&mut self, _resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        (Default::default(), self.rerender)
    }

    fn render(&mut self, window: &mut RenderTexture) {
        let rs = RenderStates::default();
        window.draw_primitives(&self.quad.0, PrimitiveType::TRIANGLE_FAN, &rs);
        self.hover_element.render(window);
        self.rerender = false;
    }

    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }

    fn sync(&mut self, sync: Syncs) {
        let Syncs::QuadColorPicker(sync_struct) = sync else {
            return;
        };

        if let Some(color) = sync_struct.top_left_color {
            self.set_top_left_color(color)
        }
        if let Some(color) = sync_struct.top_right_color {
            self.set_top_right_color(color)
        }
        if let Some(color) = sync_struct.bottom_left_color {
            self.set_bottom_left_color(color)
        }
        if let Some(color) = sync_struct.bottom_right_color {
            self.set_bottom_right_color(color)
        }
        if let Some(new_slider_value) = sync_struct.hover_element_position_percentage {
            self.set_slider_position_by_percent(new_slider_value)
        }
    }

    cast_element!();
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
        self.rerender = true;

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
