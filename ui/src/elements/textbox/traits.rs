use sfml::window::Event as SFMLEvent;

use crate::{
    elements::traits::ActionableElement,
    events::{Event, EMPTY_EVENT},
    ui_settings::{
        controls::{possible_binds::PossibleBinds, possible_inputs::PossibleInputs},
        UISettings,
    },
};

use std::{fmt::Debug, ops::Deref};

pub trait TextBox: ActionableElement + Debug {
    fn utf32_str(&self) -> &str;
    fn ascii_str(&self) -> Option<String>;
    fn box_clone(&self) -> Box<dyn TextBox>;
    fn text_entered(&mut self, event: SFMLEvent);
    fn is_selected(&self) -> bool;
    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        self.set_hover(ui_settings.cursor_position);
        match event {
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
            SFMLEvent::TextEntered { unicode: _ } => {
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
