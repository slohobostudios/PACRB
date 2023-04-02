use sfml::{
    graphics::{RenderTarget, RenderWindow, View},
    system::{Vector2, Vector2f, Vector2i},
    window::{
        mouse::{Button, Wheel},
        Event, Key,
    },
    SfBox,
};
use ui::{dom_controller::DomControllerInterface, ui_settings::UISettings};
use utils::resource_manager::ResourceManager;

use self::{
    color_grid::{color_cell::CELL_SIZE, undo_redo::UndoRedoCell, ColorGrid},
    normal_mode::{NormalMode, NormalModeEventHandlerArguments},
    ramp_mode::{RampMode, RampModeEventHandlerArguments},
    ui_components::{
        config_selector::ConfigSelector, confirm_color_ramp::ConfirmColorRamp,
        erase_mode::EraseMode, hsv_selector::HSVSelector,
    },
};

mod color_grid;
mod hover_handler;
pub mod hsv_color;
mod normal_mode;
mod ramp_mode;
mod ui_components;

enum Mode {
    NormalMode(NormalMode),
    RampMode(RampMode),
}

const VIEW_MIN_SIZE: Vector2f = Vector2f::new(100f32, 100f32);
const VIEW_MAX_SIZE: Vector2f = Vector2f::new(1920f32, 1080f32);
pub struct PalleteBuilder {
    current_mode: Mode,
    config_selector: ConfigSelector,
    hsv_selector: HSVSelector,
    erase_mode: EraseMode,
    confirm_color_ramp: ConfirmColorRamp,
    color_grid: ColorGrid,
    view: SfBox<View>,
    is_dragging_erase: bool,
    is_dragging_screen: bool,
    previous_mouse_position: Vector2i,
    undo_redo: UndoRedoCell,
}

impl PalleteBuilder {
    pub fn new(resource_manager: &ResourceManager, ui_settings: &UISettings) -> Self {
        let color_grid = ColorGrid::new();
        Self {
            current_mode: Mode::NormalMode(Default::default()),
            hsv_selector: HSVSelector::new(resource_manager, ui_settings),
            config_selector: ConfigSelector::new(resource_manager, ui_settings),
            confirm_color_ramp: ConfirmColorRamp::new(resource_manager, ui_settings),
            erase_mode: EraseMode::new(resource_manager, ui_settings),
            is_dragging_erase: false,
            is_dragging_screen: false,
            previous_mouse_position: Default::default(),
            view: View::new(
                Vector2f::new(
                    (color_grid[0].len() / 2 * usize::try_from(CELL_SIZE.x).unwrap()) as f32,
                    (color_grid[0].len() / 2 * usize::try_from(CELL_SIZE.y).unwrap()) as f32,
                ) / 2f32,
                ui_settings.aspect_ratio.current_resolution,
            ),
            color_grid,
            undo_redo: Default::default(),
        }
    }

    pub fn dom_controller_interfaces_iter_mut(&mut self) -> [&mut dyn DomControllerInterface; 4] {
        [
            &mut self.config_selector,
            &mut self.hsv_selector,
            &mut self.erase_mode,
            &mut self.confirm_color_ramp,
        ]
    }

    pub fn event_handler(
        &mut self,
        window: &mut RenderWindow,
        ui_settings: &mut UISettings,
        event: Event,
    ) {
        let mut events = Vec::new();
        for dci in self.dom_controller_interfaces_iter_mut() {
            events.append(&mut dci.event_handler(window, ui_settings, event))
        }

        match self.current_mode {
            Mode::NormalMode(_) if self.config_selector.current_config().auto_ramping => {
                self.current_mode = Mode::RampMode(RampMode::default())
            }
            Mode::RampMode(_) if !self.config_selector.current_config().auto_ramping => {
                self.confirm_color_ramp.set_enable(false);
                if let Mode::RampMode(ramp_mode) = &mut self.current_mode {
                    ramp_mode.clear_the_ramp(&mut self.undo_redo);
                }
                self.current_mode = Mode::NormalMode(NormalMode::default())
            }
            _ => {}
        }

        if events.is_empty() {
            return;
        }

        self.view_event_handler(&event);
        let event = self.correct_mouse_pos_event(event, window);
        self.erase_event_handler(&event);
        self.drag_screen_event_handler(&event);
        self.undo_redo_event_handler(&event);

        match &mut self.current_mode {
            Mode::NormalMode(normal_mode) => {
                normal_mode.event_handler(&mut NormalModeEventHandlerArguments::new(
                    &mut self.color_grid,
                    event,
                    &self.hsv_selector,
                    &self.erase_mode,
                    &mut self.undo_redo,
                ))
            }
            Mode::RampMode(ramp_mode) => {
                ramp_mode.event_handler(&mut RampModeEventHandlerArguments::new(
                    &mut self.color_grid,
                    event,
                    &self.hsv_selector,
                    &self.erase_mode,
                    &mut self.undo_redo,
                    &mut self.confirm_color_ramp,
                    &self.config_selector,
                ))
            }
        }
    }

    pub fn update(&mut self, resource_manager: &ResourceManager) {
        for dci in self.dom_controller_interfaces_iter_mut() {
            dci.update(resource_manager);
        }

        if let Mode::RampMode(ramp_mode) = &mut self.current_mode {
            ramp_mode.update(&mut RampModeEventHandlerArguments::new(
                &mut self.color_grid,
                Event::Closed,
                &self.hsv_selector,
                &self.erase_mode,
                &mut self.undo_redo,
                &mut self.confirm_color_ramp,
                &self.config_selector,
            ))
        }

        self.color_grid.update();
    }

    pub fn render(&mut self, window: &mut RenderWindow) {
        window.set_view(&self.view);
        self.color_grid.render(window);

        for dci in self.dom_controller_interfaces_iter_mut() {
            dci.render(window);
        }
    }
}

/********* EVENT HANDLING CODE ***********/
impl PalleteBuilder {
    fn correct_mouse_pos_event(&mut self, mut event: Event, window: &RenderWindow) -> Event {
        fn mutate_mouse_pos(x: &mut i32, y: &mut i32, view: &SfBox<View>, window: &RenderWindow) {
            let tx = view.size().x / (window.size().x as f32) * (*x as f32);
            let ty = view.size().y / (window.size().y as f32) * (*y as f32);
            *x = (tx + view.center().x - view.size().x / 2f32) as i32;
            *y = (ty + view.center().y - view.size().y / 2f32) as i32;
        }
        match event {
            Event::MouseButtonPressed {
                button: _,
                ref mut x,
                ref mut y,
            } => mutate_mouse_pos(x, y, &self.view, window),
            Event::MouseButtonReleased {
                button: _,
                ref mut x,
                ref mut y,
            } => mutate_mouse_pos(x, y, &self.view, window),
            Event::MouseMoved {
                ref mut x,
                ref mut y,
            } => mutate_mouse_pos(x, y, &self.view, window),
            Event::MouseWheelScrolled {
                wheel: _,
                delta,
                ref mut x,
                ref mut y,
            } if self.view.size().x > VIEW_MIN_SIZE.x
                && self.view.size().y > VIEW_MIN_SIZE.y
                && delta.is_sign_positive() =>
            {
                mutate_mouse_pos(x, y, &self.view, window)
            }
            _ => {}
        };

        event
    }

    fn view_event_handler(&mut self, event: &Event) {
        match event {
            &Event::Resized { width, height } => {
                self.view.set_size(Vector2::new(width, height).as_other())
            }
            Event::MouseWheelScrolled {
                wheel,
                delta,
                x: _,
                y: _,
            } if *wheel == Wheel::VerticalWheel
                && delta.is_sign_negative()
                && self.view.size().x < VIEW_MAX_SIZE.x
                && self.view.size().y < VIEW_MAX_SIZE.y =>
            {
                self.view.zoom(1.1);
            }
            Event::MouseWheelScrolled {
                wheel,
                delta,
                x: _,
                y: _,
            } if *wheel == Wheel::VerticalWheel
                && delta.is_sign_positive()
                && self.view.size().x > VIEW_MIN_SIZE.x
                && self.view.size().y > VIEW_MIN_SIZE.y =>
            {
                self.view.zoom(0.9);
            }
            _ => {}
        }
    }

    fn erase_event_handler(&mut self, event: &Event) {
        match event {
            // Erase color
            &Event::MouseButtonPressed { button, x, y }
                if self.erase_mode.erase_mode_enabled() && button == Button::Left =>
            {
                if let Some(color_cell) = self.color_grid.coord_to_cell_mut(Vector2::new(x, y)) {
                    color_cell.borrow_mut().empty_the_cell(&mut self.undo_redo);
                }
                self.is_dragging_erase = true
            }

            // Make sure it is still dragging
            Event::MouseMoved { x: _, y: _ }
                if self.erase_mode.erase_mode_enabled() && !Button::Left.is_pressed() =>
            {
                self.is_dragging_erase = false;
            }

            // Dragging erase
            &Event::MouseMoved { x, y }
                if self.erase_mode.erase_mode_enabled() && self.is_dragging_erase =>
            {
                if let Some(color_cell) = self.color_grid.coord_to_cell_mut(Vector2::new(x, y)) {
                    color_cell.borrow_mut().empty_the_cell(&mut self.undo_redo);
                }
            }

            // Finish dragging erase
            &Event::MouseButtonReleased { button: _, x, y }
                if self.erase_mode.erase_mode_enabled() && self.is_dragging_erase =>
            {
                self.is_dragging_erase = false;
                if let Some(color_cell) = self.color_grid.coord_to_cell_mut(Vector2::new(x, y)) {
                    color_cell.borrow_mut().empty_the_cell(&mut self.undo_redo);
                }
            }

            _ => {}
        }
    }

    fn drag_screen_event_handler(&mut self, event: &Event) {
        match *event {
            // Begin dragging the screen around
            Event::MouseButtonPressed { button, x, y }
                if (button == Button::Right || button == Button::Middle) =>
            {
                self.is_dragging_screen = true;
                self.previous_mouse_position = Vector2::new(x, y);
            }

            // Make sure the middle or right button is still pressed. If not, stop dragging
            Event::MouseMoved { x: _, y: _ }
                if self.is_dragging_screen
                    && !(Button::Middle.is_pressed() || Button::Right.is_pressed()) =>
            {
                self.is_dragging_screen = false;
            }
            // Actually drag the screen around
            Event::MouseMoved { x, y } if self.is_dragging_screen => {
                let mouse_diff = Vector2::new(x, y) - self.previous_mouse_position;
                self.previous_mouse_position = Vector2::new(x, y) - mouse_diff;
                let center = self.view.center();
                self.view.set_center(center - mouse_diff.as_other());
            }

            // Finished dragging the screen around
            Event::MouseButtonReleased { button, x: _, y: _ }
                if self.is_dragging_screen
                    && (button == Button::Right || button == Button::Middle) =>
            {
                self.is_dragging_screen = false
            }
            _ => {}
        }
    }

    fn undo_redo_event_handler(&mut self, event: &Event) {
        match *event {
            // Undo
            Event::KeyPressed {
                code,
                alt: _,
                ctrl,
                shift: _,
                system: _,
            } if code == Key::Z && ctrl => {
                self.undo_redo.undo(&mut self.color_grid);
            }

            // Redo
            Event::KeyPressed {
                code,
                alt: _,
                ctrl,
                shift: _,
                system: _,
            } if code == Key::R && ctrl => {
                self.undo_redo.redo(&mut self.color_grid);
            }
            _ => {}
        }
    }
}
