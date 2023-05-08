use sfml::{
    graphics::{IntRect, RenderTexture},
    system::Vector2u,
    window::Event as SFMLEvent,
};
use tracing::{error, warn};
use utils::resource_manager::ResourceManager;

use crate::{
    elements::{
        traits::{cast_element, Element as ElementTrait},
        Element,
    },
    events::Event,
    syncs::SyncId,
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};

use super::div::Div;

#[derive(Clone, Default, Debug)]
pub struct Sets {
    sets: Vec<Vec<Element>>,
    div: Div,
    relative_rect: IntRect,
    sync_id: SyncId,
}

impl Sets {
    pub fn new(
        position: UIPosition,
        mut sets: Vec<Vec<Element>>,
        padding: Option<UIPosition>,
        size: Option<Vector2u>,
        sync_id: SyncId,
    ) -> Self {
        if sets.is_empty() {
            warn!("Number of sets is 0! Making sets have 1 element!");
            sets = vec![vec![Default::default()]];
        }
        let div = Div::new(position, sets[0].clone(), padding, size);

        let mut s = Sets {
            sets,
            div,
            relative_rect: Default::default(),
            sync_id,
        };
        s.update_size();

        s
    }

    pub fn set_current_set(&mut self, new_set: usize) {
        if self.sets.len() <= new_set {
            error!(
                "new_set number is invalid. Max set number: {}, provided set number: {}",
                self.sets.len(),
                new_set
            );
            return;
        }
        let mut div = Div::new(
            self.div.position(),
            self.sets[new_set].clone(),
            self.div.padding(),
            self.div.size(),
        );
        div.update_size();
        div.update_position(self.relative_rect);
        self.div = div;
    }
}

impl ElementTrait for Sets {
    cast_element!();

    fn global_bounds(&self) -> IntRect {
        self.div.global_bounds()
    }

    fn update_size(&mut self) {
        self.div.update_size();
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        self.div.update_position(relative_rect);
        self.relative_rect = relative_rect;
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        self.div.set_ui_position(ui_position, relative_rect);
    }

    fn render(&mut self, render_texture: &mut RenderTexture) {
        self.div.render(render_texture);
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        self.div.event_handler(ui_settings, event)
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        self.div.update(resource_manager)
    }

    fn sync_id(&self) -> SyncId {
        self.sync_id
    }
}
