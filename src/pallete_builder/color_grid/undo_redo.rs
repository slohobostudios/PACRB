use std::time::{Duration, Instant};

use utils::center_of_rect;

use super::{color_cell::ColorCell, ColorGrid};

const NUM_EVENTS_INCREASE_DURATION: Duration = Duration::from_millis(100);
const NUM_EVENTS_INCREASE_STEPPER: u8 = 8;
#[derive(Debug)]
pub struct UndoRedoCell {
    cell_changes: Vec<(ColorCell, ColorCell)>,
    current_idx: usize,
    last_undo_redo_event: Instant,
    num_of_events: u8,
}

impl UndoRedoCell {
    fn get_num_of_repititions(&mut self) -> u8 {
        if self.last_undo_redo_event.elapsed() < NUM_EVENTS_INCREASE_DURATION {
            self.num_of_events = self.num_of_events.saturating_add(1);
        } else {
            self.num_of_events = 0;
        }
        self.last_undo_redo_event = Instant::now();

        if self.num_of_events < NUM_EVENTS_INCREASE_STEPPER {
            1
        } else if self.num_of_events < NUM_EVENTS_INCREASE_STEPPER * 2 {
            4
        } else if self.num_of_events < NUM_EVENTS_INCREASE_STEPPER * 4 {
            8
        } else if self.num_of_events < NUM_EVENTS_INCREASE_STEPPER * 8 {
            16
        } else {
            32
        }
    }

    fn undo_action(&mut self, color_grid: &mut ColorGrid, repititions: u8) {
        if repititions == 0 {
            return;
        }

        let center_of_current_cell =
            center_of_rect!(i32, self.cell_changes[self.current_idx].0.global_bounds);
        if let Some(color_cell) = color_grid.coord_to_cell_mut(center_of_current_cell) {
            *color_cell.borrow_mut() = self.cell_changes[self.current_idx].0.clone();
            self.current_idx = self.current_idx.checked_sub(1).unwrap_or(self.current_idx);
        }

        self.undo_action(color_grid, repititions - 1);
    }

    pub fn undo(&mut self, color_grid: &mut ColorGrid) {
        let repititions = self.get_num_of_repititions();
        self.undo_action(color_grid, repititions)
    }

    pub fn redo_action(&mut self, color_grid: &mut ColorGrid, repititions: u8) {
        if repititions == 0 {
            return;
        }
        self.current_idx = if self.current_idx + 1 >= self.cell_changes.len() {
            self.cell_changes.len() - 1
        } else {
            self.current_idx + 1
        };
        let center_of_current_cell =
            center_of_rect!(i32, self.cell_changes[self.current_idx].1.global_bounds);
        if let Some(color_cell) = color_grid.coord_to_cell_mut(center_of_current_cell) {
            *color_cell.borrow_mut() = self.cell_changes[self.current_idx].1.clone();
            color_cell.borrow_mut().empty_cell.is_hover = false;
        }

        self.redo_action(color_grid, repititions - 1);
    }

    pub fn redo(&mut self, color_grid: &mut ColorGrid) {
        let repititions = self.get_num_of_repititions();
        self.redo_action(color_grid, repititions)
    }

    /// MUST provide a clone to the previous color cell before the change occurs
    pub fn change_made(&mut self, previous_color_cell: ColorCell, new_color_cell: ColorCell) {
        // If erasing an already erased cell, do not insert into cell_changes
        if !previous_color_cell.draw_full_cell && !new_color_cell.draw_full_cell {
            return;
        }
        self.cell_changes.truncate(self.current_idx + 1);

        self.cell_changes
            .push((previous_color_cell, new_color_cell));
        self.current_idx = self.cell_changes.len() - 1;
    }
}

impl Default for UndoRedoCell {
    fn default() -> Self {
        Self {
            cell_changes: Default::default(),
            current_idx: Default::default(),
            last_undo_redo_event: Instant::now(),
            num_of_events: 0,
        }
    }
}
