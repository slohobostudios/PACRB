use crate::{
    assets::resource_manager::ResourceManager,
    ui::{
        dom_controller::{DomController, DomControllerInterface},
        events::*,
        ui_settings::UISettings,
    },
};
use sfml::{graphics::RenderWindow, window::Event as SFMLEvent};

mod config_selector_content;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Config {
    pub auto_ramping: bool,
    pub num_of_shades: u8,
    pub hue_shift: i8,
    pub saturation_shift: i8,
    pub value_shift: i8,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_ramping: false,
            num_of_shades: 4,
            hue_shift: 0,
            saturation_shift: 0,
            value_shift: 0,
        }
    }
}

#[derive(Default, Debug)]
pub struct ConfigSelector {
    config_selector_dom: DomController,
    current_config: Config,
    config_changed: bool,
}

impl ConfigSelector {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let mut config_selector_dom = DomController::new(
            &resource_manager,
            &ui_settings,
            include_str!("config_selector/config_selector_content.xml"),
        );
        let new_config = Default::default();
        config_selector_content::sync_events(&mut config_selector_dom, &new_config);

        Self {
            config_selector_dom,
            current_config: new_config,
            config_changed: true,
        }
    }

    pub fn current_config(&self) -> Config {
        self.current_config
    }
}

impl DomControllerInterface for ConfigSelector {
    fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event> {
        let previous_config = self.current_config;
        let events = self
            .config_selector_dom
            .event_handler(window, ui_settings, event);
        config_selector_content::perform_events(&events, &mut self.current_config);
        self.config_changed = previous_config != self.current_config;
        events
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        self.config_selector_dom.update(&resource_manager)
    }

    fn render(&mut self, window: &mut RenderWindow) {
        self.config_selector_dom.render(window);
    }
}
