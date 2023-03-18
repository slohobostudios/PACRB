use super::{image_button::ImageButton, traits::*};
use crate::{
    elements::traits::{cast_actionable_element, cast_element, ActionableElement, Element},
    events::*,
    ui_settings::UISettings,
    utils::{mouse_ui_states::UIMouseStates, positioning::UIPosition},
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    system::{Vector2, Vector2i},
    window::Event as SFMLEvent,
};
use utils::resource_manager::ResourceManager;

#[derive(Clone, Debug, Default)]
pub struct BooleanImageButton {
    event_id: u16,
    sync_id: u16,
    position: UIPosition,
    truth_button: ImageButton,
    false_button: ImageButton,
    pub state: bool,
    global_bounds: IntRect,
}

impl BooleanImageButton {
    pub fn new(
        resource_manager: &ResourceManager,
        position: UIPosition,
        scale: f32,
        state: bool,
        asset_id: &str,
        truth_frame_id: usize,
        truth_hover_frame_id: usize,
        truth_click_frame_id: usize,
        false_frame_id: usize,
        false_hover_frame_id: usize,
        false_click_frame_id: usize,
        event_id: u16,
        sync_id: u16,
    ) -> Self {
        let truth_button = ImageButton::new(
            resource_manager,
            UIPosition::CENTER,
            &asset_id,
            truth_frame_id,
            truth_hover_frame_id,
            truth_click_frame_id,
            scale,
            0, // 0 event_id means this event doens't matter
            0, // 0 sync_id means this event doesn't need to be synced
        );
        let false_button = ImageButton::new(
            resource_manager,
            UIPosition::CENTER,
            &asset_id,
            false_frame_id,
            false_hover_frame_id,
            false_click_frame_id,
            scale,
            0, // 0 event_id means this event doesn't matter
            0, // 0 sync_id means this event doesn't need to be synced
        );

        let truth_gb = truth_button.global_bounds();
        let false_gb = false_button.global_bounds();
        let max_size = Vector2::new(
            truth_gb.width.max(false_gb.width),
            truth_gb.height.max(false_gb.height),
        );

        let mut bib = Self {
            position,
            state,
            truth_button,
            false_button,
            event_id,
            sync_id,
            global_bounds: IntRect::from_vecs(Vector2::new(0, 0), max_size),
        };
        bib.update_size();

        bib
    }

    fn current_button(&self) -> &ImageButton {
        if self.state {
            &self.truth_button
        } else {
            &self.false_button
        }
    }

    fn current_button_mut(&mut self) -> &mut ImageButton {
        if self.state {
            &mut self.truth_button
        } else {
            &mut self.false_button
        }
    }
}

impl ActionableElement for BooleanImageButton {
    cast_actionable_element!();

    fn triggered_event(&self) -> Event {
        Event {
            id: self.event_id(),
            event: Events::BooleanEvent(self.state),
        }
    }

    fn bind_released(&mut self, mouse_pos: Vector2i) {
        self.truth_button.bind_released(mouse_pos);
        self.false_button.bind_released(mouse_pos);

        self.set_hover(mouse_pos);
        if self.is_hover() {
            self.state = !self.state;
        }
    }

    fn bind_pressed(&mut self, mouse_pos: Vector2i) {
        self.truth_button.bind_pressed(mouse_pos);
        self.false_button.bind_pressed(mouse_pos);
    }

    fn set_hover(&mut self, mouse_pos: Vector2i) {
        self.current_button_mut().set_hover(mouse_pos);
    }

    fn is_hover(&self) -> bool {
        self.current_button().is_hover()
    }
}

impl Button for BooleanImageButton {
    fn current_mouse_state(&self) -> UIMouseStates {
        self.current_button().current_mouse_state()
    }
    fn box_clone(&self) -> Box<dyn Button> {
        Box::new(self.clone())
    }
}

impl Element for BooleanImageButton {
    cast_element!();

    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = self.truth_button.update(resource_manager);
        events.append(&mut self.false_button.update(resource_manager));

        events
    }

    fn update_size(&mut self) {
        self.false_button.update_size();
        self.truth_button.update_size();

        self.global_bounds.width = self
            .false_button
            .global_bounds()
            .width
            .max(self.truth_button.global_bounds().width);
        self.global_bounds.height = self
            .false_button
            .global_bounds()
            .height
            .max(self.truth_button.global_bounds().height);
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = self
            .position
            .center_with_size(relative_rect, self.global_bounds.size());
        self.false_button.update_position(self.global_bounds);
        self.truth_button.update_position(self.global_bounds);
    }

    fn render(&mut self, window: &mut RenderTexture) {
        self.current_button_mut().render(window);
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        Button::event_handler(self, &ui_settings, event)
    }

    fn box_clone(&self) -> Box<dyn Element> {
        Box::new(self.clone())
    }

    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }
}
