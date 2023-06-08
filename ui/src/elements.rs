pub mod background;
pub mod button;
pub mod grouping;
pub mod listbox;
pub mod misc;
pub mod missing_texture;
pub mod root_node;
pub mod slider;
pub mod textbox;
pub mod tiling_sprites;
pub mod traits;

#[derive(Clone, Debug, Default)]
pub enum Element {
    MissingTexture(missing_texture::MissingTexture),
    Button(Box<dyn button::traits::Button>),
    Slider(Box<dyn slider::traits::Slider>),
    TextBox(Box<dyn textbox::traits::TextBox>),
    ListBox(Box<dyn listbox::traits::ListBox>),
    TilingSprite(Box<dyn tiling_sprites::traits::TilingSpriteElement>),
    Background(Box<dyn background::traits::BackgroundElement>),
    Div(div::Div),
    Grid(grid::Grid),
    Sets(sets::Sets),
    Text(text::Text),
    Primitive(primitive::Primitive),
    Image(image::Image),
    RootNode(root_node::RootNode),
    #[default]
    Empty,
}

impl Element {
    pub const fn repr(&self) -> &'static str {
        use Element::*;
        match self {
            MissingTexture(_) => "MissingTexture",
            Button(_) => "Button",
            Slider(_) => "Slider",
            TextBox(_) => "TextBox",
            ListBox(_) => "ListBox",
            TilingSprite(_) => "TilingSprite",
            Background(_) => "Background",
            Div(_) => "Div",
            Grid(_) => "Grid",
            Sets(_) => "Sets",
            Text(_) => "Text",
            Primitive(_) => "Primitive",
            Image(_) => "Image",
            RootNode(_) => "RootNode",
            Empty => "Empty",
        }
    }

    fn get_ele_with_element_trait(&self) -> Option<&dyn traits::Element> {
        use Element::*;
        match self {
            MissingTexture(ele) => Some(ele),
            Button(ele) => Some(ele.as_element()),
            Slider(ele) => Some(ele.as_element()),
            TextBox(ele) => Some(ele.as_element()),
            ListBox(ele) => Some(ele.as_element()),
            TilingSprite(ele) => Some(ele.as_element()),
            Background(ele) => Some(ele.as_element()),
            Div(ele) => Some(ele),
            Grid(ele) => Some(ele),
            Sets(ele) => Some(ele),
            Text(ele) => Some(ele),
            Primitive(ele) => Some(ele),
            Image(ele) => Some(ele),
            RootNode(ele) => Some(ele),
            Empty => None,
        }
    }

    fn get_mut_ele_with_element_trait(&mut self) -> Option<&mut dyn traits::Element> {
        use Element::*;
        match self {
            MissingTexture(ele) => Some(ele),
            Button(ele) => Some(ele.as_mut_element()),
            Slider(ele) => Some(ele.as_mut_element()),
            TextBox(ele) => Some(ele.as_mut_element()),
            ListBox(ele) => Some(ele.as_mut_element()),
            TilingSprite(ele) => Some(ele.as_mut_element()),
            Background(ele) => Some(ele.as_mut_element()),
            Div(ele) => Some(ele),
            Grid(ele) => Some(ele),
            Sets(ele) => Some(ele),
            Text(ele) => Some(ele),
            Primitive(ele) => Some(ele),
            Image(ele) => Some(ele),
            RootNode(ele) => Some(ele),
            Empty => None,
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

use crate::{events::*, syncs::Syncs, ui_settings::UISettings, utils::positioning::UIPosition};
use sfml::{
    graphics::{IntRect, RenderTexture},
    window::Event as SFMLEvent,
};
use utils::resource_manager::ResourceManager;

use self::{
    grouping::{div, grid, sets},
    misc::{image, primitive, text},
    traits::cast_element,
};

impl traits::Element for Element {
    fn global_bounds(&self) -> IntRect {
        if let Some(ele) = self.get_ele_with_element_trait() {
            ele.global_bounds()
        } else {
            Default::default()
        }
    }
    fn event_handler(&mut self, ui_settings: &UISettings, event: SFMLEvent) -> (Vec<Event>, bool) {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.event_handler(ui_settings, event)
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

    fn set_ui_position(&mut self, ui_position: UIPosition, relative_rect: IntRect) {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.set_ui_position(ui_position, relative_rect);
        }
    }

    fn update(&mut self, resource_manager: &ResourceManager) -> (Vec<Event>, bool) {
        let mut rerender = false;
        let mut events = Vec::new();
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            let mut event = ele.update(resource_manager);
            rerender |= event.1;
            events.append(&mut event.0);
        }
        (events, rerender)
    }

    fn render(&mut self, window: &mut RenderTexture) {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.render(window);
        }
    }

    fn sync_id(&self) -> u16 {
        if let Some(ele) = self.get_ele_with_element_trait() {
            ele.sync_id()
        } else {
            0
        }
    }

    fn sync(&mut self, sync: Syncs) {
        if let Some(ele) = self.get_mut_ele_with_element_trait() {
            ele.sync(sync)
        }
    }

    fn event_id(&self) -> EventId {
        if let Some(ele) = self.get_ele_with_element_trait() {
            ele.event_id()
        } else {
            0
        }
    }

    cast_element!();
}
