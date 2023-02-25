use crate::ui::{
    elements::traits::Element,
    events::*,
    ui_settings::{
        controls::{possible_binds::PossibleBinds, possible_inputs::PossibleInputs},
        UISettings,
    },
    utils::mouse_ui_states::UIMouseStates,
};
use sfml::system::Vector2i;
use sfml::window::Event as SFMLEvent;
use std::ops::Deref;

pub trait Button {
    fn triggered_event(&self) -> Event;
    fn current_mouse_state(&self) -> UIMouseStates;
    fn bind_pressed(&mut self, mouse_pos: Vector2i);
    fn bind_released(&mut self, mouse_pos: Vector2i);
    fn set_hover(&mut self, mouse_pos: Vector2i);
    fn is_hover(&self) -> bool;
    fn box_clone(&self) -> Box<dyn Button>;
}

impl Clone for Box<dyn Button> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

use std::fmt::Debug;
pub trait ButtonElement: Button + Element + Debug {
    fn as_mut_element(&mut self) -> &mut dyn Element;
    fn as_mut_button(&mut self) -> &mut dyn Button;
    fn as_element(&self) -> &dyn Element;
    fn as_button(&self) -> &dyn Button;
    fn box_clone(&self) -> Box<dyn ButtonElement>;
    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        self.set_hover(ui_settings.cursor_position);
        match event {
            SFMLEvent::MouseButtonPressed { button, x: _, y: _ }
                if self.is_hover()
                    && ui_settings.binds.is_bind_pressed_and_binded(
                        PossibleInputs::from(button),
                        PossibleBinds::Select,
                    ) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                Vec::from([EMPTY_EVENT])
            }
            SFMLEvent::MouseButtonReleased { button, x: _, y: _ }
                if self.is_hover()
                    && ui_settings.binds.is_bind_released_and_binded(
                        PossibleInputs::from(button),
                        PossibleBinds::Select,
                    ) =>
            {
                self.bind_released(ui_settings.cursor_position);
                Vec::from([self.triggered_event()])
            }
            SFMLEvent::MouseMoved { x: _, y: _ }
                if self.is_hover() && ui_settings.binds.is_bind_pressed(PossibleBinds::Select) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                Vec::from([EMPTY_EVENT])
            }
            SFMLEvent::KeyPressed {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if self.is_hover()
                && ui_settings.binds.is_bind_pressed_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Select,
                ) =>
            {
                self.bind_pressed(ui_settings.cursor_position);
                Vec::from([EMPTY_EVENT])
            }
            SFMLEvent::KeyReleased {
                code,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } if self.is_hover()
                && ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Select,
                ) =>
            {
                self.bind_released(ui_settings.cursor_position);
                Vec::from([self.triggered_event()])
            }
            _ if self.is_hover() => Vec::from([EMPTY_EVENT]),
            _ => Default::default(),
        }
    }
}

impl Clone for Box<dyn ButtonElement> {
    fn clone(&self) -> Self {
        ButtonElement::box_clone(self.deref())
    }
}
