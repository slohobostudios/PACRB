use std::str::FromStr;

use sfml::{graphics::RenderWindow, window::clipboard};
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

use crate::pallete_builder::color_grid::load_save::{
    full_file_path, list_of_files_with_pacrb_extension,
};

use super::{SettingsMenu, TriggerFileStates};

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
        6 => event6(settings_menu, ui_settings),
        100 => event100(event, ui_settings),
        101 => event101(ui_settings),
        102 => event102(event, ui_settings, window),
        // Export functions
        200 => event200(event, settings_menu, ui_settings),
        201 => event201(event, settings_menu, ui_settings),
        202 => event202(settings_menu),
        // Open file directory
        300 => event300(),
        // Copy file directory
        301 => event301(),
        // Refresh
        1097 => event1097(settings_menu),
        // Next
        1098 => event1098(settings_menu),
        // Prev
        1099 => event1099(settings_menu),
        // Load file buttons
        1100 => event1100(settings_menu),
        1101 => event1101(settings_menu),
        1102 => event1102(settings_menu),
        1103 => event1103(settings_menu),
        1104 => event1104(settings_menu),
        // Delete file buttons
        1200 => event1200(settings_menu),
        1201 => event1201(settings_menu),
        1202 => event1202(settings_menu),
        1203 => event1203(settings_menu),
        1204 => event1204(settings_menu),
        // Save file events
        2000 => event2000(event, settings_menu),
        2001 => event2001(settings_menu),
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
    sync_events(settings_menu, ui_settings);
}

fn event4(settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    set_the_current_set(&mut settings_menu.settings_menu_dom, 1);
    sync_events(settings_menu, ui_settings);
    reload_list_of_files(settings_menu);
}

fn event5(settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    set_the_current_set(&mut settings_menu.settings_menu_dom, 2);
    sync_events(settings_menu, ui_settings);
}

fn event6(settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    set_the_current_set(&mut settings_menu.settings_menu_dom, 3);
    sync_events(settings_menu, ui_settings);
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

fn event200(event: &Event, settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    let Events::StringEvent(extension) = &event.event else {
        error!("event is not a string event! {:#?}", event);
        return;
    };

    settings_menu.export_file_extension = extension.clone();
    sync_events(settings_menu, ui_settings);
}

fn event201(event: &Event, settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    let Events::TextBoxEvent(event) = &event.event else {
        error!("event is not a textbox event! {:#?}", event);
        return;
    };

    settings_menu.export_file_name = event.string.clone();
    sync_events(settings_menu, ui_settings);
}

fn event202(settings_menu: &mut SettingsMenu) {
    settings_menu.trigger_export_event = TriggerFileStates::Save;
}

fn event300() {
    let full_file_path = full_file_path();
    let Ok(full_file_path) = full_file_path else {
        error!("{:#?}", full_file_path);
        return;
    };

    if let Err(err) = open::that(full_file_path) {
        error!("{:#?}", err);
    }
}

fn event301() {
    let full_file_path = full_file_path();
    let Ok(full_file_path) = full_file_path else {
        error!("{:#?}", full_file_path);
        return;
    };

    let full_file_path = full_file_path.into_os_string().into_string();
    let Ok(full_file_path) = full_file_path else {
        error!("{:#?}", full_file_path);
        return;
    };

    clipboard::set_string(&full_file_path);
}

fn event1097(settings_menu: &mut SettingsMenu) {
    settings_menu.current_list_of_files_idx = 0;
    settings_menu.current_list_of_files_idx = 0;
    settings_menu.list_of_files = list_of_files_with_pacrb_extension();
    reload_list_of_files(settings_menu);
}

fn event1098(settings_menu: &mut SettingsMenu) {
    if settings_menu.current_list_of_files_idx >= settings_menu.list_of_files.len() {
        return;
    }
    settings_menu.current_list_of_files_idx += 5;
    reload_list_of_files(settings_menu);
}

fn event1099(settings_menu: &mut SettingsMenu) {
    settings_menu.current_list_of_files_idx =
        settings_menu.current_list_of_files_idx.saturating_sub(5);
    settings_menu.list_of_files = list_of_files_with_pacrb_extension();
    reload_list_of_files(settings_menu);
}

fn setup_load_event(settings_menu: &mut SettingsMenu, index: usize) -> Option<()> {
    let file_name = settings_menu
        .list_of_files
        .get(settings_menu.current_list_of_files_idx + index)?;

    settings_menu.file_to_load = Some(file_name.to_owned());

    Some(())
}

fn event1100(settings_menu: &mut SettingsMenu) {
    setup_load_event(settings_menu, 0);
}

fn event1101(settings_menu: &mut SettingsMenu) {
    setup_load_event(settings_menu, 1);
}

fn event1102(settings_menu: &mut SettingsMenu) {
    setup_load_event(settings_menu, 2);
}

fn event1103(settings_menu: &mut SettingsMenu) {
    setup_load_event(settings_menu, 3);
}

fn event1104(settings_menu: &mut SettingsMenu) {
    setup_load_event(settings_menu, 4);
}

fn event1200(settings_menu: &mut SettingsMenu) {
    setup_deletion_confirmation_prompt(settings_menu, settings_menu.current_list_of_files_idx);
}

fn event1201(settings_menu: &mut SettingsMenu) {
    setup_deletion_confirmation_prompt(settings_menu, settings_menu.current_list_of_files_idx + 1);
}

fn event1202(settings_menu: &mut SettingsMenu) {
    setup_deletion_confirmation_prompt(settings_menu, settings_menu.current_list_of_files_idx + 2);
}

fn event1203(settings_menu: &mut SettingsMenu) {
    setup_deletion_confirmation_prompt(settings_menu, settings_menu.current_list_of_files_idx + 3);
}

fn event1204(settings_menu: &mut SettingsMenu) {
    setup_deletion_confirmation_prompt(settings_menu, settings_menu.current_list_of_files_idx + 4);
}

fn event2000(event: &Event, settings_menu: &mut SettingsMenu) {
    let Events::TextBoxEvent(text_box_event) = event.event.clone() else {
        error!("event is not a string event! {:#?}", event);
        return;
    };

    let file_name = text_box_event.string;
    if file_name.ends_with(".pacrb") {
        settings_menu.save_file = file_name;
    } else {
        settings_menu.save_file = format!("{}{}", file_name, ".pacrb");
    }
}

fn event2001(settings_menu: &mut SettingsMenu) {
    settings_menu.trigger_save_event = TriggerFileStates::Save;
}

pub fn sync_events(settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    let export_file = &settings_menu.export_file();
    let save_file = settings_menu.save_file().to_string();
    settings_menu.settings_menu_dom
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 | 1 | 1000 | 1001 | 1002 | 1003 | 1004 => {}
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
            200 => {
                ele.sync(Syncs::String(export_file.clone()))
            }
            2000 => {
                set_save_file(ele, &save_file);
            }
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        })
}

fn setup_deletion_confirmation_prompt(settings_menu: &mut SettingsMenu, index: usize) {
    let Some(file_name) = settings_menu.list_of_files.get(index) else {
        return;
    };

    settings_menu
        .confirm_file_deletion
        .set_file_to_delete(file_name);
    settings_menu.confirm_file_deletion.set_display(true);
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

pub fn reload_list_of_files(settings_menu: &mut SettingsMenu) {
    let dom_controller = &mut settings_menu.settings_menu_dom;
    let list_of_files = &settings_menu.list_of_files;
    let mut idx = settings_menu.current_list_of_files_idx;
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            1000 | 1001 | 1002 | 1003 | 1004 => {
                let file_name = list_of_files.get(idx);
                idx += 1;

                if let Some(file_name) = file_name {
                    ele.sync(Syncs::String(file_name.clone()));
                } else {
                    ele.sync(Syncs::String("________________________".to_string()));
                }
            }
            _ => {}
        })
}

pub fn open_save_menu(settings_menu: &mut SettingsMenu, ui_settings: &UISettings) {
    event4(settings_menu, ui_settings)
}

pub fn set_save_file_traverse_dom(settings_menu: &mut SettingsMenu) {
    settings_menu
        .settings_menu_dom
        .root_node
        .traverse_dom_mut(&mut |ele| {
            if let 2000 = ele.sync_id() {
                set_save_file(ele, &settings_menu.save_file)
            }
        })
}

pub fn set_save_file(ele: &mut Element, new_save_file: &str) {
    ele.sync(Syncs::String(new_save_file.to_string()))
}

pub fn refresh_event(settings_menu: &mut SettingsMenu) {
    event1097(settings_menu)
}
