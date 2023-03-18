use sfml::{graphics::IntRect, system::Vector2u, window::Event as SFMLEvent};
use tracing::warn;
use utils::{arithmetic_util_functions::i32_from_u32, resource_manager::ResourceManager};

use crate::{events::Event, ui_settings::UISettings, utils::positioning::UIPosition};

use super::{
    traits::Element as ElementTrait,
    traits::{self, cast_element},
    Element,
};

#[derive(Clone, Default, Debug)]
pub struct Div {
    global_bounds: IntRect,
    position: UIPosition,
    children: Vec<Element>,
    padding: UIPosition,
    size: Option<Vector2u>,
}

impl Div {
    pub fn new(
        _resource_manager: &ResourceManager,
        position: UIPosition,
        children: Vec<Element>,
        mut padding: UIPosition,
        mut size: Option<Vector2u>,
    ) -> Self {
        if padding != Default::default() && size.is_some() {
            warn!("Div: padding and size are both defined! Prioritizing size and setting padding to default");
            padding = Default::default();
        } else if size.is_none() && children.len() > 1 {
            warn!(
                "Div: Size is undefined.\n\
                Therefore, padding is supposed to be used, but there are more than 1 children!\n\
                Padding is unsupported with more than 1 children.\n\
                Setting padding to default and using a size of (0,0)"
            );
            size = Some(Default::default());
            padding = Default::default();
        }
        Self {
            global_bounds: Default::default(),
            position,
            children,
            padding,
            size,
        }
    }

    pub fn mut_children(&mut self) -> impl Iterator<Item = &mut Element> {
        self.children.iter_mut()
    }

    pub fn children(&self) -> impl Iterator<Item = &Element> {
        self.children.iter()
    }

    pub fn set_size(&mut self, size: Vector2u) {
        self.size = Some(size);
        self.update_size();
    }

    pub fn is_padding(&self) -> bool {
        self.size.is_none()
    }
}

impl traits::Element for Div {
    cast_element!();
    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        self.global_bounds.width = i32_from_u32(self.size.unwrap_or_default().x);
        self.global_bounds.height = i32_from_u32(self.size.unwrap_or_default().y);

        self.mut_children().for_each(|ele| ele.update_size());

        if let (Some(ele), true) = (self.children.get_mut(0), self.size.is_none()) {
            self.global_bounds.width = ele.global_bounds().width
                + self.padding.left.unwrap_or_default()
                + self.padding.right.unwrap_or_default();
            self.global_bounds.height = ele.global_bounds().height
                + self.padding.top.unwrap_or_default()
                + self.padding.bottom.unwrap_or_default();
        }
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());

        let relative_rect = self.padding.padded_inner_rect(self.global_bounds);
        self.mut_children()
            .for_each(|ele| ele.update_position(relative_rect));
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = Vec::new();
        for ele in self.mut_children() {
            events.append(&mut ele.update(resource_manager));
        }

        events
    }

    fn set_ui_position(
        &mut self,
        _ui_position: crate::utils::positioning::UIPosition,
        relative_rect: IntRect,
    ) {
        self.update_position(relative_rect);
    }

    fn render(&mut self, render_texture: &mut sfml::graphics::RenderTexture) {
        for ele in self.mut_children() {
            ele.render(render_texture);
        }
    }

    fn box_clone(&self) -> Box<dyn traits::Element> {
        Box::new(self.clone())
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        let mut events = Vec::new();
        for ele in self.mut_children() {
            events.append(&mut ele.event_handler(ui_settings, event))
        }

        events
    }
}
