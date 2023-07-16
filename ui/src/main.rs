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

const XML_DOC: &str = r##"<RootNode scale="4" font_size="20" color="#f7e5e4" xmlns="https://www.loc.gov/marc/marcxml.html">
    <Background
        type="Repeatable3x3Background"
        asset="dark_blue_background.png"
        frame_id="0"
        size="x:800, y:500">
        <Button
            type="ImageButton"
            asset="x_button.png"
            position='t:5,r:5'
            frame_id='0'
            hover_frame_id='1'
            click_frame_id='2'
            scale='2'
            event_id='1'/>
        <Grid
            position="l:-4"
            size="x:100,y:400"
            pagination_size="x:1,y:2"
            grid_layout="x:1,y:2">
            <Button
                type="PrimitiveFillButton"
                color="#242336"
                hover_color="#51507a"
                click_color="#8482c1"
                event_id='2'>
                <Text>
                    Example 1
                </Text>
            </Button>
            <Button
                type="PrimitiveFillButton"
                color="#242336"
                hover_color="#51507a"
                click_color="#8482c1"
                event_id='3'>
                <Text>
                    Example 2
                </Text>
            </Button>
        </Grid>
        <Primitive 
            type="TriangleFan"
            position="l:96"
            vertices="(x:0,y:0),(x:4,y:0),(x:4,y:400),(x:0,y:400)"
            color="#f7e5e4"/>
        <Sets position="l:56,r:43" size="x:702,y:400" sync_id='1'>
            <Div size="x:702,y:400">
                <Button 
                    scale='2'
                    type="BooleanImageButton"
                    asset="check_box_button.png"
                    truth_frame_id='0'
                    truth_hover_frame_id='1'
                    truth_click_frame_id='2'
                    false_frame_id='3'
                    false_hover_frame_id='4'
                    false_click_frame_id='5'
                    position="b:0,r:10"/>
                <Button
                    type="TilingButton"
                    asset="3x3_tilable_button_on_background.png"
                    position="r:10,b:40"
                    size="x:22,y:22"
                    frame_id='0'
                    hover_frame_id='1'
                    click_frame_id='2'/>
                <TextBox
                    type="FixedSizeOneLineTextbox"
                    color="#091d1e"
                    size="x:80"/>
            </Div>
            <Div size="x:702,y:400">
                <Slider 
                    type="IncrementPointerSlider"
                    asset="slider.png"
                    position='b:0'
                    frame_id='0'
                    hover_frame_id='1'
                    click_frame_id='2'
                    min='-2147483647'
                    max='2147483647'
                    increment='10000000'
                    scale='2'
                    size='x:100'/>
                <ListBox 
                    type="UpDownScrollListbox"
                    asset="scroll_up_down_listbox.png"
                    frame_id='0'
                    hover_frame_id='1'
                    click_frame_id='2'
                    position='t:0'
                    number_of_buttons='5'
                    padding="t:5,b:5,l:10,r:10"
                    options="option 1,option 2,option 3,option 4,option 5,option 6,option 7,option 8,option 9,option 10"/>
            </Div>
        </Sets>
    </Background>
    <Button
        type="TilingButton"
        asset="3x3_tilable_standalone_button.png"
        position="r:5,b:5"
        frame_id="0"
        hover_frame_id="1"
        click_frame_id="2">
        <Div padding="t:5,b:5,l:5,r:5">
            <Text>
                Button
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
    let mut fps_counter = FPSCounter::new(&resource_manager, 240);

    while window.is_open() {
        for event in ui_settings.normalize_events(&mut window) {
            #[allow(clippy::single_match)]
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
            ui_settings.event_handler(event);
            let events = dom.event_handler(&mut window, &mut ui_settings, event);

            for event in events {
                match event.id {
                    1 => window.close(),
                    2 => dom.root_node.traverse_dom_mut(&mut |ele| {
                        if ele.sync_id() == 1 {
                            let Element::Sets(sets) = ele else {
                                panic!("This is supposed to be a set");
                            };
                            sets.set_current_set(0);
                        }
                    }),
                    3 => dom.root_node.traverse_dom_mut(&mut |ele| {
                        if ele.sync_id() == 1 {
                            let Element::Sets(sets) = ele else {
                                panic!("This is supposed to be a set");
                            };
                            sets.set_current_set(1);
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
