use sfml::{
    graphics::{IntRect, RcSprite, RenderTarget, Transformable},
    system::Vector2,
};
use utils::resource_manager::ResourceManager;

use crate::utils::positioning::UIPosition;

use super::super::traits::{cast_element, Element};

#[derive(Debug, Clone)]
pub struct Image {
    position: UIPosition,
    global_bounds: IntRect,
    image: RcSprite,
    rerender: bool,
}

impl Image {
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        asset_id: &str,
        frame_id: usize,
        scale: f32,
    ) -> Self {
        let asset = resource_manager.fetch_asset(asset_id);
        let mut image = asset.get_rc_sprite_with_frame_num(frame_id);
        image.set_scale(Vector2::new(scale, scale));
        let mut i = Self {
            position,
            global_bounds: image.global_bounds().as_other(),
            image,
            rerender: true,
        };
        i.update_size();

        i
    }
}

impl Element for Image {
    cast_element!();

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        self.global_bounds.width = self.image.global_bounds().width as i32;
        self.global_bounds.height = self.image.global_bounds().height as i32;
        self.rerender = true;
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());

        self.image
            .set_position(self.global_bounds.position().as_other());
        self.rerender = true;
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }

    fn render(&mut self, render_texture: &mut sfml::graphics::RenderTexture) {
        self.rerender = false;

        render_texture.draw(&self.image);
    }
}
