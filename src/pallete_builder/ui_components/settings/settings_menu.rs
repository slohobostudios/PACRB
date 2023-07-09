use sfml::{graphics::RenderWindow, window::Event as SFMLEvent};
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    events::Event,
    ui_settings::UISettings,
};
use utils::resource_manager::ResourceManager;

use self::settings_menu_content::{perform_events, sync_events};
mod settings_menu_content;

#[derive(Debug, Default)]
pub struct SettingsMenu {
    settings_menu_dom: DomController,
    pub display: bool,
}

impl SettingsMenu {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let mut settings_menu_dom = DomController::new(
            resource_manager,
            ui_settings,
            include_str!("settings_menu/settings_menu_content.xml"),
        );
        sync_events(&mut settings_menu_dom, ui_settings);
        Self {
            settings_menu_dom,
            display: false,
        }
    }
}

impl DomControllerInterface for SettingsMenu {
    fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event> {
        if !self.display && !matches!(event, SFMLEvent::Resized { .. }) {
            return Default::default();
        }
        let events = self
            .settings_menu_dom
            .event_handler(window, ui_settings, event);
        perform_events(&events, window, ui_settings, self);
        events
    }

    fn render(&mut self, window: &mut RenderWindow) {
        if !self.display {
            return;
        }
        self.settings_menu_dom.render(window);
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        if !self.display {
            return Default::default();
        }
        self.settings_menu_dom.update(resource_manager)
    }
}
