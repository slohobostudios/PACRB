use std::str::FromStr;

use sfml::graphics::RenderWindow;
use tracing::{error, warn};
use ui::{
    dom_controller::{DomController, DomControllerInterface},
    elements::{traits::Element as ElementTrait, Element},
    events::{Event, Events},
    syncs::Syncs,
    ui_settings::{
        aspect_ratio::{AspectRatio, DefaultAspectRatios},
        UISettings,
    },
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
        3 => event3(settings_menu, ui_settings),
        4 => event4(settings_menu, ui_settings),
        5 => event5(settings_menu, ui_settings),
        100 => event100(event, ui_settings),
        101 => event101(ui_settings),
        102 => event102(event, ui_settings, window),
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

fn event3(settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    set_the_current_set(&mut settings_menu.settings_menu_dom, 0);
    sync_events(&mut settings_menu.settings_menu_dom, ui_settings);
}

fn event4(settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    set_the_current_set(&mut settings_menu.settings_menu_dom, 1);
    sync_events(&mut settings_menu.settings_menu_dom, ui_settings);
}

fn event5(settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    set_the_current_set(&mut settings_menu.settings_menu_dom, 2);
    sync_events(&mut settings_menu.settings_menu_dom, ui_settings);
}

fn event100(event: &Event, ui_settings: &mut UISettings) {
    let Events::StringEvent(event) = event.event.clone() else {
        error!("Event is not a StringEvent {:#?}", event);
        return;
    };

    let Ok(default_aspect_ratio) = DefaultAspectRatios::from_str(&event) else {
        error!("Aspect ratio {:#?} does not exist", event);
        return;
    };

    let mut aspect_ratio = AspectRatio::from(default_aspect_ratio);
    aspect_ratio.current_resolution = ui_settings.aspect_ratio.current_resolution;
    aspect_ratio.compute_resolution();
    ui_settings.aspect_ratio = aspect_ratio;
}

fn event101(ui_settings: &UISettings) {
    ui_settings.save_settings()
}

fn event102(event: &Event, ui_settings: &mut UISettings, window: &mut RenderWindow) {
    let Events::BooleanEvent(enable_vsync) = event.event else {
        error!("event is not a boolean event! {:#?}", event);
        return;
    };

    if enable_vsync {
        ui_settings.enable_vsync(window)
    } else {
        ui_settings.disable_vsync(window)
    }
}

pub fn sync_events(dom_controller: &mut DomController, ui_settings: &UISettings) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {},
            1 => {},
            100 => {
                let Ok(aspect_ratio) = DefaultAspectRatios::try_from(ui_settings.aspect_ratio) else {
                    error!("Failed to convert aspect_ratio");
                    return;
                };

                let Element::ListBox(list_box) = ele else {
                    error!("Failed to convert element into listbox: {:#?}", ele);
                    return;
                };
                list_box.sync(Syncs::String(aspect_ratio.to_string()));
            }
            102 => {
                let Element::Button(boolean_image_button) = ele else {
                    error!("Failed to convert element into boolean image button: {:#?}", ele);
                    return;
                };
                boolean_image_button.sync(Syncs::Boolean(ui_settings.is_vsync_enabled()));
            }
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        })
}

fn set_the_current_set(dom_controller: &mut DomController, set_num: usize) {
    dom_controller.root_node.traverse_dom_mut(&mut |ele| {
        if ele.sync_id() == 1 {
            let Element::Sets(set) = ele else {
                error!("Element is not a set!");
                return;
            };
            set.set_current_set(set_num);
        }
    });
}
