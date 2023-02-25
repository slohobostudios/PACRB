use crate::{
    center_of_rect,
    ui::{dom_controller::DomController, events::*},
};
use tracing::{error, warn};

pub fn perform_events(events: &Vec<Event>, erase_enabled: &mut bool) {
    for event in events {
        perform_event(event, erase_enabled);
    }
}

fn perform_event(event: &Event, erase_enabled: &mut bool) {
    match event.id {
        0 => {}
        1 => event1(event, erase_enabled),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event1(event: &Event, erase_enabled: &mut bool) {
    if let Events::BooleanEvent(val) = event.event {
        *erase_enabled = !val
    }
}

use crate::ui::elements::{traits::Element as ElementTrait, Element};
pub fn sync_events(dom_controller: &mut DomController, erase_enabled: bool) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 => {
                let Element::Button(ele) = ele else { error!("{:#?} Element isn't Button", ele); return; };
                if let Events::BooleanEvent(state) = ele.triggered_event().event {
                    if erase_enabled ^ state {
                        ele.bind_pressed(center_of_rect!(i32, ele.global_bounds()));
                    }
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
