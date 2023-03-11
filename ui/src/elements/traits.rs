use crate::{events::Event, ui_settings::UISettings, utils::positioning::UIPosition};
use sfml::{
    graphics::{IntRect, RenderTexture},
    window::Event as SFMLEvent,
};
use utils::resource_manager::ResourceManager;

pub trait Element {
    /// Gets the global bounds of an element
    fn global_bounds(&self) -> IntRect;

    /// Handles events from SFML and returns a linked list of the events that has occured
    #[allow(unused_variables)]
    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        Default::default()
    }

    /// Updates an elements size
    fn update_size(&mut self);

    /// Updates an elements position relative to the outer element
    fn update_position(&mut self, relative_rect: IntRect);

    /// Allows us to set a new ui position to the component
    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect);

    /// Runs an update event on the element
    /// Returns a linked list of events that has occured
    #[allow(unused_variables)]
    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        Default::default()
    }

    /// Render the element
    fn render(&mut self, render_texture: &mut RenderTexture);
    fn box_clone(&self) -> Box<dyn Element>;

    /// Returns the id of the event that has occured. 0 for no matching event id
    fn event_id(&self) -> u16 {
        0
    }

    /// Returns the id of the synchronization that needs to occur. 0 for no matching sync id
    fn sync_id(&self) -> u16 {
        0
    }
}

impl Clone for Box<dyn Element> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}
