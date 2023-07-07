use crate::{
    elements::traits::ActionableElement,
    events::Event,
    ui_settings::{
        controls::{possible_binds::PossibleBinds, possible_inputs::PossibleInputs},
        UISettings,
    },
};
use sfml::window::Event as SFMLEvent;
use std::{fmt::Debug, ops::Deref};

pub trait ListBox: ActionableElement + Debug {
    fn box_clone(&self) -> Box<dyn ListBox>;
    fn scroll_up(&mut self) {}
    fn scroll_down(&mut self) {}

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        self.set_hover(ui_settings.cursor_position);
        match event {
            SFMLEvent::MouseButtonPressed { button, .. }
                if self.is_hover()
                    && ui_settings.binds.is_bind_pressed_and_binded(
                        PossibleInputs::from(button),
                        PossibleBinds::Select,
                    ) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                (Default::default(), true)
            }
            SFMLEvent::MouseButtonReleased { button, .. }
                if self.is_hover()
                    && ui_settings.binds.is_bind_released_and_binded(
                        PossibleInputs::from(button),
                        PossibleBinds::Select,
                    ) =>
            {
                self.bind_released(ui_settings.cursor_position);
                (vec![self.triggered_event()], true)
            }
            SFMLEvent::MouseMoved { x: _, y: _ }
                if self.is_hover() && ui_settings.binds.is_bind_pressed(PossibleBinds::Select) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                (vec![self.triggered_event()], true)
            }
            SFMLEvent::KeyPressed { code, .. }
                if self.is_hover()
                    && ui_settings.binds.is_bind_pressed_and_binded(
                        PossibleInputs::from(code),
                        PossibleBinds::Select,
                    ) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                (Default::default(), true)
            }
            SFMLEvent::KeyReleased { code, .. }
                if self.is_hover()
                    && ui_settings.binds.is_bind_released_and_binded(
                        PossibleInputs::from(code),
                        PossibleBinds::Select,
                    ) =>
            {
                self.bind_released(ui_settings.cursor_position);
                (vec![self.triggered_event()], true)
            }
            _ => (vec![], false),
        }
    }
}

impl Clone for Box<dyn ListBox> {
    fn clone(&self) -> Self {
        ListBox::box_clone(self.deref())
    }
}
