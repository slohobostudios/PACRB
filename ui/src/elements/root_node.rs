use super::{traits::Element as ElementTrait, Element};
use crate::{
    assets::resource_manager::ResourceManager,
    ui::{events::*, ui_settings::UISettings, utils::positioning::UIPosition},
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    window::Event as SFMLEvent,
};

#[derive(Default, Clone, Debug)]
pub struct RootNode {
    children: Vec<Element>,
    relative_rect: IntRect,
}

impl RootNode {
    pub fn new(
        resource_manager: &ResourceManager,
        children: Vec<Element>,
        relative_rect: IntRect,
    ) -> Self {
        let mut w = Self {
            children,
            relative_rect,
        };

        w.update_size();
        w.update_position(relative_rect);
        w
    }

    pub fn mut_children(&mut self) -> impl Iterator<Item = &mut Element> {
        self.children.iter_mut()
    }

    pub fn children(&self) -> impl Iterator<Item = &Element> {
        self.children.iter()
    }
}

impl ElementTrait for RootNode {
    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = Vec::new();
        for ele in self.mut_children() {
            events.append(&mut ele.update(&resource_manager));
        }

        events
    }

    fn update_size(&mut self) {
        self.mut_children().for_each(|ele| ele.update_size());
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.relative_rect = relative_rect;
        self.mut_children()
            .for_each(|ele| ele.update_position(relative_rect));
    }

    fn global_bounds(&self) -> IntRect {
        self.relative_rect
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        let mut events = Vec::new();
        for ele in self.mut_children() {
            events.append(&mut ele.event_handler(&ui_settings, event));
        }

        events
    }

    fn render(&mut self, window: &mut RenderTexture) {
        for ele in self.mut_children() {
            ele.render(window);
        }
    }

    fn box_clone(&self) -> Box<dyn ElementTrait> {
        Box::new(self.clone())
    }

    fn set_ui_position(&mut self, _: UIPosition, relative_rect: IntRect) {
        self.update_position(relative_rect);
    }
}
