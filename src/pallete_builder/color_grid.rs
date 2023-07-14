use std::{
    array,
    cell::RefCell,
    ops::{Index, IndexMut},
    rc::Rc,
};

use sfml::{
    graphics::RenderWindow,
    system::{Vector2, Vector2i},
};
use utils::{
    arithmetic_util_functions::u32_from_usize, sfml_util_functions::vector2i_from_vector2u,
};

use self::color_cell::{ColorCell, RcColorCell};

pub mod color_cell;
pub mod load_save;
pub mod undo_redo;

pub const GRID_SIZE: usize = 64;
pub struct ColorGrid(Vec<[RcColorCell; GRID_SIZE]>);

impl ColorGrid {
    pub fn new() -> Self {
        Self(
            (0..GRID_SIZE)
                .map(|i| {
                    array::from_fn(|j| {
                        Rc::new(RefCell::new(ColorCell::new(vector2i_from_vector2u(
                            Vector2::new(
                                color_cell::CELL_SIZE.x * u32_from_usize(i),
                                color_cell::CELL_SIZE.x * u32_from_usize(j),
                            ),
                        ))))
                    })
                })
                .collect(),
        )
    }

    pub fn coord_to_cell_mut(&mut self, coord: Vector2i) -> Option<RcColorCell> {
        let index = self.coord_to_idx(coord)?;
        Some(self.0.get_mut(index.x)?.get_mut(index.y)?.clone())
    }

    pub fn coord_to_idx(&self, coord: Vector2i) -> Option<Vector2<usize>> {
        let top_left_cell_position = self[0][0].borrow().global_bounds().position();
        let cell_size = self[0][0].borrow().global_bounds().size();

        let offset_mouse_pos = coord - top_left_cell_position;

        (offset_mouse_pos.cwise_div(cell_size))
            .try_into_other()
            .ok()
    }

    pub fn is_idx_valid(&self, idx: Vector2<usize>) -> bool {
        fn is_idx_vald_option(self_: &ColorGrid, idx: Vector2<usize>) -> Option<()> {
            self_.0.get(idx.x)?.get(idx.y)?;

            Some(())
        }

        is_idx_vald_option(self, idx).is_some()
    }

    pub fn iter(&self) -> impl Iterator<Item = &RcColorCell> {
        self.0.iter().flatten()
    }

    pub fn update(&mut self) {
        for cell in self.iter() {
            cell.borrow_mut().update();
        }
    }

    pub fn render(&self, window: &mut RenderWindow) {
        for cell in self.iter() {
            cell.borrow().render(window);
        }
    }
}

impl Index<usize> for ColorGrid {
    type Output = [RcColorCell; GRID_SIZE];
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for ColorGrid {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}
