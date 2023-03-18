use sfml::{
    graphics::{Color, IntRect, RectangleShape, RenderTarget, Shape, Transformable},
    system::Vector2,
    window::Event as SFMLEvent,
};
use tracing::error;
use utils::{arithmetic_util_functions::i32_from_u32, resource_manager::ResourceManager};

use crate::{
    elements::{
        text::Text,
        traits::{cast_actionable_element, cast_element, ActionableElement, Element},
    },
    events::{Event, EventId, Events, EMPTY_EVENT},
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};

use super::traits::TextBox;

const START_HORIZONTAL_OFFSET: UIPosition = UIPosition {
    top: None,
    bottom: None,
    left: Some(5),
    right: None,
};

#[derive(Debug, Default, Clone)]
pub struct FixedSizeOneLineTextbox {
    global_bounds: IntRect,
    position: UIPosition,
    background_rect: RectangleShape<'static>,
    text: Text,
    string: String,
    event_id: EventId,
    sync_id: EventId,
    hover: bool,
    selected: bool,
    rerender: bool,
}

impl FixedSizeOneLineTextbox {
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        width: u16,
        font_size: u32,
        text_color: Color,
        background_color: Color,
        default_text: &str,
        event_id: EventId,
        sync_id: EventId,
    ) -> Self {
        let mut background_rect = RectangleShape::with_size(Vector2::new(width as f32, 0.));
        background_rect.set_fill_color(background_color);
        let mut fstb = Self {
            global_bounds: IntRect::new(0, 0, width.into(), 0),
            position,
            background_rect,
            string: default_text.to_string(),
            text: Text::new(
                resource_manager,
                START_HORIZONTAL_OFFSET,
                default_text,
                true,
                font_size,
                text_color,
            ),
            sync_id,
            event_id,
            hover: false,
            selected: false,
            rerender: true,
        };
        fstb.update_size();

        fstb
    }
}

impl Element for FixedSizeOneLineTextbox {
    cast_element!();

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        TextBox::event_handler(self, ui_settings, event)
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }
    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn global_bounds(&self) -> sfml::graphics::IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        self.global_bounds.height = self.text.global_bounds().height
            + i32_from_u32(self.text.global_bounds().height.ilog2());
        self.background_rect
            .set_size(self.global_bounds.size().as_other());
    }

    fn update_position(&mut self, relative_rect: sfml::graphics::IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        self.background_rect
            .set_position(self.global_bounds.position().as_other());
        self.text.update_position(self.global_bounds);
    }

    fn set_ui_position(
        &mut self,
        ui_position: crate::utils::positioning::UIPosition,
        relative_rect: sfml::graphics::IntRect,
    ) {
        self.position = ui_position;
        self.update_position(relative_rect);
    }

    fn update(&mut self, _resource_manager: &ResourceManager) -> Vec<Event> {
        if self.rerender {
            vec![EMPTY_EVENT]
        } else {
            vec![]
        }
    }

    fn render(&mut self, render_texture: &mut sfml::graphics::RenderTexture) {
        render_texture.draw(&self.background_rect);
        self.text.render(render_texture);
        self.rerender = false;
    }

    fn box_clone(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }
}

impl ActionableElement for FixedSizeOneLineTextbox {
    cast_actionable_element!();

    fn triggered_event(&self) -> Event {
        Event::new(self.event_id, Events::StringEvent(self.string.clone()))
    }

    fn bind_pressed(&mut self, mouse_pos: sfml::system::Vector2i) {
        self.set_hover(mouse_pos);
    }

    fn bind_released(&mut self, mouse_pos: sfml::system::Vector2i) {
        self.set_hover(mouse_pos);
        self.selected = self.hover;
    }

    fn set_hover(&mut self, mouse_pos: sfml::system::Vector2i) {
        self.hover = self.global_bounds.contains(mouse_pos);
    }

    fn is_hover(&self) -> bool {
        self.hover
    }
}

impl TextBox for FixedSizeOneLineTextbox {
    fn utf32_str(&self) -> &str {
        &self.string
    }

    fn ascii_str(&self) -> Option<String> {
        if self.string.is_ascii() {
            Some(self.string.to_ascii_lowercase())
        } else {
            None
        }
    }

    fn box_clone(&self) -> Box<dyn TextBox> {
        Box::new(self.clone())
    }

    fn text_entered(&mut self, event: SFMLEvent) {
        if let SFMLEvent::TextEntered { unicode } = event {
            if unicode == 0x08 as char {
                self.string.pop();
            }
            // Ignore return carriage
            else if unicode == 0xD as char {
            }
            // Ignore ctrl+v/ctrl+v generated chars
            else if unicode != 0x16 as char && unicode != 0x03 as char {
                self.string.push(unicode);
            }
            self.text.set_text(&self.string);
            self.rerender = true;
        } else {
            error!("Event is not a TextEntered event! {:#?}", event)
        }
    }

    fn is_selected(&self) -> bool {
        self.selected
    }
}
