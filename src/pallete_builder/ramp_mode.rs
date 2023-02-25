use sfml::{
    system::Vector2,
    window::{mouse::Button, Event},
};

use super::{
    color_grid::{color_cell::RcColorCell, undo_redo::UndoRedoCell, ColorGrid, GRID_SIZE},
    hover_handler::HoverHandler,
    ui_components::{
        config_selector::ConfigSelector,
        confirm_color_ramp::{ConfirmColorRamp, Orientation},
        erase_mode::EraseMode,
        hsv_selector::HSVSelector,
    },
};

pub struct RampModeEventHandlerArguments<
    'color_grid,
    'undo_redo,
    'hsv_selector,
    'erase_mode,
    'confirm_color_ramp,
    'config_selector,
> {
    color_grid: &'color_grid mut ColorGrid,
    event: Event,
    hsv_selector: &'hsv_selector HSVSelector,
    config_selector: &'config_selector ConfigSelector,
    erase_mode: &'erase_mode EraseMode,
    confirm_color_ramp: &'confirm_color_ramp ConfirmColorRamp,
    undo_redo: &'undo_redo mut UndoRedoCell,
}

impl<
        'color_grid,
        'undo_redo,
        'hsv_selector,
        'erase_mode,
        'confirm_color_ramp,
        'config_selector,
    >
    RampModeEventHandlerArguments<
        'color_grid,
        'undo_redo,
        'hsv_selector,
        'erase_mode,
        'confirm_color_ramp,
        'config_selector,
    >
{
    pub fn new(
        color_grid: &'color_grid mut ColorGrid,
        event: Event,
        hsv_selector: &'hsv_selector HSVSelector,
        erase_mode: &'erase_mode EraseMode,
        undo_redo: &'undo_redo mut UndoRedoCell,
        confirm_color_ramp: &'confirm_color_ramp ConfirmColorRamp,
        config_selector: &'config_selector ConfigSelector,
    ) -> Self {
        Self {
            color_grid,
            event,
            hsv_selector,
            erase_mode,
            undo_redo,
            confirm_color_ramp,
            config_selector,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct RampMode {
    hover_handler: HoverHandler,
    ramp: Vec<RcColorCell>,
}

impl RampMode {
    fn clear_ramp(&mut self, undo_redo: &mut UndoRedoCell) {
        for cell in &mut self.ramp {
            cell.borrow_mut().empty_the_cell(undo_redo);
        }

        self.ramp = Vec::new();
    }

    /// Creates a ramp based on the configuration, current orientation, and possibly selected color
    ///
    /// Returns None on failure
    fn create_ramp(&mut self, args: &mut RampModeEventHandlerArguments) -> Option<()> {
        self.clear_ramp(args.undo_redo);
        let event = if let Event::MouseButtonReleased { button, x, y } = args.event {
            if button != Button::Left {
                return None;
            }
            (button, Vector2::new(x, y))
        } else {
            return None;
        };

        let (min, max) = match args.confirm_color_ramp.orientation() {
            Orientation::Horizontal => {
                let idx = args.color_grid.coord_to_idx(event.1)?;
                let mut right_most_idx = idx.x;
                let mut max_idx =
                    idx.x + usize::from(args.config_selector.current_config().num_of_shades);
                if max_idx >= GRID_SIZE {
                    max_idx = GRID_SIZE - 1
                }

                for x in idx.x..=max_idx {
                    if !args.color_grid.is_idx_valid(Vector2::new(x, idx.y))
                        || args.color_grid[x][idx.y].borrow_mut().draw_full_cell()
                    {
                        break;
                    }
                    right_most_idx += 1
                }

                let mut left_most_idx = idx.x;
                let min_idx = idx.x.saturating_sub(usize::from(
                    args.config_selector.current_config().num_of_shades,
                ));

                for x in idx.x..=min_idx {
                    if !args.color_grid.is_idx_valid(Vector2::new(x, idx.y))
                        || args.color_grid[x][idx.y].borrow_mut().draw_full_cell()
                    {
                        break;
                    }
                    left_most_idx -= 1;
                }
                (
                    Vector2::new(left_most_idx, idx.y),
                    Vector2::new(right_most_idx, idx.y),
                )
            }
            Orientation::Vertical => {
                let idx = args.color_grid.coord_to_idx(event.1)?;
                let mut top_most_idx = idx.y;
                let min_idx = idx.y.saturating_sub(usize::from(
                    args.config_selector.current_config().num_of_shades,
                ));

                for y in idx.y..=min_idx {
                    if !args.color_grid.is_idx_valid(Vector2::new(idx.x, y))
                        || args.color_grid[idx.x][y].borrow_mut().draw_full_cell()
                    {
                        break;
                    }
                    top_most_idx -= 1;
                }

                let mut bottom_most_idx = idx.y;
                let mut max_idx =
                    idx.y + usize::from(args.config_selector.current_config().num_of_shades);
                if max_idx >= GRID_SIZE {
                    max_idx = GRID_SIZE - 1
                }

                for y in idx.y..=max_idx {
                    if !args.color_grid.is_idx_valid(Vector2::new(idx.y, y))
                        || args.color_grid[idx.x][y].borrow_mut().draw_full_cell()
                    {
                        break;
                    }
                    bottom_most_idx -= 1;
                }

                (
                    Vector2::new(idx.x, top_most_idx),
                    Vector2::new(idx.x, bottom_most_idx),
                )
            }
        };

        Some(())
    }

    pub fn event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        if !self.ramp_being_shown() {
            self.no_ramping_event_handler(args);
        } else {
            self.ramping_event_handler(args);
        }
    }

    fn no_ramping_event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        self.hover_handler
            .event_handler(args.event, &mut args.color_grid);

        if args.erase_mode.erase_mode_enabled() {
            return;
        }
    }

    fn ramping_event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        self.hover_handler.unhover_all_cells();
    }

    pub fn ramp_being_shown(&self) -> bool {
        self.ramp.len() != 0
    }
}

impl Drop for RampMode {
    fn drop(&mut self) {
        self.hover_handler.unhover_all_cells();
    }
}
