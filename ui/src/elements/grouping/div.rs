use sfml::{graphics::IntRect, system::Vector2u, window::Event as SFMLEvent};
use utils::{
    arithmetic_util_functions::i32_from_u32, resource_manager::ResourceManager,
    sfml_util_functions::vector2u_from_vector2i,
};

use crate::{events::Event, ui_settings::UISettings, utils::positioning::UIPosition};

use super::super::{
    traits::Element as ElementTrait,
    traits::{self, cast_element},
    Element,
};

#[derive(Clone, Default, Debug)]
pub struct Div {
    global_bounds: IntRect,
    position: UIPosition,
    children: Vec<Element>,
    padding: Option<UIPosition>,
    size: Option<Vector2u>,
    use_relative_rect_size: bool,
}

impl Div {
    pub fn new(
        position: UIPosition,
        children: Vec<Element>,
        mut padding: Option<UIPosition>,
        size: Option<Vector2u>,
    ) -> Self {
        if padding.is_some() && size.is_some() {
            padding = None;
        }
        let use_relative_rect_size = padding.is_none() && size.is_none();
        Self {
            global_bounds: Default::default(),
            position,
            children,
            padding,
            size,
            use_relative_rect_size,
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

    pub fn padding(&self) -> Option<UIPosition> {
        self.padding
    }

    pub fn size(&self) -> Option<Vector2u> {
        self.size
    }

    pub fn position(&self) -> UIPosition {
        self.position
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

        if let (Some(ele), Some(padding)) = (self.children.get_mut(0), self.padding) {
            self.global_bounds.width = ele.global_bounds().width
                + padding.left.unwrap_or_default()
                + padding.right.unwrap_or_default();
            self.global_bounds.height = ele.global_bounds().height
                + padding.top.unwrap_or_default()
                + padding.bottom.unwrap_or_default();
        }
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        if self.use_relative_rect_size {
            self.set_size(vector2u_from_vector2i(relative_rect.size()));
        }

        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());

        let relative_rect = if let Some(padding) = self.padding {
            padding.padded_inner_rect(self.global_bounds)
        } else {
            self.global_bounds
        };
        self.mut_children()
            .for_each(|ele| ele.update_position(relative_rect));
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

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        let mut rerender = false;
        let mut events = Vec::new();
        for ele in self.mut_children() {
            let mut event = ele.event_handler(ui_settings, event);
            rerender |= event.1;
            events.append(&mut event.0);
        }

        (events, rerender)
    }
}
