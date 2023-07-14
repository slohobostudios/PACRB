use sfml::{
    graphics::RenderWindow,
    window::{Event as SFMLEvent, Key},
};
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    events::Event,
    ui_settings::UISettings,
};
use utils::resource_manager::ResourceManager;

use self::erase_mode_content::{perform_events, sync_events};

mod erase_mode_content;

#[derive(Debug)]
pub struct EraseMode {
    erase_disabled: bool,
    erase_mode_dom: DomController,
}

impl EraseMode {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let mut em = Self {
            erase_disabled: true,
            erase_mode_dom: DomController::new(
                resource_manager,
                ui_settings,
                include_str!("erase_mode/erase_mode_content.xml"),
            ),
        };
        sync_events(&mut em.erase_mode_dom, em.erase_disabled);

        em
    }
    pub fn erase_mode_enabled(&self) -> bool {
        !self.erase_disabled
    }
}

impl DomControllerInterface for EraseMode {
    fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event> {
        let events = self
            .erase_mode_dom
            .event_handler(window, ui_settings, event);
        perform_events(&events, &mut self.erase_disabled);
        match event {
            SFMLEvent::KeyPressed {
                code,
                ctrl,
                alt,
                system,
                ..
            } if code == Key::E && !ctrl && !alt && !system => {
                self.erase_disabled = !self.erase_disabled;
                sync_events(&mut self.erase_mode_dom, self.erase_disabled);
            }
            _ => {}
        };
        events
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        self.erase_mode_dom.update(resource_manager)
    }

    fn render(&mut self, window: &mut RenderWindow) {
        self.erase_mode_dom.render(window);
    }
}
