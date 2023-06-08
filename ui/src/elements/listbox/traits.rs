use crate::elements::traits::ActionableElement;
use std::{fmt::Debug, ops::Deref};

pub trait ListBox: ActionableElement + Debug {
    fn box_clone(&self) -> Box<dyn ListBox>;
}

impl Clone for Box<dyn ListBox> {
    fn clone(&self) -> Self {
        ListBox::box_clone(self.deref())
    }
}
