use super::{image_button::ImageButton, traits::*};
use crate::{
    elements::traits::{cast_actionable_element, cast_element, ActionableElement, Element},
    events::*,
    syncs::*,
    ui_settings::UISettings,
    utils::{mouse_ui_states::UIMouseStates, positioning::UIPosition},
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    system::{Vector2, Vector2i},
    window::Event as SFMLEvent,
};
use tracing::warn;
use utils::{center_of_rect, resource_manager::ResourceManager};

/// Displays a button which has an on and an off state.
#[derive(Clone, Debug, Default)]
pub struct BooleanImageButton {
    event_id: u16,
    sync_id: u16,
    position: UIPosition,
    truth_button: ImageButton,
    false_button: ImageButton,
    pub state: bool,
    global_bounds: IntRect,
    rerender: bool,
}

impl BooleanImageButton {
    #[allow(clippy::too_many_arguments)]
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
            asset_id,
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
            asset_id,
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
            rerender: true,
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
        self.truth_button.set_hover(mouse_pos);
        self.false_button.set_hover(mouse_pos);
    }

    fn is_hover(&self) -> bool {
        self.current_button().is_hover()
    }

    fn event_id(&self) -> EventId {
        self.event_id
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

    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        let mut rerender = self.rerender;
        let truth_button_event = self.truth_button.update(resource_manager);
        rerender |= truth_button_event.1;
        let mut events = truth_button_event.0;
        let mut false_button_event = self.false_button.update(resource_manager);
        rerender |= false_button_event.1;
        events.append(&mut false_button_event.0);

        (events, rerender)
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
        self.rerender = false;
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        let mut rerender = false;
        let mut events = Vec::new();
        let mut curr_button_event =
            Element::event_handler(self.current_button_mut(), ui_settings, event);
        rerender |= curr_button_event.1;
        events.append(&mut curr_button_event.0);
        let mut event = Button::event_handler(self, ui_settings, event);
        rerender |= curr_button_event.1;
        events.append(&mut event.0);
        (events, rerender)
    }

    fn sync_id(&self) -> u16 {
        self.sync_id
    }

    fn sync(&mut self, sync: Syncs) {
        let Syncs::Boolean(state) = sync else {
            warn!(ui_syncs_not_synced_str!(), Syncs::Boolean(Default::default()), sync);
            return;
        };
        if state ^ self.state {
            self.rerender = true;
            self.bind_released(center_of_rect!(i32, self.global_bounds));
        }
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.position = ui_position;
        self.update_size();
        self.update_position(relative_rect);
    }
}
