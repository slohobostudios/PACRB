use crate::elements::traits::Element;
use sfml::system::Vector2u;
use std::ops::Deref;

pub trait TilingSprite {
    fn set_desired_size(&mut self, desired_size: Vector2u);
    fn desired_size(&self) -> Vector2u;
    fn box_clone(&self) -> Box<dyn TilingSprite>;
}

impl Clone for Box<dyn TilingSprite> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

use std::fmt::Debug;
pub trait TilingSpriteElement: TilingSprite + Element + Debug {
    fn box_clone(&self) -> Box<dyn TilingSpriteElement>;
}

impl Clone for Box<dyn TilingSpriteElement> {
    fn clone(&self) -> Self {
        TilingSpriteElement::box_clone(self.deref())
    }
}
