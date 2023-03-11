use rand::{distributions::Uniform, prelude::Distribution};
use sfml::{
    graphics::{Color, FloatRect, RcSprite, RenderTarget, RenderWindow, Transformable, View},
    system::{Vector2, Vector2f},
    window::{Event, Style},
};
use utils::{
    fps_counter::FPSCounter, resource_manager::ResourceManager,
    tracing_subscriber_setup::setup_tracing_subscriber_with_no_logging,
};

pub mod arithmetic_util_functions;
pub mod quads;
pub mod resource_manager;
pub mod sfml_util_functions;
pub mod simple_error;
pub mod string_util_functions;
pub mod tracing_subscriber_setup;

fn every_asset_in_sprite_array(resource_manager: &ResourceManager) -> Vec<RcSprite> {
    let mut sprites = Vec::new();
    for asset_id in resource_manager.asset_keys_iter() {
        let mut sprite = RcSprite::with_texture(resource_manager.fetch_asset(asset_id).texture());
        sprite.set_origin(center_of_rect!(f32, sprite.global_bounds()));
        match asset_id {
            "missing_texture.png" => {
                sprite.set_scale(Vector2f::new(0.1, 0.1));
            }
            _ => sprite.set_scale(Vector2::new(2., 2.)),
        }
        sprites.push(sprite)
    }

    sprites
}

const INIT_WINDOW_SIZE: (u32, u32) = (1280, 720);
fn main() {
    setup_tracing_subscriber_with_no_logging();
    let mut view = View::new(
        center_of_rect!(
            f32,
            FloatRect::new(0., 0., INIT_WINDOW_SIZE.0 as f32, INIT_WINDOW_SIZE.1 as f32)
        ),
        Vector2::new(INIT_WINDOW_SIZE.0 as f32, INIT_WINDOW_SIZE.1 as f32),
    );
    let mut window = RenderWindow::new(
        INIT_WINDOW_SIZE,
        "UTILS_TEST",
        Style::DEFAULT,
        &Default::default(),
    );
    let resource_manager = ResourceManager::new();
    let mut fps_counter = FPSCounter::new(&resource_manager, 240);
    let mut sprites = every_asset_in_sprite_array(&resource_manager);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    view.set_size(Vector2f::new(width as f32, height as f32));
                    let view_size = view.size();
                    view.set_center(center_of_rect!(
                        f32,
                        FloatRect::new(0., 0., view_size.x, view_size.y)
                    ));
                    let between = Vector2::new(
                        Uniform::from(0f32..view_size.x),
                        Uniform::from(0f32..view_size.y),
                    );
                    let mut rng = rand::thread_rng();
                    for sprite in &mut sprites {
                        sprite.set_position(Vector2::new(
                            between.x.sample(&mut rng),
                            between.y.sample(&mut rng),
                        ));
                    }
                }
                _ => {}
            }
        }
        fps_counter.new_frame();

        window.clear(Color::BLACK);
        window.set_view(&view);
        for sprite in &sprites {
            window.draw(sprite);
        }
        window.draw(fps_counter.fps_text());
        window.display();
    }
}
