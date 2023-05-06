use sfml::{graphics::RenderWindow, window::Event as SFMLEvent};
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    events::Event,
    ui_settings::UISettings,
};
use utils::resource_manager::ResourceManager;

use self::{
    settings_content::{perform_events, sync_events},
    settings_menu::SettingsMenu,
};

mod settings_content;
mod settings_menu;

#[derive(Debug, Default)]
pub struct Settings {
    settings_dom: DomController,
    settings_menu: SettingsMenu,
}

impl Settings {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let mut settings_dom = DomController::new(
            resource_manager,
            ui_settings,
            include_str!("settings/settings_content.xml"),
        );
        sync_events(&mut settings_dom);
        Self {
            settings_dom,
            settings_menu: SettingsMenu::new(resource_manager, ui_settings),
        }
    }

    pub fn is_displayed(&self) -> bool {
        self.settings_menu.display
    }
}

impl DomControllerInterface for Settings {
    fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event> {
        let previous_display_state = self.settings_menu.display;
        let mut events = self.settings_dom.event_handler(window, ui_settings, event);
        let mut settings_menu_events = self.settings_menu.event_handler(window, ui_settings, event);
        perform_events(
            &events,
            window,
            ui_settings,
            &mut self.settings_menu,
            previous_display_state,
        );
        events.append(&mut settings_menu_events);
        events
    }

    fn render(&mut self, window: &mut RenderWindow) {
        self.settings_dom.render(window);
        self.settings_menu.render(window);
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = self.settings_dom.update(resource_manager);
        events.append(&mut self.settings_menu.update(resource_manager));

        events
    }
}
