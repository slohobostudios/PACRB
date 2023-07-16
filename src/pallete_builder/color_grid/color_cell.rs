use std::{cell::RefCell, rc::Rc};

use sfml::{
    graphics::{IntRect, RenderWindow},
    system::{Vector2, Vector2i, Vector2u},
};
use utils::sfml_util_functions::vector2i_from_vector2u;

use crate::pallete_builder::hsv_color::Hsv;

use self::{empty_cell::EmptyCell, full_cell::FullCell};

use super::undo_redo::UndoRedoCell;

mod empty_cell;
mod full_cell;

pub const CELL_SIZE: Vector2u = Vector2::new(32, 32);

pub type RcColorCell = Rc<RefCell<ColorCell>>;

#[derive(Clone, Debug, PartialEq)]
pub struct ColorCell {
    pub(super) global_bounds: IntRect,
    pub(super) empty_cell: EmptyCell,
    pub(super) full_cell: FullCell,
    pub(super) draw_full_cell: bool,
}

impl ColorCell {
    pub fn new(position: Vector2i) -> Self {
        let global_bounds = IntRect::from_vecs(position, vector2i_from_vector2u(CELL_SIZE));
        Self {
            empty_cell: EmptyCell::new(global_bounds),
            full_cell: FullCell::new(global_bounds),
            draw_full_cell: false,
            global_bounds,
        }
    }

    pub fn global_bounds(&self) -> IntRect {
        self.global_bounds
    }

    pub fn update(&mut self) {
        if !self.draw_full_cell {
            self.empty_cell.update();
        }
    }

    pub fn render(&self, window: &mut RenderWindow) {
        if !self.draw_full_cell {
            self.empty_cell.render(window);
        } else {
            self.full_cell.render(window);
        }
    }
}

impl ColorCell {
    pub fn set_hover(&mut self, hover: bool) {
        self.empty_cell.is_hover = hover
    }

    pub fn draw_full_cell(&self) -> bool {
        self.draw_full_cell
    }

    pub fn empty_the_cell(&mut self, undo_redo: &mut UndoRedoCell) {
        let old_color_cell = self.clone();
        self.draw_full_cell = false;
        self.full_cell.set_color(Default::default());
        undo_redo.change_made(old_color_cell, self.clone());
    }

    // pub fn empty_the_cell_no_undo_redo(&mut self) {
    //     self.draw_full_cell = false;
    //     self.full_cell.set_color(Default::default());
    // }

    pub fn fill_the_cell(&mut self, undo_redo: &mut UndoRedoCell, new_color: Hsv) {
        self.empty_cell.is_hover = false;
        let old_color_cell = self.clone();
        self.full_cell.set_color(new_color);
        self.draw_full_cell = true;
        undo_redo.change_made(old_color_cell, self.clone());
    }

    // pub fn fill_the_cell_no_undo_redo(&mut self, new_color: HSV) {
    //     self.empty_cell.is_hover = false;
    //     self.full_cell.set_color(new_color);
    //     self.draw_full_cell = true;
    // }

    pub fn full_cell_current_color(&self) -> Hsv {
        self.full_cell.current_color()
    }
}
