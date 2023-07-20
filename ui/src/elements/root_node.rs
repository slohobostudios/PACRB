use super::{
    div::Div,
    traits::{cast_element, Element as ElementTrait},
    Element,
};
use crate::{events::*, ui_settings::UISettings, utils::positioning::UIPosition};
use sfml::{
    graphics::{IntRect, RenderTexture},
    window::Event as SFMLEvent,
};
use utils::{resource_manager::ResourceManager, sfml_util_functions::vector2u_from_vector2i};

#[derive(Default, Clone, Debug)]
pub struct RootNode {
    div: Div,
    relative_rect: IntRect,
}

impl RootNode {
    pub fn new(
        _resource_manager: &ResourceManager,
        children: Vec<Element>,
        relative_rect: IntRect,
    ) -> Self {
        let mut w = Self {
            div: Div::new(
                Default::default(),
                children,
                Default::default(),
                relative_rect.size().try_into_other().ok(),
            ),
            relative_rect,
        };

        w.update_size();
        w.update_position(relative_rect);
        w
    }

    pub fn children(&self) -> impl Iterator<Item = &Element> {
        self.div.children()
    }

    pub fn mut_children(&mut self) -> impl Iterator<Item = &mut Element> {
        self.div.mut_children()
    }
}

impl ElementTrait for RootNode {
    cast_element!();
    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        self.div.update(resource_manager)
    }

    fn update_size(&mut self) {
        self.div.update_size()
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.relative_rect = relative_rect;
        self.div
            .set_size(vector2u_from_vector2i(self.relative_rect.size()));
        self.div.update_position(relative_rect);
    }

    fn global_bounds(&self) -> IntRect {
        self.relative_rect
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        self.div.event_handler(ui_settings, event)
    }

    fn render(&mut self, render_texture: &mut RenderTexture) {
        self.div.render(render_texture)
    }

    fn set_ui_position(&mut self, _: UIPosition, relative_rect: IntRect) {
        self.update_position(relative_rect);
    }
}
