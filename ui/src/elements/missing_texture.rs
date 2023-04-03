use crate::{elements::traits::Element, utils::positioning::UIPosition};
use sfml::{
    graphics::{IntRect, RcSprite, RenderTarget, RenderTexture},
    system::{Vector2, Vector2u},
};
use utils::resource_manager::ResourceManager;

use super::traits::cast_element;

// Allows custom placement of a missing texture.
#[derive(Clone, Debug)]
pub struct MissingTexture {
    pub global_bounds: IntRect,
    pub position: UIPosition,
    sprite: RcSprite,
}

impl MissingTexture {
    pub fn new(resource_manager: &ResourceManager, position: UIPosition, size: Vector2u) -> Self {
        let mut mt = Self {
            global_bounds: IntRect::from_vecs(
                Vector2::new(0, 0),
                size.try_into_other().unwrap_or_default(),
            ),
            position,
            sprite: RcSprite::with_texture(resource_manager.missing_texture().texture()),
        };
        mt.update_size();
        mt
    }
    fn readjust_sprite_texture_rect(&mut self) {
        self.sprite.set_texture_rect(IntRect::new(
            0,
            0,
            self.global_bounds.width,
            self.global_bounds.height,
        ))
    }
}

impl Element for MissingTexture {
    cast_element!();
    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }
    fn update_size(&mut self) {
        self.readjust_sprite_texture_rect();
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds.width = relative_rect.width;
        self.global_bounds.height = relative_rect.height;
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        self.readjust_sprite_texture_rect();
    }
    fn render(&mut self, window: &mut RenderTexture) {
        window.draw(&self.sprite);
    }

    fn box_clone(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_position(relative_rect);
    }
}
