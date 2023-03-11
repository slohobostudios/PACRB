use ::utils::{
    resource_manager::ResourceManager,
    tracing_subscriber_setup::setup_tracing_subscriber_with_no_logging,
};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Style},
};
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    ui_settings::UISettings,
};

pub mod dom_controller;
pub mod dom_loader;
pub mod elements;
pub mod events;
pub mod ui_settings;
pub mod utils;

const XML_DOC: &str = r##"<RootNode scale="4" font_size="24" color="#f7e5e4" xmlns="https://www.loc.gov/marc/marcxml.html">
  <Background
    type="Fixed3x3RepeatableBackground"
    asset="dark_blue_background.png"
    position="b:15,r:15"
    size="x:400,y:350"
    frame_id="0">
    <Grid
      size="x:380,y:330"
      pagination_size="x:4,y:4"
      >
      <Button
        type="BooleanImageButton"
        asset="check_box_button.png"
        truth_frame_id="0"
        truth_hover_frame_id="1"
        truth_click_frame_id="2"
        false_frame_id="3"
        false_hover_frame_id="4"
        false_click_frame_id="5"
        />
      <Button
        type="ImageButton"
        asset="x_button.png"
        frame_id="0"
        hover_frame_id="1"
        click_frame_id="2"
        />
      <Button
        type="TilingButton"
        asset="3x3_tilable_button_on_background.png"
        frame_id="0"
        hover_frame_id="1"
        click_frame_id="2"
        >
        <Div padding="t:10,b:10,l:10,r:10">
          <Text disable_padding="true">
            Test
          </Text>
        </Div>
      </Button>
    </Grid>
  </Background>
  <Button
    type="TilingButton"
    asset="3x3_tilable_standalone_button.png"
    frame_id="0"
    hover_frame_id="1"
    click_frame_id="2"
    position="r:25,t:25"
    >
    <Div padding="t:20,b:20,l:20,r:20">
      <Text disable_padding="true">
        Test
      </Text>
    </Div>
  </Button>
</RootNode>"##;

fn main() {
    setup_tracing_subscriber_with_no_logging();

    const WINDOW_SIZE: (u32, u32) = (1280, 720);
    let mut window = RenderWindow::new(WINDOW_SIZE, "ui_test", Style::DEFAULT, &Default::default());
    window.set_vertical_sync_enabled(true);
    let mut ui_settings = UISettings::from_file();
    let resource_manager = ResourceManager::new();
    let mut dom = DomController::new(&resource_manager, &ui_settings, XML_DOC);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
            ui_settings.event_handler(event);
            dom.event_handler(&mut window, &mut ui_settings, event);
        }
        dom.update(&resource_manager);

        window.clear(Color::BLACK);
        dom.render(&mut window);
        window.display();
    }
}
