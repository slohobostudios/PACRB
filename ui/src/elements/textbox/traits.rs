use sfml::{
    system::Vector2i,
    window::{Event as SFMLEvent, Key},
};

use crate::{
    elements::traits::ActionableElement,
    events::Event,
    ui_settings::{
        controls::{possible_binds::PossibleBinds, possible_inputs::PossibleInputs},
        UISettings,
    },
};

use std::{fmt::Debug, ops::Deref, time::Duration};

pub(super) const CURSOR_FONT: &str = "SourceCodePro-SemiBold.ttf";
pub(super) const CURSOR_CHAR: char = '\u{2581}';
pub(super) const BLINK_INTERVAL: Duration = Duration::from_millis(900);

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TextBoxTriggeredEvent {
    pub string: String,
    pub selected: bool,
}

pub trait TextBox: ActionableElement + Debug {
    fn move_cursor(&mut self, new_cursor_idx: usize);
    fn move_cursor_left(&mut self);
    fn move_cursor_right(&mut self);
    fn box_clone(&self) -> Box<dyn TextBox>;
    fn text_entered(&mut self, event: SFMLEvent);
    fn select_everything(&mut self);
    fn make_select_box_dissappear(&mut self);
    fn deselect(&mut self);
    fn is_selected(&self) -> bool;
    fn is_dragging(&self) -> bool;
    fn cut(&mut self);
    fn copy(&self);
    fn paste(&mut self);
    fn drag_mouse(&mut self, mouse_pos: Vector2i);
    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        self.set_hover(ui_settings.cursor_position);
        match event {
            // Escape
            SFMLEvent::KeyPressed { code, .. }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Escape,
                ) =>
            {
                self.deselect();
                (vec![self.triggered_event()], true)
            }
            // Cut, Copy, and Paste
            // This will be hardcoded per system. No binds for this

            // Cut
            SFMLEvent::KeyPressed { code, ctrl, .. } if code == Key::X && ctrl => {
                self.cut();
                (vec![self.triggered_event()], true)
            }

            // Copy
            SFMLEvent::KeyPressed { code, ctrl, .. } if code == Key::C && ctrl => {
                self.copy();
                (vec![], true)
            }

            // Paste
            SFMLEvent::KeyPressed { code, ctrl, .. } if code == Key::V && ctrl => {
                self.paste();
                (vec![self.triggered_event()], true)
            }

            // Select Everythinhg
            SFMLEvent::KeyPressed { code, ctrl, .. }
                if code == Key::A && ctrl && self.is_selected() =>
            {
                self.select_everything();
                (vec![], true)
            }

            // Mouse dragging and selection
            SFMLEvent::MouseMoved { x: _, y: _ } if self.is_dragging() => {
                self.drag_mouse(ui_settings.cursor_position);
                (vec![], true)
            }
            SFMLEvent::MouseButtonPressed { button, x: _, y: _ }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) =>
            {
                let original_selected_state = self.is_selected();
                self.bind_pressed(ui_settings.cursor_position);
                if original_selected_state ^ self.is_selected() {
                    (vec![self.triggered_event()], true)
                } else {
                    (vec![], false)
                }
            }
            SFMLEvent::MouseButtonReleased { button, x: _, y: _ }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) =>
            {
                self.bind_released(ui_settings.cursor_position);
                (vec![], self.is_dragging())
            }
            SFMLEvent::KeyPressed { code, .. }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::UILeft,
                ) =>
            {
                self.move_cursor_left();
                self.make_select_box_dissappear();
                (vec![], true)
            }
            SFMLEvent::KeyPressed { code, .. }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::UIRight,
                ) =>
            {
                self.move_cursor_right();
                self.make_select_box_dissappear();
                (vec![], true)
            }
            SFMLEvent::KeyPressed { code, .. }
                if ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Select,
                ) =>
            {
                let original_selected_state = self.is_selected();
                self.bind_pressed(ui_settings.cursor_position);
                if original_selected_state ^ self.is_selected() {
                    (vec![self.triggered_event()], true)
                } else {
                    (vec![], false)
                }
            }
            SFMLEvent::KeyReleased { code, .. }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Select,
                ) =>
            {
                self.bind_released(ui_settings.cursor_position);
                (vec![], self.is_dragging())
            }
            SFMLEvent::TextEntered { unicode: _ } if self.is_selected() => {
                self.text_entered(event);
                (vec![self.triggered_event()], true)
            }
            // if it is selected, and a key is being pressed, more than likely, text is being entered
            SFMLEvent::KeyPressed { .. } | SFMLEvent::KeyReleased { .. } if self.is_selected() => {
                (vec![self.triggered_event()], false)
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
