use super::traits::*;
use crate::{
    assets::resource_manager::ResourceManager,
    ui::{
        elements::{tiling_sprites::traits::TilingSpriteElement, traits::Element},
        events::*,
        ui_settings::UISettings,
        utils::{mouse_ui_states::UIMouseStates, positioning::UIPosition},
    },
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    system::Vector2i,
    window::Event as SFMLEvent,
};

#[derive(Debug, Clone)]
pub struct RepeatableSpritesButton {
    global_bounds: IntRect,
    position: UIPosition,
    current_mouse_state: UIMouseStates,
    repeatable_sprites: Box<dyn TilingSpriteElement>,
    hover_repeatable_sprites: Box<dyn TilingSpriteElement>,
    click_repeatable_sprites: Box<dyn TilingSpriteElement>,
    rerender: bool,
}

impl RepeatableSpritesButton {
    pub fn new(
        _resource_manager: &ResourceManager,
        position: UIPosition,
        repeatable_sprites: Box<dyn TilingSpriteElement>,
        hover_repeatable_sprites: Box<dyn TilingSpriteElement>,
        click_repeatable_sprites: Box<dyn TilingSpriteElement>,
    ) -> Self {
        let mut rswuims = Self {
            current_mouse_state: Default::default(),
            position,
            repeatable_sprites,
            hover_repeatable_sprites,
            click_repeatable_sprites,
            global_bounds: Default::default(),
            rerender: true,
        };
        rswuims.update_size();
        rswuims
    }

    fn compact_repeat_sprites_mut(&mut self) -> [&mut Box<dyn TilingSpriteElement>; 3] {
        [
            &mut self.repeatable_sprites,
            &mut self.hover_repeatable_sprites,
            &mut self.click_repeatable_sprites,
        ]
    }
}

impl Button for RepeatableSpritesButton {
    fn current_mouse_state(&self) -> UIMouseStates {
        self.current_mouse_state
    }

    fn triggered_event(&self) -> Event {
        Event {
            id: self.event_id(),
            event: Events::BooleanEvent(true),
        }
    }

    fn bind_released(&mut self, _: Vector2i) {
        self.rerender = true;
        self.current_mouse_state = UIMouseStates::Nothing;
    }

    fn bind_pressed(&mut self, mouse_pos: Vector2i) {
        self.set_hover(mouse_pos);

        if self.is_hover() {
            self.rerender = true;
            self.current_mouse_state = UIMouseStates::Click
        }
    }

    fn is_hover(&self) -> bool {
        self.current_mouse_state.is_hover()
    }

    fn set_hover(&mut self, mouse_pos: Vector2i) {
        let previous_mouse_state = self.current_mouse_state;
        self.current_mouse_state = if !self.global_bounds.contains(mouse_pos) {
            UIMouseStates::Nothing
        } else if self.current_mouse_state == UIMouseStates::Nothing {
            UIMouseStates::Hover
        } else {
            self.current_mouse_state
        };

        self.rerender |= self.current_mouse_state != previous_mouse_state;
    }

    fn box_clone(&self) -> Box<dyn Button> {
        Box::new(self.clone())
    }
}

impl Element for RepeatableSpritesButton {
    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        for r_sprite in self.compact_repeat_sprites_mut() {
            r_sprite.update_size();
        }
        self.global_bounds.width = self.repeatable_sprites.global_bounds().width;
        self.global_bounds.height = self.repeatable_sprites.global_bounds().height;
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self.position.center_with_size(
            relative_rect,
            self.global_bounds
                .size()
                .try_into_other()
                .unwrap_or_default(),
        );

        let global_bounds = self.global_bounds;
        for r_sprite in self.compact_repeat_sprites_mut() {
            r_sprite.update_position(global_bounds);
        }
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = Vec::new();
        for r_sprite in self.compact_repeat_sprites_mut() {
            events.append(&mut r_sprite.update(&resource_manager));
        }

        if self.rerender {
            self.rerender = false;
            events.push(EMPTY_EVENT);
        }

        events
    }

    fn box_clone(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }

    fn render(&mut self, window: &mut RenderTexture) {
        match self.current_mouse_state {
            UIMouseStates::Nothing => self.repeatable_sprites.render(window),
            UIMouseStates::Hover => self.hover_repeatable_sprites.render(window),
            UIMouseStates::Click => self.click_repeatable_sprites.render(window),
        }
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        ButtonElement::event_handler(self, &ui_settings, event)
    }
}

impl ButtonElement for RepeatableSpritesButton {
    fn as_mut_element(&mut self) -> &mut dyn Element {
        self
    }

    fn as_mut_button(&mut self) -> &mut dyn Button {
        self
    }

    fn as_element(&self) -> &dyn Element {
        self
    }

    fn as_button(&self) -> &dyn Button {
        self
    }

    fn box_clone(&self) -> Box<dyn ButtonElement> {
        Box::new(self.clone())
    }
}
