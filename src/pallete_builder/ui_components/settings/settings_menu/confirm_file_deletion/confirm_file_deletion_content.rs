use tracing::warn;
use ui::{dom_controller::DomController, elements::traits::Element, events::Event, syncs::Syncs};

use super::{ConfirmFileDeletion, ConfirmFileDeletionSelection};

pub fn perform_events(events: &Vec<Event>, confirm_file_deletion: &mut ConfirmFileDeletion) {
    for event in events {
        perform_event(event, confirm_file_deletion);
    }
}

fn perform_event(event: &Event, confirm_file_deletion: &mut ConfirmFileDeletion) {
    match event.id {
        0 | 1 => {}
        2 => event2(confirm_file_deletion),
        3 => event3(confirm_file_deletion),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event2(confirm_file_deletion: &mut ConfirmFileDeletion) {
    confirm_file_deletion.remove_file();
    confirm_file_deletion.confirm_file_deletion_selection = ConfirmFileDeletionSelection::Delete;
}

fn event3(confirm_file_deletion: &mut ConfirmFileDeletion) {
    confirm_file_deletion.confirm_file_deletion_selection = ConfirmFileDeletionSelection::Cancel;
}

pub fn sync_events(dom_controller: &mut DomController, file_name: &str) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 => ele.sync(Syncs::String(format!(
                "Are you sure you want to delete {}?",
                file_name
            ))),
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        })
}
