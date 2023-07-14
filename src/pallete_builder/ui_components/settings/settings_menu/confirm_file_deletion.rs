use ui::{
    dom_controller::{DomController, DomControllerInterface},
    ui_settings::UISettings,
};
use utils::resource_manager::ResourceManager;

use crate::pallete_builder::color_grid::load_save::remove_pacrb_file;

use self::confirm_file_deletion_content::{perform_events, sync_events};

pub mod confirm_file_deletion_content;

#[derive(Debug, Default, Copy, Clone)]
pub enum ConfirmFileDeletionSelection {
    #[default]
    None,
    Delete,
    Cancel,
}

#[derive(Default, Debug)]
pub struct ConfirmFileDeletion {
    file_to_delete: Option<String>,
    confirm_file_deletion_selection: ConfirmFileDeletionSelection,
    confirm_file_deletion_dom: DomController,
    display: bool,
}

impl ConfirmFileDeletion {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let confirm_file_deletion_dom = DomController::new(
            resource_manager,
            ui_settings,
            include_str!("confirm_file_deletion/confirm_file_deletion_content.xml"),
        );
        Self {
            confirm_file_deletion_dom,
            ..Default::default()
        }
    }

    pub fn set_file_to_delete(&mut self, file: &str) {
        self.file_to_delete = Some(file.to_owned());
        sync_events(&mut self.confirm_file_deletion_dom, file);
    }

    pub fn file_deletion_selection(&self) -> ConfirmFileDeletionSelection {
        self.confirm_file_deletion_selection
    }

    pub fn is_displaying(&self) -> bool {
        self.display
    }

    pub fn set_display(&mut self, display: bool) {
        self.display = display;

        self.confirm_file_deletion_selection = ConfirmFileDeletionSelection::None;
    }

    fn remove_file(&self) {
        if let Some(file_to_delete) = &self.file_to_delete {
            remove_pacrb_file(file_to_delete);
        }
    }
}

impl DomControllerInterface for ConfirmFileDeletion {
    fn render(&mut self, window: &mut sfml::graphics::RenderWindow) {
        if !self.display {
            return;
        }

        self.confirm_file_deletion_dom.render(window);
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<ui::events::Event> {
        if !self.display {
            return Default::default();
        }

        self.confirm_file_deletion_dom.update(resource_manager)
    }

    fn event_handler(
        &mut self,
        window: &mut sfml::graphics::RenderWindow,
        ui_settings: &mut UISettings,
        event: sfml::window::Event,
    ) -> Vec<ui::events::Event> {
        if !self.display && !matches!(event, sfml::window::Event::Resized { .. }) {
            return Default::default();
        }
        let events = self
            .confirm_file_deletion_dom
            .event_handler(window, ui_settings, event);
        perform_events(&events, self);
        events
    }
}
