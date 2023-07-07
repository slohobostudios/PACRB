use crate::{
    elements::traits::ActionableElement,
    events::*,
    ui_settings::{
        controls::{possible_binds::PossibleBinds, possible_inputs::PossibleInputs},
        UISettings,
    },
    utils::mouse_ui_states::UIMouseStates,
};
use sfml::window::Event as SFMLEvent;
use std::{fmt::Debug, ops::Deref};

pub trait Button: ActionableElement + Debug {
    fn box_clone(&self) -> Box<dyn Button>;
    fn current_mouse_state(&self) -> UIMouseStates;
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
            SFMLEvent::MouseMoved { .. }
                if self.is_hover() && ui_settings.binds.is_bind_pressed(PossibleBinds::Select) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                (Default::default(), true)
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
            _ if self.is_hover() => (Default::default(), true),
            _ => Default::default(),
        }
    }

    fn as_mut_button(&mut self) -> &mut dyn Button
    where
        Self: Sized,
    {
        self
    }
}

impl Clone for Box<dyn Button> {
    fn clone(&self) -> Self {
        Button::box_clone(self.deref())
    }
}
