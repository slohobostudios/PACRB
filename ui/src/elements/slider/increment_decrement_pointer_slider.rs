use super::traits::*;
use crate::{
    elements::{
        button::{
            image_button::ImageButton, repeatable_sprite_button::RepeatableSpritesButton, traits::*,
        },
        text::Text,
        tiling_sprites::repeatable_3x1_sprite::Repeatable3x1Sprite,
        traits::{cast_actionable_element, cast_element, ActionableElement, Element},
    },
    events::*,
    syncs::{ui_syncs_not_synced_str, Syncs},
    ui_settings::UISettings,
    utils::{mouse_ui_states::UIMouseStates, positioning::UIPosition},
};
use sfml::{
    graphics::{Color, IntRect, RenderTexture},
    system::{Vector2, Vector2f, Vector2i},
    window::Event as SFMLEvent,
};
use std::time::Instant;
use tracing::warn;
use utils::resource_manager::ResourceManager;

const INCREMENT_BUTTON_POSITION: UIPosition = UIPosition {
    top: Some(100),
    bottom: Some(1),
    left: None,
    right: Some(0),
};
const DECREMENT_BUTTON_POSITION: UIPosition = UIPosition {
    top: Some(100),
    bottom: Some(1),
    left: Some(0),
    right: None,
};
const SLIDER_POSITION: UIPosition = UIPosition {
    top: Some(100),
    bottom: Some(1),
    right: None,
    left: None,
};
const POINTER_POSITION: UIPosition = UIPosition {
    top: Some(1),
    bottom: Some(100),
    right: None,
    left: None,
};

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum IncrementDecrementClickState {
    Increment((Instant, Instant)),
    Decrement((Instant, Instant)),
    None,
}

impl IncrementDecrementClickState {
    fn is_update_needed(&self) -> bool {
        use crate::utils::animation_constants::*;
        use IncrementDecrementClickState::*;
        match self {
            Increment((bind_pressed_instant, last_update_instant))
                if bind_pressed_instant.elapsed() > TIME_BETWEEN_BIND_PRESSED
                    && last_update_instant.elapsed() > TIME_BETWEEN_UPDATES =>
            {
                true
            }

            Decrement((bind_pressed_instant, last_update_instant))
                if bind_pressed_instant.elapsed() > TIME_BETWEEN_BIND_PRESSED
                    && last_update_instant.elapsed() > TIME_BETWEEN_UPDATES =>
            {
                true
            }
            _ => false,
        }
    }

    fn increment_last_update_instant(&mut self) {
        use IncrementDecrementClickState::*;
        *self = match self {
            Increment((bind_pressed_instant, _)) => {
                Increment((*bind_pressed_instant, Instant::now()))
            }
            Decrement((bind_pressed_instant, _)) => {
                Decrement((*bind_pressed_instant, Instant::now()))
            }
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IncrementDecrementPointerSlider {
    global_bounds: IntRect,
    is_dragging: bool,
    position: UIPosition,
    decrement_button: ImageButton,
    increment_button: ImageButton,
    pointer: ImageButton,
    slider: RepeatableSpritesButton,
    text: Text,
    min_max_slider_values: (f32, f32),
    current_slider_value: f32,
    increment_amt: f32,
    increment_decrement_click_state: IncrementDecrementClickState,
    event_id: u16,
    sync_id: u16,
    scale: f32,
    rerender: bool,
}

impl IncrementDecrementPointerSlider {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        scale: f32,
        asset_id: &str,
        font_size: u32,
        color: Color,
        frame_id: usize,
        hover_frame_id: usize,
        click_frame_id: usize,
        desired_size: u16,
        min_max_slider_values: (f32, f32),
        increment_amt: f32,
        event_id: u16,
        sync_id: u16,
    ) -> Self {
        // Ensure the max value is on the right
        let min_max_slider_values = if min_max_slider_values.0 > min_max_slider_values.1 {
            (min_max_slider_values.1, min_max_slider_values.0)
        } else {
            min_max_slider_values
        };

        let mut idps = Self {
            global_bounds: Default::default(),
            scale,
            is_dragging: false,
            position,
            event_id,
            increment_amt,
            sync_id,
            increment_decrement_click_state: IncrementDecrementClickState::None,
            decrement_button: ImageButton::with_texture_bounds(
                resource_manager,
                DECREMENT_BUTTON_POSITION,
                asset_id,
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("decrement", frame_id),
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("decrement", hover_frame_id),
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("decrement", click_frame_id),
                scale,
                0,
                0,
            ),
            increment_button: ImageButton::with_texture_bounds(
                resource_manager,
                INCREMENT_BUTTON_POSITION,
                asset_id,
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("increment", frame_id),
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("increment", hover_frame_id),
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("increment", click_frame_id),
                scale,
                0,
                0,
            ),
            pointer: ImageButton::with_texture_bounds(
                resource_manager,
                POINTER_POSITION,
                asset_id,
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("cursor", frame_id),
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("cursor", hover_frame_id),
                resource_manager
                    .fetch_asset(asset_id)
                    .get_shifted_slice_bound("cursor", click_frame_id),
                scale,
                0,
                0,
            ),
            slider: RepeatableSpritesButton::new(
                resource_manager,
                SLIDER_POSITION,
                Box::new(Repeatable3x1Sprite::new(
                    resource_manager,
                    asset_id,
                    frame_id,
                    position,
                    desired_size,
                    scale,
                )),
                Box::new(Repeatable3x1Sprite::new(
                    resource_manager,
                    asset_id,
                    hover_frame_id,
                    position,
                    desired_size,
                    scale,
                )),
                Box::new(Repeatable3x1Sprite::new(
                    resource_manager,
                    asset_id,
                    click_frame_id,
                    position,
                    desired_size,
                    scale,
                )),
            ),
            text: Text::new(
                resource_manager,
                Default::default(),
                "NaN",
                false,
                font_size,
                color,
                0,
            ),
            min_max_slider_values,
            current_slider_value: (min_max_slider_values.0 + min_max_slider_values.1) / 2f32,
            rerender: true,
        };
        idps.update_size();
        idps.set_current_slider_value(Vector2::new(idps.current_slider_value, 0f32));

        idps
    }

    fn compact_ele_mut(&mut self) -> [&mut dyn Element; 4] {
        [
            self.slider.as_mut_element(),
            self.pointer.as_mut_element(),
            self.increment_button.as_mut_element(),
            self.decrement_button.as_mut_element(),
        ]
    }

    fn compact_button_mut(&mut self) -> [&mut dyn Button; 4] {
        [
            self.slider.as_mut_button(),
            self.pointer.as_mut_button(),
            self.increment_button.as_mut_button(),
            self.decrement_button.as_mut_button(),
        ]
    }
}

impl Element for IncrementDecrementPointerSlider {
    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }
    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        if self.is_dragging {
            self.pointer
                .as_mut_element()
                .event_handler(ui_settings, event);
            self.slider
                .as_mut_element()
                .event_handler(ui_settings, event);
        } else {
            for ele in self.compact_ele_mut() {
                ele.event_handler(ui_settings, event);
            }
        };
        let events = Slider::event_handler(self, ui_settings, event);

        match event {
            // If we are holding the mouse down and drag over the button, begin the timer
            SFMLEvent::MouseMoved { x: _, y: _ }
                if self.increment_decrement_click_state == IncrementDecrementClickState::None
                    && self.increment_button.is_hover()
                    && self.increment_button.current_mouse_state() == UIMouseStates::Click =>
            {
                self.increment_decrement_click_state =
                    IncrementDecrementClickState::Increment((Instant::now(), Instant::now()))
            }
            // If we are holding the mouse down and drag over the button, begin the timer
            SFMLEvent::MouseMoved { x: _, y: _ }
                if self.increment_decrement_click_state == IncrementDecrementClickState::None
                    && self.decrement_button.is_hover()
                    && self.decrement_button.current_mouse_state() == UIMouseStates::Click =>
            {
                self.increment_decrement_click_state =
                    IncrementDecrementClickState::Decrement((Instant::now(), Instant::now()))
            }
            // If we move the mouse around and aren't hovering either button, stop the timer
            SFMLEvent::MouseMoved { x: _, y: _ }
                if !self.increment_button.is_hover() && !self.decrement_button.is_hover() =>
            {
                self.increment_decrement_click_state = IncrementDecrementClickState::None
            }
            _ => {}
        }

        // if we are dragging the slider around, make the increment and decrement button get out
        // of the hover state even if we are hovering
        if self.is_dragging() {
            self.increment_button
                .set_hover(Vector2::new(i32::MIN, i32::MIN));
            self.decrement_button
                .set_hover(Vector2::new(i32::MIN, i32::MIN));
        }

        events
    }

    fn update_size(&mut self) {
        for ele in self.compact_ele_mut() {
            ele.update_size();
        }

        self.global_bounds.height =
            (self.increment_button.global_bounds().height as f32 - self.scale) as i32;
        self.global_bounds.width = (self.decrement_button.global_bounds().width as f32
            + self.scale
            + self.slider.global_bounds().width as f32
            + self.scale
            + self.increment_button.global_bounds().width as f32)
            as i32;
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        let global_bounds = self.global_bounds;
        for ele in self.compact_ele_mut() {
            ele.update_position(global_bounds);
        }

        self.set_current_slider_value(Vector2f::new(self.current_slider_value, 0f32));
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        let mut rerender = self.rerender;
        let mut events = Vec::new();
        for ele in self.compact_ele_mut() {
            let mut event = ele.update(resource_manager);
            rerender |= event.1;
            events.append(&mut event.0);
        }

        if self.increment_decrement_click_state.is_update_needed() {
            let (new_slider_value, new_slider_value_was_computed) =
                match self.increment_decrement_click_state {
                    IncrementDecrementClickState::Increment(_) => {
                        self.increment_decrement_click_state
                            .increment_last_update_instant();
                        (self.current_slider_value + self.increment_amt, true)
                    }
                    IncrementDecrementClickState::Decrement(_) => {
                        self.increment_decrement_click_state
                            .increment_last_update_instant();
                        (self.current_slider_value - self.increment_amt, true)
                    }
                    _ => (self.current_slider_value, false),
                };
            self.set_current_slider_value(Vector2::new(new_slider_value, 0.));
            if new_slider_value_was_computed {
                events.append(&mut Vec::from([self.triggered_event()]));
            }
        }

        self.rerender |= rerender;
        (events, self.rerender)
    }

    fn render(&mut self, window: &mut RenderTexture) {
        for ele in self.compact_ele_mut() {
            ele.render(window);
        }

        self.text.render(window);
        self.rerender = false;
    }

    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn sync(&mut self, sync: Syncs) {
        let Syncs::Numerical(new_slider_value) = sync else {
            warn!(ui_syncs_not_synced_str!(), Syncs::Numerical(Default::default()), sync);
            return;
        };

        self.set_current_slider_value(Vector2::new(new_slider_value, new_slider_value));
        self.rerender = true;
    }

    cast_element!();
}

impl ActionableElement for IncrementDecrementPointerSlider {
    fn triggered_event(&self) -> Event {
        Event::new(
            self.event_id,
            Events::NumericalEvent(self.current_slider_value),
        )
    }
    fn bind_pressed(&mut self, mouse_pos: Vector2i) {
        for ele in self.compact_button_mut() {
            ele.bind_pressed(mouse_pos);
        }
        if self.increment_button.current_mouse_state() == UIMouseStates::Click {
            self.increment_decrement_click_state =
                IncrementDecrementClickState::Increment((Instant::now(), Instant::now()));
        }
        if self.decrement_button.current_mouse_state() == UIMouseStates::Click {
            self.increment_decrement_click_state =
                IncrementDecrementClickState::Decrement((Instant::now(), Instant::now()));
        }
        if self.slider.global_bounds().contains(mouse_pos)
            || self.pointer.global_bounds().contains(mouse_pos)
        {
            self.is_dragging = true;
        }
        if self.is_dragging {
            self.set_slider_position_by_cursor_coords(mouse_pos)
        }
    }

    fn bind_released(&mut self, _: Vector2i) {
        self.increment_decrement_click_state = IncrementDecrementClickState::None;
        if !self.is_dragging {
            if self.increment_button.is_hover() {
                self.set_current_slider_value(Vector2f::new(
                    self.current_slider_value + self.increment_amt,
                    0.,
                ))
            }
            if self.decrement_button.is_hover() {
                self.set_current_slider_value(Vector2f::new(
                    self.current_slider_value - self.increment_amt,
                    0.,
                ))
            }
        }
        self.is_dragging = false;
    }

    fn set_hover(&mut self, mouse_pos: Vector2i) {
        for ele in self.compact_button_mut() {
            ele.set_hover(mouse_pos);
        }
    }

    fn is_hover(&self) -> bool {
        self.increment_button.is_hover()
            || self.decrement_button.is_hover()
            || self.pointer.is_hover()
            || self.slider.is_hover()
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }

    cast_actionable_element!();
}

impl Slider for IncrementDecrementPointerSlider {
    fn slider_global_bounds(&mut self) -> IntRect {
        self.slider.global_bounds()
    }

    fn is_dragging(&self) -> bool {
        self.is_dragging
    }

    fn min_slider_value(&mut self) -> Vector2f {
        Vector2f::new(self.min_max_slider_values.0, 0.)
    }

    fn max_slider_value(&mut self) -> Vector2f {
        Vector2f::new(self.min_max_slider_values.1, 0.)
    }

    /// Sets the slider position and text based on new slider value. Does nothing if out of range
    fn set_current_slider_value(&mut self, new_slider_value: Vector2f) {
        let new_slider_value = new_slider_value.x;
        if new_slider_value > self.min_max_slider_values.1
            || new_slider_value < self.min_max_slider_values.0
        {
            return;
        }
        // Adjust slider value to valid increment
        self.current_slider_value =
            (new_slider_value / self.increment_amt).round() * self.increment_amt;

        // Get the u16 percentage from the current slider value. Need to do this again because of increment amount
        let slider_percentage =
            (((self.current_slider_value - self.min_max_slider_values.0) * f32::from(u16::MAX))
                / (self.min_max_slider_values.1 - self.min_max_slider_values.0)) as u16;

        let relative_rect = IntRect {
            top: self.global_bounds.top,
            left: self.slider.global_bounds().left,
            width: self.slider.global_bounds().width,
            height: self.global_bounds.height,
        };

        self.pointer.set_ui_position(
            UIPosition {
                top: POINTER_POSITION.top,
                bottom: POINTER_POSITION.bottom,
                left: Some(i32::from(slider_percentage)),
                right: Some(i32::from(u16::MAX - slider_percentage)),
            },
            relative_rect,
        );

        self.text.set_text(&self.current_slider_value.to_string());
        self.text.set_ui_position(
            UIPosition {
                top: Some(-self.text.global_bounds().height - self.pointer.global_bounds().height),
                bottom: None,
                left: Some(i32::from(slider_percentage)),
                right: Some(i32::from(u16::MAX - slider_percentage)),
            },
            relative_rect,
        );
        self.rerender = true;
    }

    fn box_clone(&self) -> Box<dyn Slider> {
        Box::new(self.clone())
    }
}
