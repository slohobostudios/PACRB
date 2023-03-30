use sfml::{
    system::Vector2i,
    window::{Event as SFMLEvent, Key},
};

use crate::{
    elements::traits::ActionableElement,
    events::{Event, EMPTY_EVENT},
    ui_settings::{
        controls::{possible_binds::PossibleBinds, possible_inputs::PossibleInputs},
        UISettings,
    },
};

use std::{fmt::Debug, ops::Deref, time::Duration};

pub(super) const CURSOR_FONT: &'static str = "SourceCodePro-SemiBold.ttf";
pub(super) const CURSOR_CHAR: char = '\u{2581}';
pub(super) const BLINK_INTERVAL: Duration = Duration::from_millis(900);

pub trait TextBox: ActionableElement + Debug {
    fn move_cursor(&mut self, new_cursor_idx: usize);
    fn move_cursor_left(&mut self);
    fn move_cursor_right(&mut self);
    fn utf32_str(&self) -> &str;
    fn ascii_str(&self) -> Option<String>;
    fn box_clone(&self) -> Box<dyn TextBox>;
    fn text_entered(&mut self, event: SFMLEvent);
    fn deselect(&mut self);
    fn is_selected(&self) -> bool;
    fn is_dragging(&self) -> bool;
    fn copy(&self) -> String;
    fn paste(&mut self);
    fn drag_mouse(&mut self, mouse_pos: Vector2i);
    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        self.set_hover(ui_settings.cursor_position);
        match event {
            // Escape
            SFMLEvent::KeyPressed {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if ui_settings
                .binds
                .is_bind_pressed_and_binded(PossibleInputs::from(code), PossibleBinds::Escape) =>
            {
                self.deselect();
                vec![EMPTY_EVENT]
            }
            // Copy and paste
            // This will be hardcoded per system. No binds for this
            SFMLEvent::KeyPressed {
                code,
                alt: _,
                ctrl,
                shift: _,
                system: _,
            } if code == Key::C && ctrl => {
                self.copy();
                vec![EMPTY_EVENT]
            }
            SFMLEvent::KeyPressed {
                code,
                alt: _,
                ctrl,
                shift: _,
                system: _,
            } if code == Key::V && ctrl => {
                self.paste();
                vec![self.triggered_event()]
            }
            // Mouse dragging and selection
            SFMLEvent::MouseMoved { x: _, y: _ } if self.is_dragging() => {
                self.drag_mouse(ui_settings.cursor_position);
                vec![EMPTY_EVENT]
            }
            SFMLEvent::MouseButtonPressed { button, x: _, y: _ }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                vec![EMPTY_EVENT]
            }
            SFMLEvent::MouseButtonReleased { button, x: _, y: _ }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) =>
            {
                self.bind_released(ui_settings.cursor_position);
                vec![EMPTY_EVENT]
            }
            SFMLEvent::KeyPressed {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if ui_settings
                .binds
                .is_bind_pressed_and_binded(PossibleInputs::from(code), PossibleBinds::UILeft) =>
            {
                self.move_cursor_left();
                vec![EMPTY_EVENT]
            }
            SFMLEvent::KeyPressed {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if ui_settings
                .binds
                .is_bind_pressed_and_binded(PossibleInputs::from(code), PossibleBinds::UIRight) =>
            {
                self.move_cursor_right();
                vec![EMPTY_EVENT]
            }
            SFMLEvent::KeyPressed {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if ui_settings
                .binds
                .is_bind_pressed_and_binded(PossibleInputs::from(code), PossibleBinds::Select) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                vec![EMPTY_EVENT]
            }
            SFMLEvent::KeyReleased {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if ui_settings
                .binds
                .is_bind_released_and_binded(PossibleInputs::from(code), PossibleBinds::Select) =>
            {
                self.bind_released(ui_settings.cursor_position);
                vec![EMPTY_EVENT]
            }
            SFMLEvent::TextEntered { unicode: _ } if self.is_selected() => {
                self.text_entered(event);
                vec![self.triggered_event()]
            }
            _ => Default::default(),
        }
    }
}

impl Clone for Box<dyn TextBox> {
    fn clone(&self) -> Self {
        TextBox::box_clone(self.deref())
    }
}
