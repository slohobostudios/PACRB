use sfml::system::Vector2f;
use tracing::{error, warn};
use ui::{
    dom_controller::DomController,
    elements::{
        traits::Element as ElementTrait, Element,
    },
    events::{Event, Events},
};

use crate::pallete_builder::hsv_color::HSV;

pub fn perform_events(events: &Vec<Event>, dom_controller: &mut DomController, hsv: &mut HSV) {
    for event in events {
        perform_event(event, dom_controller, hsv);
    }
}

fn perform_event(event: &Event, dom_controller: &mut DomController, hsv: &mut HSV) {
    match event.id {
        0 => {}
        1 => event1(event, dom_controller, hsv),
        2 => event2(dom_controller, event, hsv),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event1(event: &Event, dom_controller: &mut DomController, hsv: &mut HSV) {
    let Events::Vector2fEvent(sat_val) = event.event else {
        error!("event1: Event is not a Vector2fEvent");
        return;
    };

    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.event_id() {
            1 => {
                if let Element::Slider(ele) = ele {
                    let slider_size = ele.max_slider_value() - ele.min_slider_value();
                    hsv.s = ((sat_val.x / slider_size.x) * 255f32) as u8;
                    hsv.v = 255u8 - ((sat_val.y / slider_size.y) * 255f32) as u8;

                    return;
                }
            }
            _ => {}
        })
}

fn event2(dom_controller: &mut DomController, event: &Event, hsv: &mut HSV) {
    let Events::NumericalEvent(hue_event) = event.event else {
        error!("event2: Event is not a NumericalEvent");
        return;
    };
    hsv.h = hue_event as i16;
    sync_events(dom_controller, *hsv)
}

pub fn sync_events(dom_controller: &mut DomController, hsv: HSV) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 => {
                if let Element::Slider(ele) = ele {
                    let mut hsv = hsv;
                    hsv.s = 255;
                    hsv.v = 255;
                    let color = hsv.into();
                    ele.set_top_right_color(color);
                }
            }
            2 => {
                if let Element::Slider(ele) = ele {
                    ele.set_current_slider_value(Vector2f::new(f32::from(hsv.h), 0.));
                }
            }
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        });
}
