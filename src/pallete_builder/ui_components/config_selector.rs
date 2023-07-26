use sfml::{graphics::RenderWindow, window::Event as SFMLEvent};
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    events::Event,
    ui_settings::UISettings,
};
use utils::resource_manager::ResourceManager;

mod config_selector_content;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
}

impl ConfigSelector {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let mut config_selector_dom = DomController::new(
            resource_manager,
            ui_settings,
            include_str!("config_selector/config_selector_content.xml"),
        );
        let new_config = Default::default();
        config_selector_content::sync_events(&mut config_selector_dom, &new_config);

        Self {
            config_selector_dom,
            current_config: new_config,
        }
    }

    pub fn set_config(&mut self, new_config: Config) {
        self.current_config = new_config;
        config_selector_content::sync_events(&mut self.config_selector_dom, &self.current_config);
    }

    pub fn current_config(&self) -> Config {
        self.current_config
    }

    pub fn toggle_auto_ramping(&mut self) {
        self.current_config.auto_ramping = !self.current_config.auto_ramping;
        config_selector_content::sync_events(&mut self.config_selector_dom, &self.current_config);
    }
}

impl DomControllerInterface for ConfigSelector {
    fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event> {
        let events = self
            .config_selector_dom
            .event_handler(window, ui_settings, event);
        config_selector_content::perform_events(&events, self);
        events
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let events = self.config_selector_dom.update(resource_manager);
        config_selector_content::perform_events(&events, self);
        events
    }

    fn render(&mut self, window: &mut RenderWindow) {
        self.config_selector_dom.render(window);
    }
}
