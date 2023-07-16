use ui::{
    dom_controller::{DomController, DomControllerInterface},
    ui_settings::UISettings,
};
use utils::resource_manager::ResourceManager;

use self::current_quick_save_file_content::sync_events;

mod current_quick_save_file_content;

#[derive(Debug)]
pub struct CurrentQuickSaveFile(DomController);

impl CurrentQuickSaveFile {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        CurrentQuickSaveFile(DomController::new(
            resource_manager,
            ui_settings,
            include_str!("current_quick_save_file/current_quick_save_file_content.xml"),
        ))
    }

    pub fn set_current_quick_save_file(&mut self, current_save_file: &str) {
        sync_events(&mut self.0, current_save_file)
    }

    pub fn current_quick_save_file(&self) -> String {
        current_quick_save_file_content::current_save_file(&self.0)
    }
}

impl DomControllerInterface for CurrentQuickSaveFile {
    fn render(&mut self, window: &mut sfml::graphics::RenderWindow) {
        self.0.render(window)
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<ui::events::Event> {
        self.0.update(resource_manager)
    }

    fn event_handler(
        &mut self,
        window: &mut sfml::graphics::RenderWindow,
        ui_settings: &mut UISettings,
        event: sfml::window::Event,
    ) -> Vec<ui::events::Event> {
        self.0.event_handler(window, ui_settings, event)
    }
}
