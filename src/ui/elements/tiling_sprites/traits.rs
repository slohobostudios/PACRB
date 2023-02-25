use crate::{assets::resource_manager::ResourceManager, ui::elements::traits::Element};
use sfml::system::Vector2u;
use std::ops::Deref;

pub trait TilingSprite {
    fn set_desired_size(&mut self, resource_manager: &ResourceManager, desired_size: Vector2u);
    fn box_clone(&self) -> Box<dyn TilingSprite>;
}

impl Clone for Box<dyn TilingSprite> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

use std::fmt::Debug;
pub trait TilingSpriteElement: TilingSprite + Element + Debug {
    fn as_mut_element(&mut self) -> &mut dyn Element;
    fn as_mut_tiling_sprite(&mut self) -> &mut dyn TilingSprite;
    fn as_element(&self) -> &dyn Element;
    fn as_tiling_sprite(&self) -> &dyn TilingSprite;
    fn box_clone(&self) -> Box<dyn TilingSpriteElement>;
}

impl Clone for Box<dyn TilingSpriteElement> {
    fn clone(&self) -> Self {
        TilingSpriteElement::box_clone(self.deref())
    }
}
