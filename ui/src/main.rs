use ::utils::{
    fps_counter::FPSCounter, resource_manager::ResourceManager,
    tracing_subscriber_setup::setup_tracing_subscriber_with_no_logging,
};
use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Style},
};
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    elements::{traits::Element as ElementTrait, Element},
    ui_settings::UISettings,
};

pub mod dom_controller;
pub mod dom_loader;
pub mod elements;
pub mod events;
pub mod syncs;
pub mod ui_settings;
pub mod utils;

const XML_DOC: &str = r##"<RootNode scale="2" font_size="16" color="#f7e5e4" xmlns="https://www.loc.gov/marc/marcxml.html">
  <Background
    type="Repeatable3x3Background"
    asset="dark_blue_background.png"
    position="b:7,r:7"
    size="x:200,y:175"
    frame_id="0">
    <Grid
      size="x:190,y:115"
      pagination_size="x:2,y:2"
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
        <Div padding="t:5,b:5,l:5,r:5">
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
    position="r:12,t:12"
    >
    <Div padding="t:5,b:5,l:5,r:5">
      <Text disable_padding="true">
        Test
      </Text>
    </Div>
  </Button>
  <Button
    type="TilingButton"
    asset="3x3_tilable_standalone_button.png"
    frame_id="0"
    hover_frame_id="1"
    click_frame_id="2"
    position="r:12,t:52"
    >
    <Div padding="t:5,b:5,l:5,r:5">
      <Image type="Icon" name="gear"/>
    </Div>
  </Button>
  <Background
    type="Repeatable3x3Background"
    asset="dark_blue_background.png"
    frame_id="0"
    padding="t:25,b:25,l:25,r:25">
  <TextBox
    type="FixedSizeOneLineTextbox"
    size="x:200,y:0"
    color="#081a1b"
    >
  </TextBox>
  </Background>
  <Background
    type="Repeatable3x3Background"
    asset="dark_blue_background.png"
    frame_id="0"
    position="b:12,l:12"
    padding="t:5,b:5,l:5,r:5">
    <Sets size="x:150,y:40" sync_id="1">
      <Button
        type="TilingButton"
        asset="3x3_tilable_button_on_background.png"
        frame_id="0"
        hover_frame_id="1"
        click_frame_id="2"
        position="l:10"
        event_id="1">
        <Div padding="t:5,b:5,l:5,r:5">
          <Text>
            Go Right
          </Text>
        </Div>
      </Button>
      <Button
        type="TilingButton"
        asset="3x3_tilable_button_on_background.png"
        frame_id="0"
        hover_frame_id="1"
        click_frame_id="2"
        position="r:10"
        event_id="2">
        <Div padding="t:5,b:5,l:5,r:5">
          <Text>
            Go Left
          </Text>
        </Div>
      </Button>
    </Sets>
  </Background>
</RootNode>"##;

fn main() {
    setup_tracing_subscriber_with_no_logging();

    const WINDOW_SIZE: (u32, u32) = (1280, 720);
    let mut window = RenderWindow::new(WINDOW_SIZE, "ui_test", Style::DEFAULT, &Default::default());
    window.set_vertical_sync_enabled(true);
    let mut ui_settings = UISettings::from_file();
    let resource_manager = ResourceManager::new();
    let mut dom = DomController::new(&resource_manager, &ui_settings, XML_DOC);
    let mut fps_counter = FPSCounter::new(&resource_manager, 240);
    dom.event_handler(
        &mut window,
        &mut ui_settings,
        Event::Resized {
            width: WINDOW_SIZE.0,
            height: WINDOW_SIZE.1,
        },
    );

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            #[allow(clippy::single_match)]
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
            ui_settings.event_handler(event);
            let events = dom.event_handler(&mut window, &mut ui_settings, event);

            for event in events {
                match event.id {
                    1 => dom.root_node.traverse_dom_mut(&mut |ele| {
                        if ele.sync_id() == 1 {
                            if let Element::Sets(sets) = ele {
                                sets.set_current_set(1);
                            }
                        }
                    }),
                    2 => dom.root_node.traverse_dom_mut(&mut |ele| {
                        if ele.sync_id() == 1 {
                            if let Element::Sets(sets) = ele {
                                sets.set_current_set(0);
                            }
                        }
                    }),
                    _ => {}
                }
            }
        }
        dom.update(&resource_manager);
        fps_counter.new_frame();

        window.clear(Color::BLACK);
        dom.render(&mut window);
        window.draw(fps_counter.fps_text());
        window.display();
    }
}
