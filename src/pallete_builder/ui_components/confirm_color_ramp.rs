use sfml::graphics::RenderWindow;

mod confirm_color_ramp_content;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    #[default]
    Vertical,
}

impl Orientation {
    fn swap(&mut self) {
        use Orientation::{Horizontal, Vertical};
        *self = match self {
            Horizontal => Vertical,
            Vertical => Horizontal,
        };
    }
}

#[derive(Debug, Default)]
pub struct ConfirmColorRamp {
    confirm_color_ramp_dom: DomController,
    enable: bool,
    orientation: Orientation,
}

impl ConfirmColorRamp {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let mut confirm_color_ramp_dom = DomController::new(
            resource_manager,
            ui_settings,
            include_str!("confirm_color_ramp/confirm_color_ramp_content.xml"),
        );
        confirm_color_ramp_content::sync_events(&mut confirm_color_ramp_dom, false);

        Self {
            confirm_color_ramp_dom,
            enable: false,
            orientation: Orientation::Vertical,
        }
    }

    pub fn set_enable(&mut self, enable: bool) {
        self.enable = enable;
        confirm_color_ramp_content::sync_events(&mut self.confirm_color_ramp_dom, enable);
    }

    pub fn is_enabled(&self) -> bool {
        self.enable
    }

    pub fn orientation(&self) -> Orientation {
        self.orientation
    }
}

use sfml::window::Event as SFMLEvent;
use ui::dom_controller::{DomController, DomControllerInterface};
use ui::events::Event;
use ui::ui_settings::UISettings;
use utils::resource_manager::ResourceManager;

impl DomControllerInterface for ConfirmColorRamp {
    fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event> {
        if !matches!(event, SFMLEvent::Resized { .. }) && !self.enable {
            return Default::default();
        }
        let events = self
            .confirm_color_ramp_dom
            .event_handler(window, ui_settings, event);
        confirm_color_ramp_content::perform_events(
            &events,
            &mut self.enable,
            &mut self.orientation,
        );
        events
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        if !self.enable {
            return Default::default();
        }
        self.confirm_color_ramp_dom.update(resource_manager)
    }

    fn render(&mut self, window: &mut RenderWindow) {
        if self.enable {
            self.confirm_color_ramp_dom.render(window);
        }
    }
}
