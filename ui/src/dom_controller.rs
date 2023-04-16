use crate::{
    dom_loader::dom_loader,
    elements::{traits::Element as ElementTrait, Element},
    events::*,
    ui_settings::UISettings,
};
use sfml::{
    graphics::{Color, Rect, RenderTarget, RenderTexture, RenderWindow, Sprite, View},
    window::Event as SFMLEvent,
    SfBox,
};
use utils::{resource_manager::ResourceManager, vector_to_rect_with_zeroed_origin};

#[derive(Default, Debug)]
pub struct DomController {
    pub root_node: Element,
    render_texture: Option<RenderTexture>,
    needs_rerender: bool,
    view: SfBox<View>,
}

impl DomController {
    pub fn new(
        resource_manager: &ResourceManager,
        ui_settings: &UISettings,
        xml_doc: &str,
    ) -> Self {
        let view_size =
            vector_to_rect_with_zeroed_origin!(f32, ui_settings.aspect_ratio.computed_resolution());
        Self {
            root_node: Element::RootNode(dom_loader(
                resource_manager,
                view_size.as_other(),
                xml_doc,
            )),
            view: View::from_rect(view_size),
            needs_rerender: true,
            render_texture: RenderTexture::new(view_size.width as u32, view_size.height as u32),
        }
    }

    pub fn reset_view(&mut self, ui_settings: &UISettings) -> Vec<Event> {
        let view_size =
            vector_to_rect_with_zeroed_origin!(f32, ui_settings.aspect_ratio.computed_resolution());
        let (events, _) = self.root_node.event_handler(
            ui_settings,
            SFMLEvent::Resized {
                width: view_size.width as u32,
                height: view_size.height as u32,
            },
        );
        self.root_node.update_size();
        self.root_node.update_position(view_size.as_other());

        self.view = View::from_rect(view_size);
        self.needs_rerender = true;

        events
    }
}

impl DomControllerInterface for DomController {
    fn event_handler(
        &mut self,
        _window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event> {
        match event {
            SFMLEvent::Resized { .. } => self.reset_view(ui_settings),
            _ => {
                let events = self.root_node.event_handler(ui_settings, event);
                self.needs_rerender |= events.1;
                events.0
            }
        }
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let events = self.root_node.update(resource_manager);
        self.needs_rerender |= events.1;

        events.0
    }

    fn render(&mut self, window: &mut RenderWindow) {
        if self.needs_rerender {
            self.render_texture =
                RenderTexture::new(self.view.size().x as u32, self.view.size().y as u32);
            if let Some(render_texture) = &mut self.render_texture {
                render_texture.set_smooth(false);
                render_texture.clear(Color::TRANSPARENT);
                render_texture.set_view(&self.view);
                self.root_node.render(render_texture);
                render_texture.display();
                self.needs_rerender = false;
            }
        }

        if let Some(render_texture) = &self.render_texture {
            window.set_view(&self.view);
            window.draw(&Sprite::with_texture(render_texture.texture()));
        }
    }
}

pub trait DomControllerInterface {
    fn render(&mut self, window: &mut RenderWindow);
    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event>;
    fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: SFMLEvent,
    ) -> Vec<Event>;
}
