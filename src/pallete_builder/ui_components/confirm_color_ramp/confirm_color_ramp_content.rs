use crate::ui::{dom_controller::DomController, events::*};
use tracing::warn;

use super::Orientation;

pub fn perform_events(events: &Vec<Event>, display: &mut bool, orientation: &mut Orientation) {
    for event in events {
        perform_event(event, display, orientation);
    }
}

fn perform_event(event: &Event, display: &mut bool, orientation: &mut Orientation) {
    match event.id {
        0 => {}
        1 => event1(event, display),
        2 => event2(event, orientation),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event1(event: &Event, display: &mut bool) {
    if let Events::BooleanEvent(val) = event.event {
        *display = val
    }
}
fn event2(event: &Event, orientation: &mut Orientation) {
    if event.event == Events::BooleanEvent(true) {
        orientation.swap();
    }
}

use crate::ui::elements::{traits::Element as ElementTrait, Element};

pub fn sync_events(dom_controller: &mut DomController, display: bool) {
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
