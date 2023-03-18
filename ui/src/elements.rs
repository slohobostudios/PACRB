pub mod background;
pub mod button;
pub mod div;
pub mod grid;
pub mod missing_texture;
pub mod root_node;
pub mod slider;
pub mod text;
pub mod textbox;
pub mod tiling_sprites;
pub mod traits;

#[derive(Clone, Debug)]
pub enum Element {
    MissingTexture(missing_texture::MissingTexture),
    Button(Box<dyn button::traits::Button>),
    Slider(Box<dyn slider::traits::Slider>),
    TextBox(Box<dyn textbox::traits::TextBox>),
    TilingSprite(Box<dyn tiling_sprites::traits::TilingSpriteElement>),
    Background(Box<dyn background::traits::BackgroundElement>),
    Div(div::Div),
    Grid(grid::Grid),
    Text(text::Text),
    RootNode(root_node::RootNode),
    Empty(()),
}

impl Element {
    pub fn repr(&self) -> &'static str {
        use Element::*;
        match self {
            MissingTexture(_) => "MissingTexture",
            Button(_) => "Button",
            Slider(_) => "Slider",
            TextBox(_) => "TextBox",
            TilingSprite(_) => "TilingSprite",
            Background(_) => "Background",
            Div(_) => "Div",
            Grid(_) => "Grid",
            Text(_) => "Text",
            RootNode(_) => "RootNode",
            Empty(_) => "Empty",
        }
    }

    fn get_ele_with_element_trait(&self) -> Option<Box<&dyn traits::Element>> {
        use Element::*;
        match self {
            MissingTexture(ele) => Some(Box::new(ele)),
            Button(ele) => Some(Box::new(ele.as_element())),
            Slider(ele) => Some(Box::new(ele.as_element())),
            TextBox(ele) => Some(Box::new(ele.as_element())),
            TilingSprite(ele) => Some(Box::new(ele.as_element())),
            Background(ele) => Some(Box::new(ele.as_element())),
            Div(ele) => Some(Box::new(ele)),
            Grid(ele) => Some(Box::new(ele)),
            Text(ele) => Some(Box::new(ele)),
            RootNode(ele) => Some(Box::new(ele)),
            Empty(_) => None,
        }
    }

    fn get_mut_ele_with_element_trait(&mut self) -> Option<Box<&mut dyn traits::Element>> {
        use Element::*;
        match self {
            MissingTexture(ele) => Some(Box::new(ele)),
            Button(ele) => Some(Box::new(ele.as_mut_element())),
            Slider(ele) => Some(Box::new(ele.as_mut_element())),
            TextBox(ele) => Some(Box::new(ele.as_mut_element())),
            TilingSprite(ele) => Some(Box::new(ele.as_mut_element())),
            Background(ele) => Some(Box::new(ele.as_mut_element())),
            Div(ele) => Some(Box::new(ele)),
            Grid(ele) => Some(Box::new(ele)),
            Text(ele) => Some(Box::new(ele)),
            RootNode(ele) => Some(Box::new(ele)),
            Empty(_) => None,
        }
    }

    pub fn traverse_dom_mut<F: FnMut(&mut Element)>(&mut self, sync_element: &mut F) {
        sync_element(self);

        use Element::*;
        match self {
            RootNode(ele) => {
                for ele in ele.mut_children() {
                    ele.traverse_dom_mut(&mut *sync_element)
                }
            }
            Background(ele) => {
                for ele in ele.mut_children() {
                    ele.traverse_dom_mut(&mut *sync_element)
                }
            }
            Grid(ele) => {
                for ele in ele.mut_children() {
                    ele.traverse_dom_mut(&mut *sync_element)
                }
            }
            Div(ele) => {
                for ele in ele.mut_children() {
                    ele.traverse_dom_mut(&mut *sync_element)
                }
            }
            _ => {}
        }
    }
}

impl Default for Element {
    fn default() -> Self {
        Self::Empty(())
    }
}

use crate::{events::*, ui_settings::UISettings, utils::positioning::UIPosition};
use sfml::{
    graphics::{IntRect, RenderTexture},
    window::Event as SFMLEvent,
};
use utils::resource_manager::ResourceManager;

use self::traits::cast_element;

impl traits::Element for Element {
    cast_element!();
    fn global_bounds(&self) -> IntRect {
        if let Some(ele) = self.get_ele_with_element_trait() {
            ele.global_bounds()
        } else {
            Default::default()
        }
    }

    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> Vec<Event> {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.event_handler(&ui_settings, event)
        } else {
            Default::default()
        }
    }

    fn update_size(&mut self) {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.update_size();
        }
    }

    fn update_position(&mut self, relative_rect: IntRect) {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.update_position(relative_rect);
        }
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> Vec<Event> {
        let mut events = Vec::new();
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            events.append(&mut ele.update(&resource_manager));
        }
        events
    }

    fn render(&mut self, window: &mut RenderTexture) {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.render(window);
        }
    }

    fn box_clone(&self) -> Box<dyn traits::Element> {
        Box::new(self.clone())
    }

    fn event_id(&self) -> EventId {
        if let Some(ele) = self.get_ele_with_element_trait() {
            ele.event_id()
        } else {
            0
        }
    }

    fn sync_id(&self) -> u16 {
        if let Some(ele) = self.get_ele_with_element_trait() {
            ele.sync_id()
        } else {
            0
        }
    }

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.set_ui_position(ui_position, relative_rect);
        }
    }
}
