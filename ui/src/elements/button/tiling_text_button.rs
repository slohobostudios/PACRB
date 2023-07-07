use super::{repeatable_sprite_button::RepeatableSpritesButton, traits::*};
use crate::{
    elements::{
        tiling_sprites::{repeatable_3x3_sprite::Repeatable3x3Sprite, traits::TilingSprite},
        traits::{
            cast_actionable_element, cast_element, ActionableElement, Element as TraitElement,
        },
        Element,
    },
    events::*,
    ui_settings::UISettings,
    utils::{mouse_ui_states::UIMouseStates, positioning::UIPosition},
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    system::{Vector2, Vector2i, Vector2u},
    window::Event as SFMLEvent,
};
use utils::{
    arithmetic_util_functions::u32_from_i32, resource_manager::ResourceManager,
    sfml_util_functions::vector2i_from_vector2u,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        background_asset_id: &str,
        background_frame_id: usize,
        background_hover_frame_id: usize,
        background_click_frame_id: usize,
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
                resource_manager,
                Default::default(),
                Box::new(Repeatable3x3Sprite::new(
                    resource_manager,
                    background_asset_id,
                    background_frame_id,
                    UIPosition::CENTER,
                    *desired_size,
                    scale,
                )),
                Box::new(Repeatable3x3Sprite::new(
                    resource_manager,
                    background_asset_id,
                    background_hover_frame_id,
                    UIPosition::CENTER,
                    *desired_size,
                    scale,
                )),
                Box::new(Repeatable3x3Sprite::new(
                    resource_manager,
                    background_asset_id,
                    background_click_frame_id,
                    UIPosition::CENTER,
                    *desired_size,
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

    pub fn inner_element_mut(&mut self) -> &mut Element {
        &mut self.inner_element
    }

    pub fn inner_element(&self) -> &Element {
        &self.inner_element
    }

    pub fn set_desired_size(&mut self, desired_size: Vector2u) {
        self.backgrounds.set_desired_size(desired_size);
        self.update_size();
    }
}

impl ActionableElement for TilingButton {
    cast_actionable_element!();
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

    fn event_id(&self) -> EventId {
        self.event_id
    }
}

impl Button for TilingButton {
    fn current_mouse_state(&self) -> UIMouseStates {
        self.current_mouse_state
    }
    fn box_clone(&self) -> Box<dyn Button> {
        Box::new(self.clone())
    }
}

impl TraitElement for TilingButton {
    cast_element!();
    fn render(&mut self, window: &mut RenderTexture) {
        self.backgrounds.render(window);
        self.inner_element.render(window);
        self.rerender = false;
    }

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        fn update_bg_ie(self_: &mut TilingButton) {
            self_.backgrounds.update_size();
            self_.inner_element.update_size();
            self_.global_bounds = self_.backgrounds.global_bounds();

            self_.backgrounds.update_position(self_.global_bounds);
            self_.inner_element.update_position(self_.global_bounds);
        }
        update_bg_ie(self);

        let desired_size = vector2i_from_vector2u(self.backgrounds.desired_size());
        if self.inner_element.global_bounds().width > desired_size.x {
            self.backgrounds.set_desired_size(Vector2::new(
                u32_from_i32(self.inner_element.global_bounds().width),
                self.backgrounds.desired_size().y,
            ));
            update_bg_ie(self);
        }

        if self.inner_element.global_bounds().height > desired_size.y {
            self.backgrounds.set_desired_size(Vector2::new(
                self.backgrounds.desired_size().x,
                u32_from_i32(self.inner_element.global_bounds().height),
            ));
            update_bg_ie(self);
        }
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

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        Button::event_handler(&mut self.backgrounds, ui_settings, event);
        Button::event_handler(self, ui_settings, event)
    }

    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        let mut rerender = self.rerender;
        let background_event = self.backgrounds.update(resource_manager);
        rerender |= background_event.1;
        (background_event.0, rerender)
    }
}
