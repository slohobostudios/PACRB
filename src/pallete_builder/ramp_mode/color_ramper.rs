use sfml::system::{Vector2, Vector2i};

use crate::{
    clamp_to_primitive_bounds,
    pallete_builder::{
        color_grid::{color_cell::RcColorCell, undo_redo::UndoRedoCell},
        hsv_color::HSV,
        ui_components::confirm_color_ramp::Orientation,
    },
};

use super::RampModeEventHandlerArguments;

#[derive(Clone, Debug, Default)]
pub struct ColorRamper {
    min_ramp: Vec<RcColorCell>,
    max_ramp: Vec<RcColorCell>,
}

impl ColorRamper {
    pub fn ramp_being_shown(&self) -> bool {
        self.min_ramp.len() != 0 || self.max_ramp.len() != 0
    }

    pub fn clear_ramp(&mut self, undo_redo: &mut UndoRedoCell) {
        for color_cell in self.min_ramp.iter_mut().chain(self.max_ramp.iter_mut()) {
            color_cell.borrow_mut().empty_the_cell(undo_redo);
        }

        self.min_ramp = Vec::new();
        self.max_ramp = Vec::new();
    }

    /// Creates a ramp based on the configuration, current, orientation, and possibly selected color
    ///
    /// Returns None on failure
    pub fn create_ramp(
        &mut self,
        coord: Vector2i,
        args: &mut RampModeEventHandlerArguments,
    ) -> Option<()> {
        let color_grid = &mut args.color_grid;
        let num_of_shades_per_direction = args.config_selector.current_config().num_of_shades / 2;
        let starting_idx = color_grid.coord_to_idx(coord)?;
        let starting_color = if color_grid[starting_idx.x][starting_idx.y]
            .borrow()
            .draw_full_cell()
        {
            color_grid[starting_idx.x][starting_idx.y]
                .borrow()
                .full_cell_current_color()
        } else {
            args.hsv_selector.curr_color()
        };
        self.min_ramp
            .push(color_grid[starting_idx.x][starting_idx.y].clone());
        self.clear_ramp(args.undo_redo);

        match args.confirm_color_ramp.orientation() {
            Orientation::Horizontal => {
                for i in 0..=num_of_shades_per_direction {
                    let idx = Vector2::new(starting_idx.x - usize::from(i), starting_idx.y);
                    if !color_grid.is_idx_valid(idx)
                        || color_grid[idx.x][idx.y].borrow().draw_full_cell()
                    {
                        break;
                    }
                    self.min_ramp.push(color_grid[idx.x][idx.y].clone());
                }
                for i in 0..=num_of_shades_per_direction {
                    let idx = Vector2::new(starting_idx.x + usize::from(i), starting_idx.y);
                    if !color_grid.is_idx_valid(idx)
                        || color_grid[idx.x][idx.y].borrow().draw_full_cell()
                    {
                        break;
                    }
                    self.max_ramp.push(color_grid[idx.x][idx.y].clone());
                }
            }
            Orientation::Vertical => {
                for i in 0..=num_of_shades_per_direction {
                    let idx = Vector2::new(starting_idx.x, starting_idx.y - usize::from(i));
                    if !color_grid.is_idx_valid(idx)
                        || color_grid[idx.x][idx.y].borrow().draw_full_cell()
                    {
                        break;
                    }
                    self.min_ramp.push(color_grid[idx.x][idx.y].clone());
                }
                for i in 0..=num_of_shades_per_direction {
                    let idx = Vector2::new(starting_idx.x, starting_idx.y + usize::from(i));
                    if !color_grid.is_idx_valid(idx)
                        || color_grid[idx.x][idx.y].borrow().draw_full_cell()
                    {
                        break;
                    }
                    self.max_ramp.push(color_grid[idx.x][idx.y].clone());
                }
            }
        }
        self.min_ramp
            .first_mut()?
            .borrow_mut()
            .fill_the_cell(args.undo_redo, starting_color);
        self.max_ramp
            .first_mut()?
            .borrow_mut()
            .fill_the_cell(args.undo_redo, starting_color);

        self.color_the_ramp(args);

        Some(())
    }

    pub fn color_the_ramp(&mut self, args: &mut RampModeEventHandlerArguments) {
        let config = args.config_selector.current_config();
        let Some(first_cell) = self.min_ramp.first() else { return; };
        let starting_color = first_cell.borrow().full_cell_current_color();

        for (i, color_cell) in self.min_ramp.iter_mut().enumerate() {
            let i = i as i16;
            let h = starting_color.h - i * i16::from(config.hue_shift);
            let s = clamp_to_primitive_bounds!(
                u8,
                i16::from(starting_color.s) - i * i16::from(config.saturation_shift)
            );
            let v = clamp_to_primitive_bounds!(
                u8,
                i16::from(starting_color.v) - i * i16::from(config.value_shift)
            );
            color_cell
                .borrow_mut()
                .fill_the_cell(args.undo_redo, HSV { h, s, v })
        }
        for (i, color_cell) in self.max_ramp.iter_mut().enumerate() {
            let i = i as i16;
            let h = starting_color.h + i * i16::from(config.hue_shift);
            let s = clamp_to_primitive_bounds!(
                u8,
                i16::from(starting_color.s) + i * i16::from(config.saturation_shift)
            );
            let v = clamp_to_primitive_bounds!(
                u8,
                i16::from(starting_color.v) + i * i16::from(config.value_shift)
            );
            color_cell
                .borrow_mut()
                .fill_the_cell(args.undo_redo, HSV { h, s, v })
        }
    }
}
