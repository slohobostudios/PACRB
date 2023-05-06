use super::traits::*;
use crate::{
    elements::traits::{cast_actionable_element, cast_element, ActionableElement, Element},
    events::*,
    ui_settings::UISettings,
    utils::{mouse_ui_states::UIMouseStates, positioning::UIPosition},
};
use sfml::{
    graphics::{IntRect, RcSprite, RenderTarget, RenderTexture, Transformable},
    system::{Vector2, Vector2i},
    window::Event as SFMLEvent,
};
use utils::resource_manager::ResourceManager;

const CLICK_BOUNDS_SLICE_NAME: &str = "click";

#[derive(Debug, Clone, Default)]
pub struct ImageButton {
    global_bounds: IntRect,
    position: UIPosition,
    texture_sprite: RcSprite,
    hover_texture_sprite: RcSprite,
    click_texture_sprite: RcSprite,
    current_mouse_state: UIMouseStates,
    event_id: u16,
    sync_id: u16,
    rerender: bool,
}

impl ImageButton {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        asset_id: &str,
        frame_id: usize,
        hover_frame_id: usize,
        click_frame_id: usize,
        scale: f32,
        event_id: u16,
        sync_id: u16,
    ) -> Self {
        let asset = &resource_manager.fetch_asset(asset_id);

        let mut ib = Self {
            global_bounds: asset.get_scaled_and_shifted_slice_bound(
                CLICK_BOUNDS_SLICE_NAME,
                frame_id,
                scale,
            ),
            position,
            texture_sprite: asset.get_rc_sprite_with_frame_num(frame_id),
            hover_texture_sprite: asset.get_rc_sprite_with_frame_num(hover_frame_id),
            click_texture_sprite: asset.get_rc_sprite_with_frame_num(click_frame_id),
            current_mouse_state: UIMouseStates::Nothing,
            event_id,
            sync_id,
            rerender: true,
        };
        for sprite in ib.compact_sprites_mut() {
            sprite.set_scale(Vector2::new(scale, scale));
        }
        ib.update_size();

        ib
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_texture_bounds(
        resource_manager: &ResourceManager,
        position: UIPosition,
        asset_id: &str,
        texture_bounds: IntRect,
        hover_texture_bounds: IntRect,
        click_texture_bounds: IntRect,
        scale: f32,
        event_id: u16,
        sync_id: u16,
    ) -> Self {
        let texture = &resource_manager.fetch_asset(asset_id).texture();

        let mut ib = Self {
            global_bounds: texture_bounds,
            position,
            texture_sprite: RcSprite::with_texture_and_rect(texture, texture_bounds),
            hover_texture_sprite: RcSprite::with_texture_and_rect(texture, hover_texture_bounds),
            click_texture_sprite: RcSprite::with_texture_and_rect(texture, click_texture_bounds),
            current_mouse_state: UIMouseStates::Nothing,
            event_id,
            sync_id,
            rerender: true,
        };
        for sprite in ib.compact_sprites_mut() {
            sprite.set_scale(Vector2::new(scale, scale));
        }
        ib.update_size();

        ib
    }

    fn current_sprite(&self) -> &RcSprite {
        match self.current_mouse_state {
            UIMouseStates::Nothing => &self.texture_sprite,
            UIMouseStates::Hover => &self.hover_texture_sprite,
            UIMouseStates::Click => &self.click_texture_sprite,
        }
    }

    fn compact_sprites_mut(&mut self) -> [&mut RcSprite; 3] {
        [
            &mut self.texture_sprite,
            &mut self.hover_texture_sprite,
            &mut self.click_texture_sprite,
        ]
    }
}

impl ActionableElement for ImageButton {
    cast_actionable_element!();

    fn triggered_event(&self) -> Event {
        Event {
            id: self.event_id(),
            event: Events::BooleanEvent(true),
        }
    }

    fn is_hover(&self) -> bool {
        self.current_mouse_state == UIMouseStates::Hover
            || self.current_mouse_state == UIMouseStates::Click
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

    fn bind_pressed(&mut self, mouse_pos: Vector2i) {
        self.set_hover(mouse_pos);

        if self.is_hover() {
            self.rerender = true;
            self.current_mouse_state = UIMouseStates::Click
        }
    }

    fn bind_released(&mut self, mouse_pos: Vector2i) {
        self.current_mouse_state = UIMouseStates::Nothing;
        self.rerender = true;
        self.set_hover(mouse_pos);
    }
}

impl Button for ImageButton {
    fn current_mouse_state(&self) -> UIMouseStates {
        self.current_mouse_state
    }
    fn box_clone(&self) -> Box<dyn Button> {
        Box::new(self.clone())
    }
}

impl Element for ImageButton {
    cast_element!();

    fn update(&mut self, _resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        (Default::default(), self.rerender)
    }

    fn update_size(&mut self) {
        self.global_bounds.width = self.texture_sprite.global_bounds().width as i32;
        self.global_bounds.height = self.texture_sprite.global_bounds().height as i32;
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());

        let gb_pos = self.global_bounds.position().as_other();
        for sprite in self.compact_sprites_mut() {
            sprite.set_position(gb_pos);
        }
    }

    fn render(&mut self, window: &mut RenderTexture) {
        self.rerender = false;
        window.draw(self.current_sprite());
    }

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        Button::event_handler(self, ui_settings, event)
    }

    fn event_id(&self) -> EventId {
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
}
