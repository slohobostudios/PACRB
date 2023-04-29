use sfml::{
    graphics::{FloatRect, IntRect, PrimitiveType, RenderStates, RenderTarget, RenderWindow},
    system::{Vector2, Vector2i},
    window::Event as SFMLEvent,
};
use std::str::FromStr;
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    events::Event,
    ui_settings::UISettings,
    utils::positioning::UIPosition,
};
use utils::{quads::Quad, resource_manager::ResourceManager};

use crate::pallete_builder::hsv_color::Hsv;

use self::hsv_selector_content::{perform_events, sync_events};

mod hsv_selector_content;

pub struct HSVSelector {
    hsi_selector_dom: DomController,
    current_color_rect: Quad,
    display_current_color: bool,
    current_color: Hsv,
    hex_string: String,
    current_aspect_ratio: Vector2i,
}

impl HSVSelector {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let mut hsis = Self {
            hsi_selector_dom: DomController::new(
                resource_manager,
                ui_settings,
                include_str!("hsv_selector/hsv_selector_content.xml"),
            ),
            current_color: Hsv {
                h: 0,
                s: 255 / 2,
                v: 255 / 2,
            },
            display_current_color: false,
            current_color_rect: Quad::from(FloatRect::new(0., 0., 32., 32.)),
            hex_string: Default::default(),
            current_aspect_ratio: ui_settings.aspect_ratio.computed_resolution().as_other(),
        };
        sync_events(
            &mut hsis.hsi_selector_dom,
            hsis.current_color,
            &hsis.hex_string,
        );
        hsis.current_color_rect
            .set_quad_to_one_color(hsis.current_color.into());

        hsis
    }

    fn color_current_color_rect(&mut self) {
        let rect = Quad::into_rect(&self.current_color_rect);
        self.current_color_rect.mut_quad_positions_to_rect(
            UIPosition::from_str("r:22,b:192")
                .unwrap()
                .center_with_size(
                    IntRect::from_vecs(Vector2::new(0, 0), self.current_aspect_ratio),
                    rect.size().as_other(),
                )
                .as_other(),
        );
        self.current_color_rect
            .set_quad_to_one_color(self.current_color.into());
    }

    pub fn set_hsv_color(&mut self, hsv: Hsv) {
        self.current_color = hsv;
        self.color_current_color_rect();

        self.hex_string = hsv.to_string();
        sync_events(
            &mut self.hsi_selector_dom,
            self.current_color,
            &self.hex_string,
        );
    }

    pub fn curr_color(&self) -> Hsv {
        self.current_color
    }
}

impl DomControllerInterface for HSVSelector {
    fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event> {
        self.current_aspect_ratio = ui_settings.aspect_ratio.computed_resolution().as_other();
        let events = self
            .hsi_selector_dom
            .event_handler(window, ui_settings, event);
        perform_events(
            &events,
            &mut self.hsi_selector_dom,
            &mut self.current_color,
            &mut self.hex_string,
        );
        self.color_current_color_rect();
        self.display_current_color = true;
        if events.is_empty() {
            self.display_current_color = false;
        }
        events
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        self.hsi_selector_dom.update(resource_manager)
    }

    fn render(&mut self, window: &mut RenderWindow) {
        self.hsi_selector_dom.render(window);

        if self.display_current_color {
            let rs = RenderStates::default();
            window.draw_primitives(&self.current_color_rect.0, PrimitiveType::QUADS, &rs);
        }
    }
}
