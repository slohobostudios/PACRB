use sfml::graphics::RenderWindow;
use tracing::warn;
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    elements::traits::Element,
    events::Event,
    ui_settings::UISettings,
    utils::consts::DUMMY_MOUSE_MOVED_EVENT,
};

use super::SettingsMenu;

pub fn perform_events(
    events: &Vec<Event>,
    window: &mut RenderWindow,
    ui_settings: &mut UISettings,
    settings_menu: &mut SettingsMenu,
) {
    for event in events {
        perform_event(event, window, ui_settings, settings_menu, events.len());
    }
}

fn perform_event(
    event: &Event,
    window: &mut RenderWindow,
    ui_settings: &mut UISettings,
    settings_menu: &mut SettingsMenu,
    num_of_events: usize,
) {
    match event.id {
        0 => {}
        1 => event1(settings_menu, num_of_events),
        2 => event2(window, ui_settings, settings_menu),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event1(settings_menu: &mut SettingsMenu, num_of_events: usize) {
    // If we click on the background, it'll throw an event. Div will also throw an event.
    // Meaning we have 2 events. 2 events, means we clicked on the background.
    if num_of_events > 1 {
        return;
    }
    settings_menu.display = false;
}

fn event2(
    window: &mut RenderWindow,
    ui_settings: &mut UISettings,
    settings_menu: &mut SettingsMenu,
) {
    settings_menu.display = false;
    settings_menu
        .settings_menu_dom
        .event_handler(window, ui_settings, DUMMY_MOUSE_MOVED_EVENT);
}

pub fn sync_events(dom_controller: &mut DomController) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        })
}
