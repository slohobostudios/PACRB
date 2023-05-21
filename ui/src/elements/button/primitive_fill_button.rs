use sfml::{
    graphics::{Color, IntRect, PrimitiveType},
    system::Vector2i,
};

use crate::{
    elements::{
        misc::primitive::Primitive,
        traits::{
            cast_actionable_element, cast_element, ActionableElement, Element as ElementTrait,
        },
        Element,
    },
    events::{Event, EventId, Events},
    utils::mouse_ui_states::UIMouseStates,
};

use super::traits::Button;

#[derive(Debug, Clone)]
pub struct PrimitiveFillButton {
    global_bounds: IntRect,
    current_mouse_state: UIMouseStates,
    primitive: Primitive,
    hover_primitive: Primitive,
    click_primitive: Primitive,
    inner_element: Element,
    rerender: bool,
    event_id: EventId,
}

impl PrimitiveFillButton {
    pub fn new(
        inner_element: Element,
        color: Color,
        hover_color: Color,
        click_color: Color,
        event_id: EventId,
    ) -> Self {
        Self {
            global_bounds: Default::default(),
            current_mouse_state: UIMouseStates::Nothing,
            primitive: Primitive::new(None, vec![], color, PrimitiveType::TRIANGLE_FAN),
            hover_primitive: Primitive::new(None, vec![], hover_color, PrimitiveType::TRIANGLE_FAN),
            click_primitive: Primitive::new(None, vec![], click_color, PrimitiveType::TRIANGLE_FAN),
            inner_element,
            rerender: true,
            event_id,
        }
    }

    fn compact_primitive_mut(&mut self) -> [&mut Primitive; 3] {
        [
            &mut self.primitive,
            &mut self.hover_primitive,
            &mut self.click_primitive,
        ]
    }
}

impl Button for PrimitiveFillButton {
    fn box_clone(&self) -> Box<dyn Button> {
        Box::new(self.clone())
    }

    fn current_mouse_state(&self) -> UIMouseStates {
        self.current_mouse_state
    }
}

impl ActionableElement for PrimitiveFillButton {
    fn triggered_event(&self) -> crate::events::Event {
        Event {
            id: self.event_id(),
            event: Events::BooleanEvent(self.current_mouse_state == UIMouseStates::Click),
        }
    }

    fn bind_pressed(&mut self, mouse_pos: sfml::system::Vector2i) {
        self.set_hover(mouse_pos);
        if self.is_hover() {
            self.rerender = true;
            self.current_mouse_state = UIMouseStates::Click;
        }
    }

    fn bind_released(&mut self, _: sfml::system::Vector2i) {
        self.rerender = true;
        self.current_mouse_state = UIMouseStates::Nothing;
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

    fn is_hover(&self) -> bool {
        self.current_mouse_state.is_hover()
    }
    cast_actionable_element!();
}

impl ElementTrait for PrimitiveFillButton {
    cast_element!();
    fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    fn update_size(&mut self) {
        self.inner_element.update_size();
        for primitive in self.compact_primitive_mut() {
            primitive.update_size();
        }
        self.global_bounds.width = self.primitive.global_bounds().width;
        self.global_bounds.height = self.primitive.global_bounds().height;
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.global_bounds = relative_rect;
        for primitive in self.compact_primitive_mut() {
            primitive.update_position(relative_rect);
        }
        self.inner_element.update_position(relative_rect);
    }

    fn set_ui_position(
        &mut self,
        _: crate::utils::positioning::UIPosition,
        relative_rect: IntRect,
    ) {
        // ui_position is not applicable to this element.
        self.update_position(relative_rect);
    }

    fn render(&mut self, render_texture: &mut sfml::graphics::RenderTexture) {
        match self.current_mouse_state {
            UIMouseStates::Nothing => self.primitive.render(render_texture),
            UIMouseStates::Hover => self.hover_primitive.render(render_texture),
            UIMouseStates::Click => self.click_primitive.render(render_texture),
        }
        self.inner_element.render(render_texture);
        self.rerender = false;
    }

    fn event_handler(
        &mut self,
        ui_settings: &crate::ui_settings::UISettings,
        sfml_event: sfml::window::Event,
    ) -> (Vec<Event>, bool) {
        let mut events = vec![];
        let mut event = Button::event_handler(self, ui_settings, sfml_event);
        events.append(&mut event.0);
        self.rerender |= event.1;

        let mut event = self.inner_element.event_handler(ui_settings, sfml_event);
        events.append(&mut event.0);
        self.rerender |= event.1;

        (events, self.rerender)
    }

    fn update(
        &mut self,
        resource_manager: &utils::resource_manager::ResourceManager,
    ) -> (Vec<Event>, bool) {
        let mut rerender = self.rerender;
        let mut events = Vec::new();
        for primitive in self.compact_primitive_mut() {
            let mut event = primitive.update(resource_manager);
            rerender |= event.1;
            events.append(&mut event.0);
        }

        let mut event = self.inner_element.update(resource_manager);
        events.append(&mut event.0);
        rerender |= event.1;

        self.rerender = rerender;
        (events, rerender)
    }

    fn sync_id(&self) -> crate::syncs::SyncId {
        0
    }

    fn event_id(&self) -> EventId {
        self.event_id
    }
}
