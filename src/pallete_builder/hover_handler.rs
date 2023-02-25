use sfml::{system::Vector2, window::Event};

use super::color_grid::{color_cell::RcColorCell, ColorGrid};

#[derive(Debug, Clone, Default)]
pub struct HoverHandler {
    cells_in_hover_state: Vec<RcColorCell>,
}

impl HoverHandler {
    pub fn unhover_all_cells(&mut self) {
        for cell in &mut self.cells_in_hover_state {
            cell.borrow_mut().set_hover(false);
        }
        self.cells_in_hover_state.clear();
        if self.cells_in_hover_state.capacity() > usize::from(u8::MAX) {
            self.cells_in_hover_state = Vec::with_capacity(usize::from(u8::MAX))
        }
    }

    pub fn event_handler(&mut self, event: Event, color_grid: &mut ColorGrid) {
        self.unhover_all_cells();
        fn set_mouse_hover(
            color_grid: &mut ColorGrid,
            x: i32,
            y: i32,
            cells_in_hover_state: &mut Vec<RcColorCell>,
        ) {
            if let Some(color_cell) = color_grid.coord_to_cell_mut(Vector2::new(x, y)).clone() {
                cells_in_hover_state.push(color_cell.clone());
                color_cell.borrow_mut().set_hover(true);
            }
        }

        match event {
            Event::MouseMoved { x, y } => {
                set_mouse_hover(color_grid, x, y, &mut self.cells_in_hover_state)
            }
            Event::MouseButtonPressed { button: _, x, y } => {
                set_mouse_hover(color_grid, x, y, &mut self.cells_in_hover_state)
            }
            Event::MouseButtonReleased { button: _, x, y } => {
                set_mouse_hover(color_grid, x, y, &mut self.cells_in_hover_state)
            }
            Event::MouseWheelScrolled {
                wheel: _,
                delta: _,
                x,
                y,
            } => set_mouse_hover(color_grid, x, y, &mut self.cells_in_hover_state),
            _ => {}
        }
    }
}
