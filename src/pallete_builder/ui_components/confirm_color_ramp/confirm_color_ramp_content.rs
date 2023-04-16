use tracing::warn;
use ui::{
    dom_controller::DomController,
    elements::traits::Element,
    events::{Event, Events},
};

use super::Orientation;

pub fn perform_events(
    events: &Vec<Event>,
    enable: &mut bool,
    orientation: &mut Orientation,
    cancel: &mut bool,
) {
    for event in events {
        perform_event(event, enable, orientation, cancel);
    }
}

fn perform_event(
    event: &Event,
    enable: &mut bool,
    orientation: &mut Orientation,
    cancel: &mut bool,
) {
    match event.id {
        0 => {}
        1 => event1(event, enable),
        2 => event2(event, orientation),
        3 => event3(event, enable, cancel),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event1(event: &Event, enable: &mut bool) {
    if let Events::BooleanEvent(_) = event.event {
        *enable = false
    }
}
fn event2(event: &Event, orientation: &mut Orientation) {
    if event.event == Events::BooleanEvent(true) {
        orientation.swap();
    }
}
fn event3(event: &Event, enable: &mut bool, cancel: &mut bool) {
    if event.event == Events::BooleanEvent(true) {
        *cancel = true;
        *enable = false;
    }
}

pub fn sync_events(dom_controller: &mut DomController, _enable: bool) {
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
        });
}
