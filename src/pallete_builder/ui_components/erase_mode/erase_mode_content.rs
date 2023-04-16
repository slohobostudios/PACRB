use tracing::warn;
use ui::{
    dom_controller::DomController,
    elements::traits::Element as ElementTrait,
    events::{Event, Events},
    syncs::Syncs,
};

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

pub fn sync_events(dom_controller: &mut DomController, erase_enabled: bool) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 => {
                ele.sync(Syncs::Boolean(!erase_enabled));
            }
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        });
}
