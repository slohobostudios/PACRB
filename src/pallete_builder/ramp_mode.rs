use sfml::{
    system::Vector2,
    window::{mouse::Button, Event},
};

use self::color_ramper::ColorRamper;

use super::{
    color_grid::{color_cell::RcColorCell, undo_redo::UndoRedoCell, ColorGrid},
    hover_handler::HoverHandler,
    hsv_color::Hsv,
    ui_components::{
        config_selector::{Config, ConfigSelector},
        confirm_color_ramp::ConfirmColorRamp,
        erase_mode::EraseMode,
        hsv_selector::HSVSelector,
    },
};

mod color_ramper;

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
    confirm_color_ramp: &'confirm_color_ramp mut ConfirmColorRamp,
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
        confirm_color_ramp: &'confirm_color_ramp mut ConfirmColorRamp,
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
    ramp: ColorRamper,
    previous_config: Config,
    previous_color: Hsv,
    middle_cell: Option<RcColorCell>,
    original_middle_cell_color: Option<Hsv>,
}

impl RampMode {
    pub fn regenerate_ramp_new_config(&mut self, args: &mut RampModeEventHandlerArguments) {
        if self.previous_config != args.config_selector.current_config()
            || self.previous_color != args.hsv_selector.curr_color()
        {
            self.ramp.create_ramp(self.ramp.ramp_start_coord(), args);
        }

        self.previous_color = args.hsv_selector.curr_color();
        self.previous_config = args.config_selector.current_config();
    }

    pub fn event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        if !self.ramp.ramp_being_shown() {
            self.no_ramp_event_handler(args);
        } else {
            self.ramp_event_handler(args);
        }
    }

    fn no_ramp_event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        self.hover_handler
            .event_handler(args.event, args.color_grid);

        if args.erase_mode.erase_mode_enabled() {
            return;
        }

        match args.event {
            Event::MouseButtonPressed { button, x, y } if button == Button::Left => {
                let coord = Vector2::new(x, y);
                if let Some(starting_idx) = args.color_grid.coord_to_idx(coord) {
                    if args.color_grid[starting_idx.x][starting_idx.y]
                        .borrow()
                        .draw_full_cell()
                    {
                        self.middle_cell =
                            Some(args.color_grid[starting_idx.x][starting_idx.y].clone());
                        self.original_middle_cell_color = Some(
                            self.middle_cell
                                .clone()
                                .unwrap()
                                .borrow()
                                .full_cell_current_color(),
                        );
                    }
                }
                self.ramp.create_ramp(coord, args);
                self.previous_config = args.config_selector.current_config();
                self.previous_color = args.hsv_selector.curr_color();
                args.confirm_color_ramp.set_enable(true);
            }
            _ => {}
        }
    }

    fn ramp_event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        if !args.confirm_color_ramp.is_enabled() {
            self.ramp = Default::default();
            return;
        }

        self.hover_handler.unhover_all_cells();
    }

    pub fn update(&mut self, args: &mut RampModeEventHandlerArguments) {
        if !self.ramp.ramp_being_shown() {
        } else {
            self.ramp_update(args);
        }
    }

    pub fn ramp_update(&mut self, args: &mut RampModeEventHandlerArguments) {
        if self.ramp.current_orientation() != args.confirm_color_ramp.orientation() {
            self.ramp.change_orientation(args)
        }
    }

    pub fn clear_the_ramp(&mut self, undo_redo: &mut UndoRedoCell) {
        self.ramp.clear_ramp(undo_redo);
        if let (Some(middle_cell), Some(color)) =
            (&mut self.middle_cell, self.original_middle_cell_color)
        {
            middle_cell.borrow_mut().fill_the_cell(undo_redo, color);
        }
    }
}

impl Drop for RampMode {
    fn drop(&mut self) {
        self.hover_handler.unhover_all_cells();
    }
}
