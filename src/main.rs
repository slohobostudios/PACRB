#![windows_subsystem = "windows"]
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    system::Vector2f,
    window::{Event, Style},
};
use ui::ui_settings::UISettings;
use utils::{
    fps_counter::FPSCounter, resource_manager::ResourceManager,
    tracing_subscriber_setup::setup_tracing_subscriber,
};

use crate::pallete_builder::PalleteBuilder;

mod pallete_builder;

fn main() {
    setup_tracing_subscriber();

    const WINDOW_SIZE: (u32, u32) = (1280, 720);
    // Create a new window
    let mut window = RenderWindow::new(WINDOW_SIZE, "PACRB", Style::DEFAULT, &Default::default());
    let mut ui_settings = UISettings::from_file();
    ui_settings.synchronize_ui_settings_and_sfml(&mut window);
    // This prevents ui elements from creating render textures that are of size 0x0
    ui_settings.event_handler(Event::Resized {
        width: WINDOW_SIZE.0,
        height: WINDOW_SIZE.1,
    });
    ui_settings.save_settings();
    let resource_manager = ResourceManager::new();
    let mut fps_counter = FPSCounter::new(&resource_manager, 60);
    let mut pallete_builder = PalleteBuilder::new(&resource_manager, &ui_settings);
    while window.is_open() {
        for event in ui_settings.normalize_events(&mut window) {
            ui_settings.event_handler(event);
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
        // window.draw(fps_counter.fps_text());
        window.display();
    }
}
