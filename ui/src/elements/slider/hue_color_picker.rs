use sfml::{
    graphics::{Color, IntRect, PrimitiveType, RenderStates, RenderTarget, RenderTexture, Vertex},
    system::{Vector2f, Vector2i, Vector2u},
    window::Event as SFMLEvent,
};
use tracing::warn;
use utils::{
    arithmetic_util_functions::{i32_from_u32, i32_from_usize, u16_from_usize},
    quads::Quad,
    resource_manager::ResourceManager,
    sfml_util_functions::vector2i_from_vector2u,
};

use crate::{
    elements::{
        traits::{
            cast_actionable_element, cast_element, ActionableElement, Element as ElementTrait,
        },
        Element,
    },
    events::{Event, EventId, Events},
    syncs::ui_syncs_not_synced_str,
    syncs::Syncs,
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};

use super::traits::Slider;

const NUM_OF_QUADS: u8 = 6;

const ONE: Color = Color::rgb(255, 0, 0);
const TWO: Color = Color::rgb(255, 255, 0);
const THREE: Color = Color::rgb(0, 255, 0);
const FOUR: Color = Color::rgb(0, 255, 255);
const FIVE: Color = Color::rgb(0, 0, 255);
const SIX: Color = Color::rgb(255, 0, 255);
const QUADS_DEFAULT: [Quad; NUM_OF_QUADS as usize] = [
    Quad([
        Vertex::with_pos_color(Vertex::DEFAULT.position, ONE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, TWO),
        Vertex::with_pos_color(Vertex::DEFAULT.position, TWO),
        Vertex::with_pos_color(Vertex::DEFAULT.position, ONE),
    ]),
    Quad([
        Vertex::with_pos_color(Vertex::DEFAULT.position, TWO),
        Vertex::with_pos_color(Vertex::DEFAULT.position, THREE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, THREE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, TWO),
    ]),
    Quad([
        Vertex::with_pos_color(Vertex::DEFAULT.position, THREE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, FOUR),
        Vertex::with_pos_color(Vertex::DEFAULT.position, FOUR),
        Vertex::with_pos_color(Vertex::DEFAULT.position, THREE),
    ]),
    Quad([
        Vertex::with_pos_color(Vertex::DEFAULT.position, FOUR),
        Vertex::with_pos_color(Vertex::DEFAULT.position, FIVE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, FIVE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, FOUR),
    ]),
    Quad([
        Vertex::with_pos_color(Vertex::DEFAULT.position, FIVE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, SIX),
        Vertex::with_pos_color(Vertex::DEFAULT.position, SIX),
        Vertex::with_pos_color(Vertex::DEFAULT.position, FIVE),
    ]),
    Quad([
        Vertex::with_pos_color(Vertex::DEFAULT.position, SIX),
        Vertex::with_pos_color(Vertex::DEFAULT.position, ONE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, ONE),
        Vertex::with_pos_color(Vertex::DEFAULT.position, SIX),
    ]),
];

/// This struct NEEDS to be defined on the heap.
/// It stores and internal array that if defined on the stack, can cause stack oveflow.#[derive(Debug, Clone, Default)]
#[derive(Debug, Clone, Default)]
pub struct HueColorPicker {
    position: UIPosition,
    size: Vector2u,
    global_bounds: IntRect,
    event_id: u16,
    sync_id: u16,
    quads: [Quad; NUM_OF_QUADS as usize],
    curr_hue: u16,
    is_hover: bool,
    is_dragging: bool,
    rerender: bool,
    hover_element: Element,
}

impl HueColorPicker {
    pub fn new(
        hover_element: Element,
        position: UIPosition,
        size: Vector2u,
        event_id: u16,
        sync_id: u16,
    ) -> Self {
        Self {
            position,
            event_id,
            sync_id,
            global_bounds: IntRect::from_vecs(Default::default(), vector2i_from_vector2u(size)),
            size,
            quads: QUADS_DEFAULT,
            curr_hue: Default::default(),
            is_hover: false,
            is_dragging: false,
            hover_element,
            rerender: true,
        }
    }

    fn set_quad_positions(&mut self) {
        let each_quad_width = self.global_bounds.width / i32::from(NUM_OF_QUADS);
        for (idx, quad) in self.quads.iter_mut().enumerate() {
            let rect = IntRect::new(
                i32_from_usize(idx) * each_quad_width + self.global_bounds.left,
                self.global_bounds.top,
                each_quad_width,
                self.global_bounds.height,
            );

            Quad::set_position_from_rect(quad, rect.as_other());
        }
    }
}

impl ElementTrait for HueColorPicker {
    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        Slider::event_handler(self, ui_settings, event)
    }

    fn update_size(&mut self) {
        // Make sure width is of a multiple of 6
        self.global_bounds.width =
            i32_from_u32(self.size.x / u32::from(NUM_OF_QUADS) * u32::from(NUM_OF_QUADS));
        self.global_bounds.height = i32_from_u32(self.size.y);

        self.hover_element.update_size();
        self.set_quad_positions();
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());

        self.hover_element.update_position(self.global_bounds);
        self.set_quad_positions();
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
        for quad in &self.quads {
            window.draw_primitives(&quad.0, PrimitiveType::TRIANGLE_FAN, &rs)
        }

        self.hover_element.render(window);
        self.rerender = false;
    }

    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn sync(&mut self, sync: Syncs) {
        let Syncs::Numerical(degree) = sync else {
            warn!(ui_syncs_not_synced_str!(), Syncs::Numerical(Default::default()), sync);
            return;
        };
        self.rerender = true;
        self.set_current_slider_value(Vector2f::new(degree, degree));
    }

    cast_element!();
}

impl ActionableElement for HueColorPicker {
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
    fn bind_released(&mut self, _: Vector2i) {
        self.is_dragging = false
    }
    fn triggered_event(&self) -> Event {
        Event::new(
            self.event_id,
            Events::NumericalEvent(f32::from(self.curr_hue)),
        )
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }
}

impl Slider for HueColorPicker {
    fn slider_global_bounds(&mut self) -> IntRect {
        self.global_bounds
    }

    fn is_dragging(&self) -> bool {
        self.is_dragging
    }
    fn min_slider_value(&mut self) -> Vector2f {
        Vector2f::new(0., 0.)
    }
    fn max_slider_value(&mut self) -> Vector2f {
        Vector2f::new(360., 0.)
    }

    fn set_current_slider_value(&mut self, new_slider_value: Vector2f) {
        let new_slider_value = new_slider_value.x;
        if !(0. ..=360.).contains(&new_slider_value) {
            return;
        }

        self.curr_hue = new_slider_value.round() as u16;

        let slider_percentage =
            u16_from_usize(usize::from(self.curr_hue) * usize::from(u16::MAX) / 360);

        self.hover_element.set_ui_position(
            UIPosition {
                top: None,
                bottom: None,
                left: Some(i32::from(slider_percentage)),
                right: Some(i32::from(u16::MAX - slider_percentage)),
            },
            self.global_bounds,
        );

        self.rerender = true;
    }

    fn box_clone(&self) -> Box<dyn Slider> {
        Box::new(self.clone())
    }
}
