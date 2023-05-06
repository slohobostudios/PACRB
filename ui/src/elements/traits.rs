use crate::{
    events::{Event, EventId},
    syncs::{SyncId, Syncs},
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};
use sfml::{
    graphics::{IntRect, RenderTexture},
    system::Vector2i,
    window::Event as SFMLEvent,
};
use utils::resource_manager::ResourceManager;

pub trait Element {
    /// Gets the global bounds of an element
    fn global_bounds(&self) -> IntRect;

    /// Handles events from SFML and returns a vector of the events that has occured along with
    /// a bool.
    ///
    /// the boolean indicates whether we need a rerender to occur
    #[allow(unused_variables)]
    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        Default::default()
    }

    /// Updates an elements size
    fn update_size(&mut self);

    /// Updates an elements position relative to the outer element
    fn update_position(&mut self, relative_rect: IntRect);

    /// Allows us to set a new ui position to the component
    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect);

    /// Runs an update event on the element
    /// Returns a vector of events that has occured along with a bool
    ///
    /// the boolean indicates whether we need a rerender to occur
    #[allow(unused_variables)]
    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        Default::default()
    }

    /// Render the element
    fn render(&mut self, render_texture: &mut RenderTexture);

    /// Returns the id of the synchronization that needs to occur. 0 for no matching sync id
    fn sync_id(&self) -> SyncId {
        0
    }

    /// Returns the id of the event that has occured. 0 for no matching event id
    fn event_id(&self) -> EventId {
        0
    }

    /// Syncs the element via the Sync information
    /// By default does nothing
    ///
    /// WARNING:
    /// This functions may cause side-effects. You should ONLY use it when needed.
    /// Calling this function may cause state changes and put the exterior code in a
    /// incorrect state.
    #[allow(unused_variables)]
    fn sync(&mut self, sync: Syncs) {}

    fn box_clone(&self) -> Box<dyn Element>;

    fn as_element(&self) -> &dyn Element;

    fn as_mut_element(&mut self) -> &mut dyn Element;
}

/// Auto import as casting element functions
macro_rules! cast_element {
    () => {
        fn as_element(&self) -> &dyn crate::elements::traits::Element {
            self
        }
        fn as_mut_element(&mut self) -> &mut dyn crate::elements::traits::Element {
            self
        }
        fn box_clone(&self) -> Box<dyn crate::elements::traits::Element> {
            Box::new(self.clone())
        }
    };
}
pub(crate) use cast_element;

impl Clone for Box<dyn Element> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

pub trait ActionableElement: Element {
    /// Returns current Event.
    fn triggered_event(&self) -> Event;

    /// Trigger certain events when a certain bind is pressed.
    fn bind_pressed(&mut self, mouse_pos: Vector2i);

    /// Trigger certain events when a certain bind is released.
    fn bind_released(&mut self, mouse_pos: Vector2i);

    /// Sets the element's state based on cursor position.
    fn set_hover(&mut self, mouse_pos: Vector2i);

    /// Returns whether cursor is hovering element
    fn is_hover(&self) -> bool;

    fn as_actionable_element(&self) -> &dyn ActionableElement;
    fn as_mut_actionable_element(&mut self) -> &mut dyn ActionableElement;
}

/// Auto import as casting actionable element functions
macro_rules! cast_actionable_element {
    () => {
        fn as_actionable_element(&self) -> &dyn ActionableElement {
            self
        }
        fn as_mut_actionable_element(&mut self) -> &mut dyn ActionableElement {
            self
        }
    };
}
pub(crate) use cast_actionable_element;
