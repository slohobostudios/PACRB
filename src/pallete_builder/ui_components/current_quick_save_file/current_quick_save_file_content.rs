use std::{cell::RefCell, rc::Rc};

use tracing::{error, warn};
use ui::{
    dom_controller::DomController,
    elements::{traits::Element as ElementTrait, Element},
    syncs::Syncs,
};

pub fn current_save_file(dom_controller: &DomController) -> String {
    let current_save_file = Rc::new(RefCell::new(String::new()));
    dom_controller
        .root_node
        .traverse_dom(&|ele| match ele.sync_id() {
            0 => {}
            1 => {
                let Element::Text(text) = ele else {
                error!("ele is not a Text element");
                    return;
                };

                *current_save_file.borrow_mut() = text.text();
            }
            _ => {}
        });

    let x = RefCell::borrow(&current_save_file).clone();
    x
}

pub fn sync_events(dom_controller: &mut DomController, current_save_file: &str) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 => ele.sync(Syncs::String(current_save_file.to_string())),
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        });
}
