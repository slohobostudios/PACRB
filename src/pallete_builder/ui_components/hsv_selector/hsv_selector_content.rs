use std::str::FromStr;

use sfml::system::Vector2;
use tracing::{error, warn};
use ui::{
    dom_controller::DomController,
    elements::{
        slider::quad_color_picker::QuadColorPickerSync, textbox::traits::TextBoxTriggeredEvent,
        traits::Element as ElementTrait, Element,
    },
    events::{Event, Events},
    syncs::Syncs,
};
use utils::sfml_util_functions::try_from_color_hash_string_to_sfml_color;

use crate::pallete_builder::hsv_color::Hsv;

pub fn perform_events(
    events: &Vec<Event>,
    dom_controller: &mut DomController,
    hsv: &mut Hsv,
    hex_str: &mut String,
) {
    for event in events {
        perform_event(event, dom_controller, hsv, hex_str);
    }
}

fn perform_event(
    event: &Event,
    dom_controller: &mut DomController,
    hsv: &mut Hsv,
    hex_str: &mut String,
) {
    match event.id {
        0 => {}
        1 => event1(event, dom_controller, hsv, hex_str),
        2 => event2(event, dom_controller, hsv, hex_str),
        3 => event3(event, dom_controller, hsv, hex_str),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event1(event: &Event, dom_controller: &mut DomController, hsv: &mut Hsv, hex_str: &mut String) {
    let Events::Vector2fEvent(sat_val) = event.event else {
        error!("event1: Event is not a Vector2fEvent");
        return;
    };

    dom_controller.root_node.traverse_dom_mut(&mut |ele| {
        if let 1 = ele.sync_id() {
            if let Element::Slider(ele) = ele {
                let slider_size = ele.max_slider_value() - ele.min_slider_value();
                hsv.s = ((sat_val.x / slider_size.x) * 255f32) as u8;
                hsv.v = 255u8 - ((sat_val.y / slider_size.y) * 255f32) as u8;
            }
        }
    });
    *hex_str = hsv.to_string();
    sync_events(dom_controller, *hsv, hex_str)
}

fn event2(event: &Event, dom_controller: &mut DomController, hsv: &mut Hsv, hex_str: &mut String) {
    let Events::NumericalEvent(hue_event) = event.event else {
        error!("event2: Event is not a NumericalEvent");
        return;
    };
    hsv.h = hue_event as i16;
    *hex_str = hsv.to_string();
    sync_events(dom_controller, *hsv, hex_str)
}

fn event3(event: &Event, dom_controller: &mut DomController, hsv: &mut Hsv, hex_str: &mut String) {
    let Events::TextBoxEvent(text_box_event) = &event.event else {
        error!("event3: Event is not a StringEvent!");
        return;
    };

    fn set_hex_string_back_to_valid_state(
        text_box_event: &TextBoxTriggeredEvent,
        dom_controller: &mut DomController,
        hsv: Hsv,
        hex_str: &mut String,
    ) {
        if text_box_event.selected {
            return;
        }
        *hex_str = hsv.to_string();

        sync_events(dom_controller, hsv, hex_str)
    }

    let Ok(rgb) = try_from_color_hash_string_to_sfml_color(&text_box_event.string) else {
        set_hex_string_back_to_valid_state(text_box_event, dom_controller, *hsv, hex_str);
        return;
    };
    if rgb.a != 0xff {
        set_hex_string_back_to_valid_state(text_box_event, dom_controller, *hsv, hex_str);
        return;
    }
    *hex_str = text_box_event.string.clone();
    if !hex_str.starts_with('#') {
        *hex_str = format!("#{}", hex_str);
        sync_events(dom_controller, *hsv, hex_str);
    }
    let hsv_from_str = match Hsv::from_str(hex_str) {
        Ok(hsv) => hsv,
        Err(err) => {
            error!(
                "Failed converting {} into hsv value! Error: {:?}",
                hex_str, err
            );
            return;
        }
    };
    *hsv = hsv_from_str;
    sync_events_specific_sync(dom_controller, *hsv, hex_str, true, true, false)
}

pub fn sync_events(dom_controller: &mut DomController, hsv: Hsv, hex_str: &str) {
    sync_events_specific_sync(dom_controller, hsv, hex_str, true, true, true)
}

fn sync_events_specific_sync(
    dom_controller: &mut DomController,
    hsv: Hsv,
    hex_str: &str,
    one: bool,
    two: bool,
    three: bool,
) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 if one => {
                let full_bright_hsv = Hsv::new(hsv.h, u8::MAX, u8::MAX);
                ele.sync(Syncs::QuadColorPicker(QuadColorPickerSync {
                    top_right_color: Some(full_bright_hsv.into()),
                    bottom_right_color: Some(full_bright_hsv.into()),
                    hover_element_position_percentage: Some(Vector2::new(
                        (f32::from(hsv.s) / 255. * 65535.) as u16,
                        (65535. - f32::from(hsv.v) / 255. * 65535.) as u16,
                    )),
                    ..Default::default()
                }));
            }
            2 if two => {
                ele.sync(Syncs::Numerical(hsv.h.into()));
            }
            3 => {
                let Element::TextBox(text_box) = ele else {
                    error!("element is not a textbox!");
                    return;
                };
                if three && !text_box.is_selected() {
                    text_box.sync(Syncs::String(hex_str.to_owned()));
                    for _ in 0..hex_str.len() {
                        text_box.move_cursor_left();
                    }
                }
            }
            4 => {
                ele.sync(Syncs::QuadColorPicker(QuadColorPickerSync {
                    hover_element_position_percentage: Some(Vector2::new(
                        (f32::from(hsv.s) / 255. * 65535.) as u16,
                        (65535. - f32::from(hsv.v) / 255. * 65535.) as u16,
                    )),
                    ..Default::default()
                }));
            }
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        });
}
