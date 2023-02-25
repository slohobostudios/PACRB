use std::env;

use crate::{
    assets::resource_manager::ResourceManager, pallete_builder::PalleteBuilder,
    utils::fps_counter::FPSCounter,
};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::{Vector2f, Vector2i},
    window::{Event, Style},
};
use ui::ui_settings::UISettings;

pub mod assets;
mod pallete_builder;

mod tracing_subscriber_setup;
pub mod ui;
pub mod utils;

fn ui_settings_event_handler(ui_settings: &mut UISettings, event: Event) {
    use crate::ui::ui_settings::controls::possible_inputs::PossibleInputs;
    match event {
        Event::MouseButtonPressed { button, x, y } => {
            ui_settings
                .binds
                .input_pressed(PossibleInputs::from(button));
            ui_settings.cursor_position = ui_settings
                .aspect_ratio
                .relative_mouse_coords(Vector2i::new(x, y));
        }
        Event::MouseButtonReleased { button, x, y } => {
            ui_settings
                .binds
                .input_released(PossibleInputs::from(button));
            ui_settings.cursor_position = ui_settings
                .aspect_ratio
                .relative_mouse_coords(Vector2i::new(x, y));
        }
        Event::MouseMoved { x, y } => {
            ui_settings.cursor_position = ui_settings
                .aspect_ratio
                .relative_mouse_coords(Vector2i::new(x, y));
        }
        Event::KeyPressed {
            code,
            alt,
            ctrl,
            shift,
            system,
        } => {
            ui_settings
                .binds
                .ctrl_alt_shift_system_is_pressed(ctrl, alt, shift, system);
            ui_settings.binds.input_pressed(PossibleInputs::from(code));
        }
        Event::KeyReleased {
            code,
            alt,
            ctrl,
            shift,
            system,
        } => {
            ui_settings
                .binds
                .ctrl_alt_shift_system_is_pressed(ctrl, alt, shift, system);
            ui_settings.binds.input_pressed(PossibleInputs::from(code));
        }
        _ => {}
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if tracing_subscriber_setup::setup_tracing_subscriber(&args).is_err() {
        eprintln!("Unable to setup logging subscriber! No log file will be generated.")
    }

    const WINDOW_SIZE: (u32, u32) = (1280, 720);
    // Create a new window
    let mut window = RenderWindow::new(WINDOW_SIZE, "PACRB", Style::DEFAULT, &Default::default());
    window.set_vertical_sync_enabled(false);
    let mut ui_settings = UISettings::from_file();
    ui_settings.save_settings();
    let resource_manager = ResourceManager::new();
    let mut fps_counter = FPSCounter::new(&resource_manager, 60);
    let mut pallete_builder = PalleteBuilder::new(&resource_manager, &ui_settings);
    while window.is_open() {
        while let Some(event) = window.poll_event() {
            ui_settings_event_handler(&mut ui_settings, event);
            match event {
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    ui_settings.aspect_ratio.current_resolution =
                        Vector2f::new(width as f32, height as f32);
                    ui_settings.aspect_ratio.compute_resolution();
                }
                _ => {}
            }
            pallete_builder.event_handler(&mut window, &mut ui_settings, event);
        }
        fps_counter.new_frame();
        pallete_builder.update(&resource_manager);
        window.clear(Color::rgb(35, 38, 39));
        pallete_builder.render(&mut window);
        window.draw(fps_counter.fps_text());
        window.display();
    }
}
