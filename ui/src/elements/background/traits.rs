use crate::{
    elements::{traits::Element as ElementTrait, Element},
    events::*,
    ui_settings::{
        controls::{possible_binds::*, possible_inputs::*},
        UISettings,
    },
};
use sfml::{graphics::RenderTexture, system::Vector2i, window::Event as SFMLEvent};
use utils::resource_manager::ResourceManager;

pub trait Background {
    fn is_hover(&self) -> bool;
    fn set_hover(&mut self, mouse_pos: Vector2i);
    fn children(&self) -> Box<dyn Iterator<Item = &Element> + '_>;
    fn mut_children(&mut self) -> Box<dyn Iterator<Item = &mut Element> + '_>;
    fn box_clone(&self) -> Box<dyn Background>;
}

impl Clone for Box<dyn Background> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

use std::fmt::Debug;
pub trait BackgroundElement: Background + ElementTrait + Debug {
    fn box_clone(&self) -> Box<dyn BackgroundElement>;

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        let mut rerender = false;
        let mut events = Vec::new();
        for ele in self.mut_children() {
            let mut event = ele.event_handler(ui_settings, event);
            events.append(&mut event.0);
            rerender |= event.1;
        }

        self.set_hover(ui_settings.cursor_position);
        match event {
            SFMLEvent::MouseMoved { .. } if self.is_hover() => {
                rerender |= false;
                events.push(EMPTY_EVENT);
            }
            SFMLEvent::MouseButtonReleased { button, .. }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) && self.global_bounds().contains(ui_settings.cursor_position) =>
            {
                rerender = true;
                events.push(EMPTY_EVENT);
            }
            SFMLEvent::MouseButtonPressed { button, .. }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(button),
                    PossibleBinds::Select,
                ) && self.global_bounds().contains(ui_settings.cursor_position) =>
            {
                rerender = true;
                events.push(EMPTY_EVENT);
            }
            SFMLEvent::KeyReleased { code, .. }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Select,
                ) && self.global_bounds().contains(ui_settings.cursor_position) =>
            {
                rerender = true;
                events.push(EMPTY_EVENT);
            }
            SFMLEvent::KeyPressed { code, .. }
                if ui_settings.binds.is_bind_released_and_binded(
                    PossibleInputs::from(code),
                    PossibleBinds::Select,
                ) && self.global_bounds().contains(ui_settings.cursor_position) =>
            {
                rerender = true;
                events.push(EMPTY_EVENT);
            }
            _ => {}
        }

        (events, rerender)
    }

    fn render(&mut self, window: &mut RenderTexture) {
        for ele in self.mut_children() {
            ele.render(window);
        }
    }

    fn update_element_size(&mut self) {
        for ele in self.mut_children() {
            ele.update_size();
        }
    }

    fn update_element_position(&mut self) {
        let relative_rect = self.global_bounds();
        for ele in self.mut_children() {
            ele.update_position(relative_rect);
        }
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        let mut rerender = false;
        let mut events = Vec::new();
        for ele in self.mut_children() {
            let mut event = ele.update(resource_manager);
            rerender |= event.1;
            events.append(&mut event.0);
        }
        (events, rerender)
    }
}

use std::ops::Deref;
impl Clone for Box<dyn BackgroundElement> {
    fn clone(&self) -> Self {
        BackgroundElement::box_clone(self.deref())
    }
}
