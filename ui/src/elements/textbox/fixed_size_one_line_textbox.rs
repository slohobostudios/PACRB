use std::time::Instant;

use sfml::{
    graphics::{Color, IntRect, RcText, RectangleShape, RenderTarget, Shape, Transformable},
    system::{Vector2, Vector2i},
    window::{clipboard, Event as SFMLEvent},
};
use tracing::error;
use utils::{
    arithmetic_util_functions::i32_from_u32,
    resource_manager::ResourceManager,
    sfml_util_functions::{get_character_idx_of_rc_text_at_point, glyph_from_rc_text},
};

use crate::{
    elements::{
        text::Text,
        traits::{cast_actionable_element, cast_element, ActionableElement, Element},
    },
    events::{Event, EventId, Events, EMPTY_EVENT},
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};

use super::traits::{TextBox, BLINK_INTERVAL, CURSOR_CHAR, CURSOR_FONT};

const START_HORIZONTAL_OFFSET: UIPosition = UIPosition {
    left: Some(5),
    ..UIPosition::CENTER
};

macro_rules! no_text_in_textbox {
    ( $self:ident, $($code:tt)* ) => {
        let no_text_in_textbox = $self.string.is_empty();
        if no_text_in_textbox {
            $self.rendered_text
                .set_text(&((32 as char)..=(126 as char)).collect::<String>())
        }
        $($code)*;

        if no_text_in_textbox {
            $self.rendered_text.set_text("");
        }
    };
}

#[derive(Debug, Clone)]
pub struct FixedSizeOneLineTextbox {
    global_bounds: IntRect,
    position: UIPosition,
    background_rect: RectangleShape<'static>,
    rendered_text: Text,
    string: String,
    cursor_idx: usize,
    starting_idx: usize,
    event_id: EventId,
    sync_id: EventId,
    hover: bool,
    selected: bool,
    rerender: bool,
    instant_since_cursor_blink: Instant,
    display_cursor: bool,
    cursor: RcText,
    bind_pressed_location: Option<Vector2i>,
    select_start_idx: Option<usize>,
    select_end_idx: Option<usize>,
}

impl FixedSizeOneLineTextbox {
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        width: u16,
        font_size: u32,
        text_color: Color,
        background_color: Color,
        default_text: &str,
        event_id: EventId,
        sync_id: EventId,
    ) -> Self {
        let mut background_rect = RectangleShape::with_size(Vector2::new(width as f32, 0.));
        background_rect.set_fill_color(background_color);
        let mut cursor = RcText::new(
            &CURSOR_CHAR.to_string(),
            resource_manager.fetch_font_with_id(CURSOR_FONT),
            font_size,
        );
        cursor.set_fill_color(text_color);
        let mut fstb = Self {
            global_bounds: IntRect::new(0, 0, width.into(), 0),
            position,
            background_rect,
            string: default_text.to_string(),
            rendered_text: Text::new(
                resource_manager,
                START_HORIZONTAL_OFFSET,
                default_text,
                true,
                font_size,
                text_color,
            ),
            sync_id,
            event_id,
            hover: false,
            selected: false,
            rerender: true,
            cursor_idx: default_text.len().saturating_sub(1),
            starting_idx: 0,
            instant_since_cursor_blink: Instant::now(),
            display_cursor: false,
            cursor,
            bind_pressed_location: None,
            select_start_idx: None,
            select_end_idx: None,
        };
        fstb.update_size();

        fstb
    }

    fn is_text_too_big(&self) -> bool {
        let width = self.rendered_text.global_bounds().width;
        let cap = self.global_bounds.width - START_HORIZONTAL_OFFSET.left.unwrap_or(0) * 2;
        width > cap
    }

    fn cursor_update(&mut self) {
        let change_cursor = if self.selected {
            self.instant_since_cursor_blink.elapsed() > BLINK_INTERVAL
        } else {
            self.display_cursor
        };
        if change_cursor {
            self.display_cursor = !self.display_cursor;
            self.cursor.set_position(
                self.rendered_text
                    .rc_text()
                    .find_character_pos(self.cursor_idx.saturating_sub(self.starting_idx)),
            );
            self.instant_since_cursor_blink = Instant::now();
        }
    }

    fn make_cursor_disappear(&mut self) {
        let selected = self.selected;
        self.selected = false;
        self.cursor_update();
        self.selected = selected;
    }

    fn make_cursor_appear(&mut self) {
        self.instant_since_cursor_blink = Instant::now() - 2 * BLINK_INTERVAL;
        self.display_cursor = false;
        self.cursor_update();
    }

    fn get_cursor_idx_of_point_in_space_with_side_clamping(&self, point: Vector2i) -> usize {
        if let Some(cursor_idx) = get_character_idx_of_rc_text_at_point(
            self.rendered_text.rc_text(),
            point,
            false,
            false,
            true,
            true,
        ) {
            self.starting_idx + cursor_idx
        } else if point.x <= self.rendered_text.global_bounds().left {
            self.starting_idx
        } else {
            self.starting_idx + self.rendered_text.text().len().saturating_sub(1)
        }
    }

    fn get_character_idx_of_rc_text_at_point_fully_clamped(
        &self,
        point: Vector2i,
    ) -> Option<usize> {
        get_character_idx_of_rc_text_at_point(
            self.rendered_text.rc_text(),
            point,
            true,
            true,
            true,
            true,
        )
    }
}

impl Element for FixedSizeOneLineTextbox {
    cast_element!();

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        TextBox::event_handler(self, ui_settings, event)
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }
    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn global_bounds(&self) -> sfml::graphics::IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        no_text_in_textbox!(self,
            self.global_bounds.height = self.rendered_text.global_bounds().height
                + i32_from_u32(self.rendered_text.global_bounds().height.ilog2());
            self.background_rect
                .set_size(self.global_bounds.size().as_other())
        );
    }

    fn update_position(&mut self, relative_rect: sfml::graphics::IntRect) {
        let no_text_in_textbox = self.string.is_empty();
        if no_text_in_textbox {
            self.rendered_text
                .set_text(&((32 as char)..=(126 as char)).collect::<String>())
        }

        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        self.background_rect
            .set_position(self.global_bounds.position().as_other());
        self.rendered_text.update_position(self.global_bounds);

        if no_text_in_textbox {
            self.rendered_text.set_text("");
        }
    }

    fn set_ui_position(
        &mut self,
        ui_position: crate::utils::positioning::UIPosition,
        relative_rect: sfml::graphics::IntRect,
    ) {
        self.position = ui_position;
        self.update_position(relative_rect);
    }

    fn update(&mut self, _resource_manager: &ResourceManager) -> Vec<Event> {
        self.cursor_update();
        if self.rerender {
            vec![EMPTY_EVENT]
        } else {
            vec![]
        }
    }

    fn render(&mut self, render_texture: &mut sfml::graphics::RenderTexture) {
        render_texture.draw(&self.background_rect);
        self.rendered_text.render(render_texture);
        if self.display_cursor {
            render_texture.draw(&self.cursor);
        }
        self.rerender = false;
    }

    fn box_clone(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }
}

impl ActionableElement for FixedSizeOneLineTextbox {
    cast_actionable_element!();

    fn triggered_event(&self) -> Event {
        Event::new(self.event_id, Events::StringEvent(self.string.clone()))
    }

    fn bind_pressed(&mut self, mouse_pos: sfml::system::Vector2i) {
        self.set_hover(mouse_pos);
        self.selected = self.hover;

        if !self.hover {
            return;
        }
        self.rerender = true;
        self.bind_pressed_location = Some(mouse_pos);
        self.move_cursor(self.get_cursor_idx_of_point_in_space_with_side_clamping(mouse_pos));
        self.make_cursor_appear();
    }

    fn bind_released(&mut self, mouse_pos: sfml::system::Vector2i) {
        self.set_hover(mouse_pos);

        if self.hover || self.is_dragging() {
            self.rerender = true;
        }
        if self.is_dragging() {
            self.drag_mouse(mouse_pos);
            self.bind_pressed_location = None;
            self.select_start_idx = None;
            self.select_end_idx = None;
        }
    }

    fn set_hover(&mut self, mouse_pos: sfml::system::Vector2i) {
        self.hover = self.global_bounds.contains(mouse_pos);
    }

    fn is_hover(&self) -> bool {
        self.hover
    }
}

impl TextBox for FixedSizeOneLineTextbox {
    fn drag_mouse(&mut self, mouse_pos: Vector2i) {
        if !self.is_dragging() {
            return;
        }
        self.make_cursor_appear();
        self.rerender = true;
        let Some( glyph )= glyph_from_rc_text(self.rendered_text.rc_text(), 'A' as u32) else {
            return;
        };
        if self.select_end_idx.is_none() {
            let Some(start_mouse_pos) = self.bind_pressed_location else {
                error!("self.bind_pressed_location is none!");
                return;
            };
            if let (Some(start_idx), Some(end_idx)) = (
                self.get_character_idx_of_rc_text_at_point_fully_clamped(start_mouse_pos),
                self.get_character_idx_of_rc_text_at_point_fully_clamped(mouse_pos),
            ) {
                if start_idx != end_idx {
                    self.select_start_idx = Some(self.starting_idx + start_idx);
                    self.select_end_idx = Some(self.starting_idx + end_idx);
                    return;
                }
            }
            if (start_mouse_pos.x - mouse_pos.x).abs() > glyph.bounds().width as i32 {
                if let Some(start_idx) =
                    self.get_character_idx_of_rc_text_at_point_fully_clamped(mouse_pos)
                {
                    self.select_start_idx = Some(self.starting_idx + start_idx);
                    self.select_end_idx = Some(self.starting_idx + start_idx);
                }
            }

            return;
        }

        let adjusted_cursor_idx = self
            .get_character_idx_of_rc_text_at_point_fully_clamped(mouse_pos)
            .unwrap_or(self.cursor_idx);
        if adjusted_cursor_idx == 0 {
            self.move_cursor_left();
        } else if adjusted_cursor_idx >= self.rendered_text.text().len().saturating_sub(1) {
            self.move_cursor_right();
        } else {
            self.move_cursor(self.starting_idx + adjusted_cursor_idx);
        }
        self.select_end_idx = Some(self.cursor_idx);
    }

    fn is_dragging(&self) -> bool {
        self.bind_pressed_location.is_some()
    }

    fn utf32_str(&self) -> &str {
        &self.string
    }

    fn ascii_str(&self) -> Option<String> {
        if self.string.is_ascii() {
            Some(self.string.to_ascii_lowercase())
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<dyn TextBox> {
        Box::new(self.clone())
    }

    fn text_entered(&mut self, event: SFMLEvent) {
        if let SFMLEvent::TextEntered { unicode } = event {
            // Back space
            if unicode == 0x08 as char {
                if self.cursor_idx != 0 {
                    if self.cursor_idx >= self.string.len() {
                        self.string.pop();
                    } else {
                        self.string.remove(self.cursor_idx);
                    }
                } else if !self.string.is_empty() {
                    self.string.remove(0);
                } else {
                    self.string = String::new();
                }
                self.move_cursor_left();
            }
            // Delete
            else if unicode == 0x7f as char {
                if let Some(_) = self.string.chars().nth(self.cursor_idx + 1) {
                    self.string.remove(self.cursor_idx + 1);
                    self.move_cursor(self.cursor_idx);
                }
            }
            // Ignore return carriage
            else if unicode == 0xd as char || unicode == 0xa as char {
            }
            // Ignore ctrl+v/ctrl+v generated chars
            else if unicode != 0x16 as char && unicode != 0x03 as char {
                if self.cursor_idx >= self.string.len().saturating_sub(1) {
                    self.string.push(unicode);
                } else {
                    self.string.insert(self.cursor_idx, unicode);
                }
                self.move_cursor_right();
            }
            self.rerender = true;
        } else {
            error!("Event is not a TextEntered event! {:#?}", event)
        }
    }

    fn is_selected(&self) -> bool {
        self.selected
    }

    fn move_cursor_left(&mut self) {
        self.move_cursor(self.cursor_idx.saturating_sub(1))
    }

    fn move_cursor_right(&mut self) {
        self.move_cursor(self.cursor_idx.saturating_add(1))
    }

    fn move_cursor(&mut self, new_cursor_idx: usize) {
        self.rerender = true;
        self.make_cursor_disappear();

        let mut check_sub_failed = false;
        let adjusted_cursor_idx = new_cursor_idx
            .checked_sub(self.starting_idx)
            .unwrap_or_else(|| {
                check_sub_failed = true;
                0
            });
        self.cursor_idx = if adjusted_cursor_idx >= self.rendered_text.text().len() {
            // Move everything right
            let end = new_cursor_idx.min(self.string.len());
            let mut start = end;

            self.rendered_text.set_text(&self.string[start..end]);
            while start > 0 && !self.is_text_too_big() {
                start -= 1;
                self.rendered_text.set_text(&self.string[start..end]);
            }

            if self.is_text_too_big() {
                start += 1;
                self.rendered_text.set_text(&self.string[start..end]);
            }
            self.starting_idx = start;

            end
        } else if adjusted_cursor_idx == 0 && check_sub_failed {
            // Move everything left
            let start = new_cursor_idx;
            let mut end = start;
            self.rendered_text.set_text(&self.string[start..end]);
            while end < self.string.len() && !self.is_text_too_big() {
                end += 1;
                self.rendered_text.set_text(&self.string[start..end]);
            }

            if self.is_text_too_big() {
                end -= 1;
                self.rendered_text.set_text(&self.string[start..end]);
            }
            self.starting_idx = start;
            start
        } else {
            // Only move cursor
            let start = self.starting_idx;
            let end = self
                .string
                .len()
                .min(start + self.rendered_text.text().len());
            self.rendered_text.set_text(&self.string[start..end]);
            new_cursor_idx
        };

        self.make_cursor_appear();
    }

    fn deselect(&mut self) {
        self.make_cursor_disappear();
        self.selected = false;
    }

    fn copy(&self) -> String {
        let Some(min_idx) = self.select_start_idx.min(self.select_end_idx) else { 
            return "".to_string(); 
        };
        let Some(max_idx) = self.select_start_idx.max(self.select_end_idx) else { 
            return "".to_string(); 
        };
        let max_idx = max_idx.min(self.string.len());

        self.string[min_idx..max_idx].to_string()
    }

    fn paste(&mut self) {
        for unicode in clipboard::get_string().chars() {
            self.text_entered(SFMLEvent::TextEntered { unicode });
        }
    }
}

impl Default for FixedSizeOneLineTextbox {
    fn default() -> Self {
        Self {
            global_bounds: Default::default(),
            position: Default::default(),
            background_rect: Default::default(),
            rendered_text: Default::default(),
            string: Default::default(),
            cursor_idx: Default::default(),
            event_id: Default::default(),
            sync_id: Default::default(),
            hover: Default::default(),
            selected: Default::default(),
            rerender: Default::default(),
            display_cursor: Default::default(),
            starting_idx: Default::default(),
            cursor: Default::default(),
            bind_pressed_location: None,
            select_start_idx: None,
            select_end_idx: None,
            instant_since_cursor_blink: Instant::now(),
        }
    }
}
