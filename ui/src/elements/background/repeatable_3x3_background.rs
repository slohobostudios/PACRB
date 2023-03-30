use std::iter;

use super::traits::*;
use crate::{
    elements::{
        div::Div,
        tiling_sprites::{repeatable_3x3_sprite::Repeatable3x3Sprite, traits::TilingSprite},
        traits,
        traits::{cast_element, Element as ElementTrait},
        Element,
    },
    events::Event,
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    system::Vector2u,
    window::Event as SFMLEvent,
};
use utils::resource_manager::ResourceManager;

#[derive(Clone, Debug)]
pub struct Repeatable3x3Background {
    background: Repeatable3x3Sprite,
    hover: bool,
    div: Element,
}

impl Repeatable3x3Background {
    pub fn new(
        resource_manager: &ResourceManager,
        children: Vec<Element>,
        position: UIPosition,
        background_asset_id: &str,
        background_frame_id: u16,
        padding: UIPosition,
        desired_size: Option<Vector2u>,
        scale: f32,
    ) -> Self {
        let padding = if desired_size.is_some() {
            Default::default()
        } else {
            padding
        };
        let mut fsr33b = Self {
            div: Element::Div(Div::new(
                resource_manager,
                Default::default(),
                children,
                padding,
                desired_size,
            )),
            background: Repeatable3x3Sprite::new(
                resource_manager,
                background_asset_id,
                background_frame_id.into(),
                position,
                desired_size.unwrap_or_default(),
                scale,
            ),
            hover: false,
        };
        fsr33b.update_size();
        fsr33b
    }
}

impl Background for Repeatable3x3Background {
    fn is_hover(&self) -> bool {
        self.hover
    }
    fn set_hover(&mut self, mouse_pos: sfml::system::Vector2i) {
        self.hover = self.background.global_bounds().contains(mouse_pos);
    }
    fn children(&self) -> Box<dyn Iterator<Item = &Element> + '_> {
        Box::new(iter::once(&self.div))
    }

    fn mut_children(&mut self) -> Box<dyn Iterator<Item = &mut Element> + '_> {
        Box::new(iter::once(&mut self.div))
    }

    fn box_clone(&self) -> Box<dyn Background> {
        Box::new(self.clone())
    }
}

impl traits::Element for Repeatable3x3Background {
    cast_element!();
    fn global_bounds(&self) -> IntRect {
        self.background.global_bounds()
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        BackgroundElement::event_handler(self, ui_settings, event)
    }

    fn update_size(&mut self) {
        self.background.update_size();
        if let Element::Div(div) = &self.div {
            if div.is_padding() {
                self.background
                    .set_desired_size(self.div.global_bounds().size().as_other());
                self.background.update_size();
                self.update_element_size();
            }
        }
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.background.update_position(relative_rect);
        self.update_element_position();
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = Vec::new();
        events.append(&mut self.background.update(&resource_manager));
        events.append(&mut BackgroundElement::update(self, resource_manager));
        events
    }

    fn render(&mut self, window: &mut RenderTexture) {
        self.background.render(window);
        BackgroundElement::render(self, window);
    }

    fn box_clone(&self) -> Box<dyn traits::Element> {
        Box::new(self.clone())
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.background.set_ui_position(ui_position, relative_rect);
        self.update_size();
        self.update_position(relative_rect);
    }
}

impl BackgroundElement for Repeatable3x3Background {
    fn box_clone(&self) -> Box<dyn BackgroundElement> {
        Box::new(self.clone())
    }
}
