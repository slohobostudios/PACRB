use std::time::Instant;

use crate::{
    elements::{
        button::traits::Button,
        grouping::div::Div,
        traits::{cast_actionable_element, ActionableElement},
    },
    events::Events,
    syncs::Syncs,
    utils::mouse_ui_states::UIMouseStates,
};
use sfml::{
    graphics::{Color, IntRect},
    system::{Vector2, Vector2i},
};
use tracing::{error, warn};
use utils::{
    arithmetic_util_functions::{u32_from_i32, wrapping_sub_custom_clamps},
    resource_manager::ResourceManager,
};

use crate::{
    elements::{
        button::{image_button::ImageButton, tiling_text_button::TilingButton},
        misc::text::Text,
        traits::{cast_element, Element as ElementTrait},
        Element,
    },
    events::{Event, EventId},
    syncs::SyncId,
    utils::positioning::UIPosition,
};

use super::traits::ListBox;

#[derive(Clone, Debug, PartialEq, Eq, Copy, Default)]
enum UpDownClickState {
    Up((Instant, Instant)),
    Down((Instant, Instant)),
    #[default]
    None,
}

impl UpDownClickState {
    fn is_update_needed(&self) -> bool {
        use crate::utils::animation_constants::*;
        use UpDownClickState::*;

        match self {
            Up((bind_pressed_instant, last_update_instant))
                if bind_pressed_instant.elapsed() > TIME_BETWEEN_BIND_PRESSED
                    && last_update_instant.elapsed() > TIME_BETWEEN_UPDATES =>
            {
                true
            }
            Down((bind_pressed_instant, last_update_instant))
                if bind_pressed_instant.elapsed() > TIME_BETWEEN_BIND_PRESSED
                    && last_update_instant.elapsed() > TIME_BETWEEN_UPDATES =>
            {
                true
            }
            _ => false,
        }
    }

    fn increment_last_update_instant(&mut self) {
        use UpDownClickState::*;
        *self = match self {
            Up((bind_pressed_instant, _)) => Up((*bind_pressed_instant, Instant::now())),
            Down((bind_pressed_instant, _)) => Down((*bind_pressed_instant, Instant::now())),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpDownScrollListBox {
    global_bounds: IntRect,
    position: UIPosition,
    rerender: bool,
    event_id: EventId,
    sync_id: SyncId,
    scroll_up_button: ImageButton,
    scroll_down_button: ImageButton,
    buttons: Vec<TilingButton>,
    options: Vec<String>,
    current_option_idx: usize,
    up_down_click_state: UpDownClickState,
}

impl UpDownScrollListBox {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        resource_manager: &ResourceManager,
        asset_id: &str,
        position: UIPosition,
        frame_id: usize,
        hover_frame_id: usize,
        click_frame_id: usize,
        mut options: Vec<String>,
        mut number_of_buttons: usize,
        padding: Option<UIPosition>,
        scale: f32,
        font_size: u32,
        font_color: Color,
        event_id: EventId,
        sync_id: SyncId,
    ) -> Self {
        let asset = resource_manager.fetch_asset(asset_id);

        let scroll_up_button = ImageButton::with_texture_bounds(
            resource_manager,
            UIPosition::START_VERTICAL,
            asset_id,
            asset.get_shifted_slice_bound("up", frame_id),
            asset.get_shifted_slice_bound("up", hover_frame_id),
            asset.get_shifted_slice_bound("up", click_frame_id),
            scale,
            0,
            0,
        );
        let scroll_down_button = ImageButton::with_texture_bounds(
            resource_manager,
            UIPosition::END_VERTICAL,
            asset_id,
            asset.get_shifted_slice_bound("down", frame_id),
            asset.get_shifted_slice_bound("down", hover_frame_id),
            asset.get_shifted_slice_bound("down", click_frame_id),
            scale,
            0,
            0,
        );
        let button = TilingButton::new(
            resource_manager,
            Default::default(),
            asset_id,
            frame_id,
            hover_frame_id,
            click_frame_id,
            Element::Div(Div::new(
                Default::default(),
                vec![Element::Text(Text::new(
                    resource_manager,
                    Default::default(),
                    "",
                    true,
                    font_size,
                    font_color,
                    0,
                ))],
                padding,
                None,
            )),
            &Vector2::new(0, 0),
            scale,
            0,
            0,
        );
        let mut buttons = vec![];
        if options.is_empty() {
            error!("Number of options provided is zero! Adding an option to the list.");
            options.push(Default::default());
        }
        if number_of_buttons > options.len() {
            number_of_buttons = options.len();
            warn!("Number of buttons is greater than the number of options provided! Clipping to number of options provided {}", number_of_buttons);
        }
        if number_of_buttons % 2 == 0 {
            number_of_buttons -= 1;
            warn!("Number of buttons is even. Subtracting one number to make it odd. Going from {} to {}", number_of_buttons + 1, number_of_buttons);
        }

        for i in 0..number_of_buttons {
            buttons.push(button.clone());
            if let Element::Text(text) = buttons[i].inner_element_mut() {
                text.set_text(options[i].as_str());
            }
        }

        let mut udsl = Self {
            global_bounds: Default::default(),
            position,
            rerender: false,
            event_id,
            sync_id,
            scroll_up_button,
            scroll_down_button,
            buttons,
            current_option_idx: options.len() / 2,
            options,
            up_down_click_state: Default::default(),
        };
        udsl.update_size();
        udsl
    }

    fn inner_text_element_mut_from_button_at_index(&mut self, index: usize) -> &mut Text {
        let Element::Div(div) = self.buttons.get_mut(index).expect("Index array out of bounds").inner_element_mut() else {
            error!("Inner element inside button is not a Div element!");
            panic!("Inner element inside button is not a Div element!"); 
        };
        let Element::Text(text) = div.mut_children().next().expect("Div does not have child!") else {
            error!("Inner element inside Div is not a Text element!");
            panic!("Inner element inside Div is not a Text element!");
        };

        text
    }

    fn current_option(&self) -> &str {
        &self.options[self.current_option_idx]
    }

    fn actionable_elements_mut(&mut self) -> impl Iterator<Item = &mut dyn ActionableElement> {
        std::iter::once(&mut self.scroll_up_button as _)
            .chain([&mut self.scroll_down_button as _])
            .chain(self.buttons.iter_mut().map(|button| button as _))
    }

    fn actionable_elements(&self) -> impl Iterator<Item = &dyn ActionableElement> {
        std::iter::once(&self.scroll_up_button as _)
            .chain([&self.scroll_down_button as _])
            .chain(self.buttons.iter().map(|button| button as _))
    }

    fn set_button_strings_based_on_current_option_idx(&mut self) {
        // Borrow checker is a bitch
        let mut option_idx = wrapping_sub_custom_clamps(
            self.current_option_idx,
            self.buttons.len() / 2,
            0,
            self.options.len().saturating_sub(1),
        )
        .unwrap();

        let options = self.options.clone();

        for idx in 0..self.buttons.len() {
            self.inner_text_element_mut_from_button_at_index(idx)
                .set_text(&options[option_idx]);
            option_idx = (option_idx + 1) % self.options.len();
        }

        self.rerender = true;
    }
}

impl ElementTrait for UpDownScrollListBox {
    cast_element!();

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        let mut text = self.inner_text_element_mut_from_button_at_index(0).clone();
        let mut biggest_width = 0;
        let mut biggest_string = "".to_string();
        for option in &self.options {
            text.set_text(option);
            let gb_width = text.global_bounds().width;
            if gb_width > biggest_width {
                biggest_width = gb_width;
                biggest_string = option.clone();
            }
        }

        let biggest_width = biggest_width.try_into().unwrap_or_else(|_| {
            error!("Biggest size has a negative number in it. Unable to properly determine size.");
            Default::default()
        });

        let mut gb_size = Vector2::new(0, 0);
        for idx in 0..self.buttons.len() {
            self.inner_text_element_mut_from_button_at_index(idx)
                .set_text(&biggest_string);

            let button_height = u32_from_i32(
                self.inner_text_element_mut_from_button_at_index(idx)
                    .global_bounds()
                    .height,
            );

            let Some(button) = self.buttons.get_mut(idx) else {
              break;
            };

            button.set_desired_size(Vector2::new(biggest_width, button_height));

            gb_size.y += button.global_bounds().height;
            gb_size.x = button.global_bounds().width;
        }

        self.scroll_up_button.update_size();
        gb_size.y += self.scroll_up_button.global_bounds().height;
        self.scroll_down_button.update_size();
        gb_size.y += self.scroll_down_button.global_bounds().height;

        // Give a little buffer room between the scroll buttons and the option buttons
        gb_size.y += 2;

        self.global_bounds.width = gb_size.x;
        self.global_bounds.height = gb_size.y;

        self.set_button_strings_based_on_current_option_idx();

        self.rerender = true;
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        let gb = self.global_bounds;
        let max_button_idx = self.buttons.len() - 1;
        let middle_button = max_button_idx / 2;

        // Position the middle button
        self.buttons[middle_button].set_ui_position(UIPosition::CENTER, gb);

        // Position buttons above middle button
        for button_idx in (0..middle_button).rev() {
            let prev_button_gb = self.buttons[button_idx + 1].global_bounds();
            let ui_position_above_button = UIPosition::position_above_bounds_in_relative_rect(
                self.buttons[button_idx].global_bounds(),
                prev_button_gb,
                gb,
            );
            self.buttons[button_idx].set_ui_position(ui_position_above_button, gb);
        }

        // This will crash if we only have one button
        if middle_button != 0 {
            // Position buttons below middle button
            for button_idx in middle_button..=max_button_idx {
                let prev_button_gb = self.buttons[button_idx - 1].global_bounds();
                let ui_position_below_button =
                    UIPosition::position_below_bounds_in_relative_rect(prev_button_gb, gb);
                self.buttons[button_idx].set_ui_position(ui_position_below_button, gb);
            }
        }

        // position scroll up and down buttons
        self.scroll_up_button.update_position(gb);
        self.scroll_down_button.update_position(gb);

        self.rerender = true;
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_position(relative_rect);
    }

    fn event_handler(
        &mut self,
        ui_settings: &crate::ui_settings::UISettings,
        event: sfml::window::Event,
    ) -> (Vec<Event>, bool) {
        for act_ele in self.actionable_elements_mut() {
            act_ele.event_handler(ui_settings, event);
        }

        match event {
            sfml::window::Event::MouseButtonReleased { .. } => {
                self.up_down_click_state = UpDownClickState::None;
            }
            sfml::window::Event::MouseMoved { .. }
                if !self.scroll_up_button.is_hover() && !self.scroll_down_button.is_hover() =>
            {
                self.up_down_click_state = UpDownClickState::None;
            }
            _ => {}
        }

        ListBox::event_handler(self, ui_settings, event)
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        // Borrow checker is a bitch
        let mut rerender = self.rerender;
        let mut events = Vec::new();
        for ele in self.actionable_elements_mut() {
            let mut event = ele.update(resource_manager);
            rerender |= event.1;
            events.append(&mut event.0);
        }

        if self.up_down_click_state.is_update_needed() {
            use UpDownClickState::*;
            match self.up_down_click_state {
                Up(_) => {
                    rerender = true;
                    self.scroll_up();
                    self.up_down_click_state.increment_last_update_instant();
                    events.push(self.triggered_event());
                }
                Down(_) => {
                    rerender = true;
                    self.scroll_down();
                    self.up_down_click_state.increment_last_update_instant();
                    events.push(self.triggered_event());
                }
                None => {}
            }
        }

        self.rerender |= rerender;
        (events, self.rerender)
    }

    fn render(&mut self, render_texture: &mut sfml::graphics::RenderTexture) {
        for button in &mut self.buttons {
            button.render(render_texture);
        }
        self.scroll_up_button.render(render_texture);
        self.scroll_down_button.render(render_texture);

        self.rerender = false;
    }

    fn sync(&mut self, sync: Syncs) {
        match sync {
            Syncs::Numerical(idx) => {
                let idx = idx as usize;

                if idx < self.options.len() {
                    self.current_option_idx = idx;
                    self.set_button_strings_based_on_current_option_idx();
                }
            }
            Syncs::String(string) => {
                let selected_index = self
                    .options
                    .iter()
                    .position(|option| option == &string)
                    .unwrap_or(self.current_option_idx);

                self.current_option_idx = selected_index;
                self.set_button_strings_based_on_current_option_idx();
            }
            _ => {
                error!("Sync: {:#?} is not valid for up down list box", sync)
            }
        }
    }

    fn sync_id(&self) -> SyncId {
        self.sync_id
    }
}

impl ActionableElement for UpDownScrollListBox {
    fn triggered_event(&self) -> Event {
        Event::new(
            self.event_id,
            Events::StringEvent(self.current_option().to_string()),
        )
    }

    fn bind_pressed(&mut self, _: Vector2i) {
        if self.scroll_up_button.current_mouse_state() == UIMouseStates::Click
            && self.scroll_up_button.is_hover()
            && self.up_down_click_state == UpDownClickState::None
        {
            self.up_down_click_state = UpDownClickState::Up((Instant::now(), Instant::now()));
        }
        if self.scroll_down_button.current_mouse_state() == UIMouseStates::Click
            && self.scroll_down_button.is_hover()
            && self.up_down_click_state == UpDownClickState::None
        {
            self.up_down_click_state = UpDownClickState::Down((Instant::now(), Instant::now()));
        }
    }

    fn bind_released(&mut self, _: Vector2i) {
        self.up_down_click_state = UpDownClickState::None;
        if !self.is_hover() {
            return;
        }

        match (
            self.scroll_up_button.is_hover(),
            self.scroll_down_button.is_hover(),
        ) {
            // scroll up button got selected
            (true, false) => self.scroll_up(),
            // scroll down button got selected
            (false, true) => self.scroll_down(),
            // one of the listboxes got selected
            _ => {
                let selected_button_idx =
                    self.buttons.iter().position(|b| b.is_hover()).unwrap_or(0);
                let middle_button_idx = (self.buttons.len() + 2 - 1) / 2;

                let idx_diff = middle_button_idx as i32 - selected_button_idx as i32 - 1;
                if idx_diff.is_positive() {
                    for _ in 0..idx_diff {
                        self.scroll_up();
                    }
                } else if idx_diff.is_negative() {
                    for _ in (0..idx_diff.abs()).rev() {
                        self.scroll_down();
                    }
                }
            }
        }
    }

    fn set_hover(&mut self, mouse_pos: Vector2i) {
        for act_ele in self.actionable_elements_mut() {
            act_ele.set_hover(mouse_pos);
        }
        self.rerender = true;
    }

    fn is_hover(&self) -> bool {
        let mut is_hover = false;
        for act_ele in self.actionable_elements() {
            is_hover |= act_ele.is_hover();
        }

        is_hover
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }

    cast_actionable_element!();
}

impl ListBox for UpDownScrollListBox {
    fn scroll_up(&mut self) {
        self.current_option_idx = self
            .current_option_idx
            .checked_sub(1)
            .unwrap_or(self.options.len() - 1);
        self.set_button_strings_based_on_current_option_idx();
    }

    fn scroll_down(&mut self) {
        self.current_option_idx = (self.current_option_idx + 1) % self.options.len();
        self.set_button_strings_based_on_current_option_idx();
    }

    fn box_clone(&self) -> Box<dyn ListBox> {
        Box::new(self.clone())
    }
}
