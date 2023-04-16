use sfml::{
    system::Vector2,
    window::{mouse::Button, Event},
};

use super::{
    color_grid::{color_cell::RcColorCell, undo_redo::UndoRedoCell, ColorGrid},
    hover_handler::HoverHandler,
    hsv_color::Hsv,
    ui_components::{erase_mode::EraseMode, hsv_selector::HSVSelector},
};

pub struct NormalModeEventHandlerArguments<'color_grid, 'undo_redo, 'hsv_selector, 'erase_mode> {
    color_grid: &'color_grid mut ColorGrid,
    event: Event,
    hsv_selector: &'hsv_selector HSVSelector,
    erase_mode: &'erase_mode EraseMode,
    undo_redo: &'undo_redo mut UndoRedoCell,
}

impl<'color_grid, 'undo_redo, 'hsv_selector, 'erase_mode>
    NormalModeEventHandlerArguments<'color_grid, 'undo_redo, 'hsv_selector, 'erase_mode>
{
    pub fn new(
        color_grid: &'color_grid mut ColorGrid,
        event: Event,
        hsv_selector: &'hsv_selector HSVSelector,
        erase_mode: &'erase_mode EraseMode,
        undo_redo: &'undo_redo mut UndoRedoCell,
    ) -> Self {
        Self {
            color_grid,
            event,
            hsv_selector,
            erase_mode,
            undo_redo,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct NormalMode {
    hover_handler: HoverHandler,
    is_dragging_cell: bool,
    color_being_dragged: Hsv,
    cells_dragged_over: Vec<(Hsv, bool, RcColorCell)>,
}

impl NormalMode {
    pub fn event_handler(&mut self, args: &mut NormalModeEventHandlerArguments) {
        self.hover_handler
            .event_handler(args.event, args.color_grid);
        if !args.erase_mode.erase_mode_enabled() {
            self.mouse_actions_handler(args);
        }
    }

    fn mouse_actions_handler(&mut self, args: &mut NormalModeEventHandlerArguments) {
        fn is_draw_full_cell_enabled(color_cell: Option<RcColorCell>) -> bool {
            if let Some(color_cell) = color_cell {
                color_cell.borrow().draw_full_cell()
            } else {
                false
            }
        }

        fn empty_cells_dragged_over(
            cells_dragged_over: &mut Vec<(Hsv, bool, RcColorCell)>,
            undo_redo: &mut UndoRedoCell,
        ) {
            for (hsv, is_full_cell, cell) in &mut *cells_dragged_over {
                let cell = &mut cell.borrow_mut();

                if *is_full_cell {
                    cell.fill_the_cell(undo_redo, *hsv);
                } else {
                    cell.empty_the_cell(undo_redo);
                }
            }

            cells_dragged_over.clear();
            if cells_dragged_over.capacity() > usize::from(u8::MAX) {
                cells_dragged_over.shrink_to(0);
            }
        }
        match args.event {
            // Insert new color
            Event::MouseButtonReleased { button, x, y }
                if !self.is_dragging_cell && button == Button::Left =>
            {
                if let Some(color_cell) = args.color_grid.coord_to_cell_mut(Vector2::new(x, y)) {
                    color_cell
                        .borrow_mut()
                        .fill_the_cell(args.undo_redo, args.hsv_selector.curr_color())
                }
            }

            // Start dragging color
            Event::MouseButtonPressed { button, x, y }
                if button == Button::Left
                    && is_draw_full_cell_enabled(
                        args.color_grid.coord_to_cell_mut(Vector2::new(x, y)),
                    ) =>
            {
                self.is_dragging_cell = true;
                if let Some(color_cell) = args.color_grid.coord_to_cell_mut(Vector2::new(x, y)) {
                    self.color_being_dragged = color_cell.borrow().full_cell_current_color();
                    self.cells_dragged_over.push((
                        color_cell.borrow().full_cell_current_color(),
                        false,
                        color_cell.clone(),
                    ));
                    color_cell
                        .borrow_mut()
                        .fill_the_cell(args.undo_redo, self.color_being_dragged);
                }
            }
            // Dragging color
            Event::MouseMoved { x, y } if self.is_dragging_cell => {
                empty_cells_dragged_over(&mut self.cells_dragged_over, args.undo_redo);
                if let Some(color_cell) = args.color_grid.coord_to_cell_mut(Vector2::new(x, y)) {
                    self.cells_dragged_over.push((
                        color_cell.borrow().full_cell_current_color(),
                        color_cell.borrow().draw_full_cell(),
                        color_cell.clone(),
                    ));
                    color_cell
                        .borrow_mut()
                        .fill_the_cell(args.undo_redo, self.color_being_dragged);
                }
            }
            // Finish dragging color
            Event::MouseButtonReleased { button: _, x, y } if self.is_dragging_cell => {
                let last_dragged_over_cell_color =
                    if let Some(tuple) = self.cells_dragged_over.last() {
                        tuple.0
                    } else {
                        Default::default()
                    };
                empty_cells_dragged_over(&mut self.cells_dragged_over, args.undo_redo);

                if let Some(color_cell) = args.color_grid.coord_to_cell_mut(Vector2::new(x, y)) {
                    color_cell
                        .borrow_mut()
                        .fill_the_cell(args.undo_redo, last_dragged_over_cell_color);
                    color_cell
                        .borrow_mut()
                        .fill_the_cell(args.undo_redo, self.color_being_dragged);
                }

                self.is_dragging_cell = false;
            }
            _ => {}
        }
    }
}

impl Drop for NormalMode {
    fn drop(&mut self) {
        self.hover_handler.unhover_all_cells();
    }
}
