use super::traits::Element;
use crate::{assets::resource_manager::ResourceManager, ui::utils::positioning::UIPosition};
use sfml::graphics::{Color, IntRect, RcText, RenderTarget, RenderTexture, Transformable};

#[derive(Debug, Clone)]
pub struct Text {
    position: UIPosition,
    global_bounds: IntRect,
    text: RcText,
    pub color: Color,
}

impl Default for Text {
    fn default() -> Self {
        Self {
            position: Default::default(),
            global_bounds: Default::default(),
            text: Default::default(),
            color: Color::WHITE,
        }
    }
}

impl Text {
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        text: &str,
        font_size: u32,
        color: Color,
    ) -> Self {
        let mut t = Self {
            position,
            text: RcText::new(text, resource_manager.fetch_current_font(), font_size),
            global_bounds: Default::default(),
            color,
        };
        t.update_size();

        t
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.set_string(text);
    }
}

impl Element for Text {
    fn update_size(&mut self) {
        self.global_bounds.width = self.text.global_bounds().width as i32;
        self.global_bounds.height = self.text.global_bounds().height as i32;
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());

        self.text
            .set_position(self.global_bounds.position().as_other());
    }

    fn render(&mut self, window: &mut RenderTexture) {
        window.draw(&self.text);
    }

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn box_clone(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }
}
