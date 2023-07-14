use sfml::graphics::RenderWindow;
use tracing::warn;
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    elements::traits::Element,
    events::Event,
    ui_settings::UISettings,
    utils::consts::DUMMY_MOUSE_MOVED_EVENT,
};

use super::settings_menu::SettingsMenu;

pub fn perform_events(
    events: &Vec<Event>,
    window: &mut RenderWindow,
    ui_settings: &mut UISettings,
    settings_menu: &mut SettingsMenu,
    previous_display_state: bool,
) {
    for event in events {
        perform_event(
            event,
            window,
            ui_settings,
            settings_menu,
            previous_display_state,
        );
    }
}

fn perform_event(
    event: &Event,
    window: &mut RenderWindow,
    ui_settings: &mut UISettings,
    settings_menu: &mut SettingsMenu,
    previous_display_state: bool,
) {
    match event.id {
        0 => {}
        1 => event1(
            event,
            window,
            ui_settings,
            settings_menu,
            previous_display_state,
        ),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event1(
    _event: &Event,
    window: &mut RenderWindow,
    ui_settings: &mut UISettings,
    settings_menu: &mut SettingsMenu,
    previous_display_state: bool,
) {
    settings_menu.display = !previous_display_state;
    settings_menu.event_handler(window, ui_settings, DUMMY_MOUSE_MOVED_EVENT);
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
