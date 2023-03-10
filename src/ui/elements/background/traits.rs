use crate::{
    assets::resource_manager::ResourceManager,
    ui::{
        elements::{traits::Element as ElementTrait, Element},
        events::*,
        ui_settings::{
            controls::{possible_binds::*, possible_inputs::*},
            UISettings,
        },
    },
};
use sfml::{graphics::RenderTexture, system::Vector2i, window::Event as SFMLEvent};

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
    fn as_mut_element(&mut self) -> &mut dyn ElementTrait;
    fn as_mut_background(&mut self) -> &mut dyn Background;
    fn as_element(&self) -> &dyn ElementTrait;
    fn as_background(&self) -> &dyn Background;
    fn box_clone(&self) -> Box<dyn BackgroundElement>;

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        let mut events = Vec::new();
        for ele in self.mut_children() {
            events.append(&mut ele.event_handler(&ui_settings, event));
        }

        self.set_hover(ui_settings.cursor_position);

        events
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

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = Vec::new();
        for ele in self.mut_children() {
            events.append(&mut ele.update(&resource_manager));
        }
        events
    }
}

use std::ops::Deref;
impl Clone for Box<dyn BackgroundElement> {
    fn clone(&self) -> Self {
        BackgroundElement::box_clone(self.deref())
    }
}
