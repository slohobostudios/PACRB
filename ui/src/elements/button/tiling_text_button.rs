use super::{repeatable_sprite_button::RepeatableSpritesButton, traits::*};
use crate::{
    assets::resource_manager::ResourceManager,
    ui::{
        elements::{
            tiling_sprites::repeatable_3x3_sprite::Repeatable3x3Sprite,
            traits::Element as TraitElement, Element,
        },
        events::*,
        ui_settings::UISettings,
        utils::{mouse_ui_states::UIMouseStates, positioning::UIPosition},
    },
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    system::{Vector2, Vector2i},
    window::Event as SFMLEvent,
};

#[derive(Debug, Clone)]
pub struct TilingButton {
    global_bounds: IntRect,
    position: UIPosition,
    backgrounds: RepeatableSpritesButton,
    inner_element: Element,
    event_id: u16,
    sync_id: u16,
    current_mouse_state: UIMouseStates,
    rerender: bool,
}

impl TilingButton {
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        background_asset_id: &str,
        background_frame_id: usize,
        hover_background_frame_id: usize,
        click_background_frame_id: usize,
        inner_element: Element,
        desired_size: &Vector2<u32>,
        scale: f32,
        event_id: u16,
        sync_id: u16,
    ) -> Self {
        let mut mmb = Self {
            global_bounds: Default::default(),
            current_mouse_state: Default::default(),
            position,
            backgrounds: RepeatableSpritesButton::new(
                &resource_manager,
                Default::default(),
                Box::new(Repeatable3x3Sprite::new(
                    &resource_manager,
                    background_asset_id.into(),
                    background_frame_id.into(),
                    UIPosition::CENTER,
                    desired_size.clone(),
                    scale,
                )),
                Box::new(Repeatable3x3Sprite::new(
                    &resource_manager,
                    background_asset_id.into(),
                    hover_background_frame_id.into(),
                    UIPosition::CENTER,
                    desired_size.clone(),
                    scale,
                )),
                Box::new(Repeatable3x3Sprite::new(
                    &resource_manager,
                    background_asset_id.into(),
                    click_background_frame_id.into(),
                    UIPosition::CENTER,
                    desired_size.clone(),
                    scale,
                )),
            ),
            inner_element,
            event_id,
            sync_id,
            rerender: true,
        };
        mmb.update_size();

        mmb
    }
}

impl Button for TilingButton {
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

        self.rerender |= previous_mouse_state != self.current_mouse_state;
    }

    fn box_clone(&self) -> Box<dyn Button> {
        Box::new(self.clone())
    }
}

impl TraitElement for TilingButton {
    fn render(&mut self, window: &mut RenderTexture) {
        self.backgrounds.render(window);

        self.inner_element.render(window);
    }

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        self.inner_element.update_size();
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self.position.center_with_size(
            relative_rect,
            self.backgrounds
                .global_bounds()
                .size()
                .try_into_other()
                .unwrap_or_default(),
        );
        self.backgrounds.update_position(self.global_bounds);

        self.inner_element.update_position(self.global_bounds);
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        ButtonElement::event_handler(&mut self.backgrounds, &ui_settings, event);
        ButtonElement::event_handler(self, &ui_settings, event)
    }

    fn box_clone(&self) -> Box<dyn TraitElement> {
        Box::new(self.clone())
    }

    fn event_id(&self) -> u16 {
        self.event_id
    }

    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = self.backgrounds.update(resource_manager);

        if self.rerender {
            self.rerender = false;
            events.push(EMPTY_EVENT);
        }
        events
    }
}

impl ButtonElement for TilingButton {
    fn as_mut_element(&mut self) -> &mut dyn TraitElement {
        self
    }

    fn as_mut_button(&mut self) -> &mut dyn Button {
        self
    }

    fn as_element(&self) -> &dyn TraitElement {
        self
    }

    fn as_button(&self) -> &dyn Button {
        self
    }

    fn box_clone(&self) -> Box<dyn ButtonElement> {
        Box::new(self.clone())
    }
}
