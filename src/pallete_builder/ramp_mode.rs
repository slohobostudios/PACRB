use sfml::{
    system::Vector2,
    window::{mouse::Button, Event},
};

use self::color_ramper::ColorRamper;

use super::{
    color_grid::{color_cell::RcColorCell, undo_redo::UndoRedoCell, ColorGrid, GRID_SIZE},
    hover_handler::HoverHandler,
    hsv_color::HSV,
    ui_components::{
        config_selector::ConfigSelector,
        confirm_color_ramp::{ConfirmColorRamp, Orientation},
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
}

impl RampMode {
    pub fn event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        if !self.ramp.ramp_being_shown() {
            self.no_ramp_event_handler(args);
        } else {
            self.ramp_event_handler(args);
        }
    }

    fn no_ramp_event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        self.hover_handler
            .event_handler(args.event, &mut args.color_grid);

        if args.erase_mode.erase_mode_enabled() {
            return;
        }

        match args.event {
            Event::MouseButtonPressed { button, x, y } if button == Button::Left => {
                self.ramp.create_ramp(Vector2::new(x, y), args);
                args.confirm_color_ramp.set_enable(true);
            }
            _ => {}
        }
    }

    fn ramp_event_handler(&mut self, args: &mut RampModeEventHandlerArguments) {
        self.hover_handler.unhover_all_cells();
    }
}

impl Drop for RampMode {
    fn drop(&mut self) {
        self.hover_handler.unhover_all_cells();
    }
}
