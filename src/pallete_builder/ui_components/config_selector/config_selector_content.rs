use super::{Config, ConfigSelector};

use tracing::warn;
use ui::{
    dom_controller::DomController,
    elements::traits::Element as ElementTrait,
    events::{Event, Events},
    syncs::Syncs,
};

pub fn perform_events(events: &Vec<Event>, config_selector: &mut ConfigSelector) {
    for event in events {
        perform_event(event, config_selector);
    }
}

fn perform_event(event: &Event, config_selector: &mut ConfigSelector) {
    match event.id {
        0 => {}
        1 => event1(event.clone(), &mut config_selector.current_config),
        2 => event2(event.clone(), &mut config_selector.current_config),
        3 => event3(event.clone(), &mut config_selector.current_config),
        4 => event4(event.clone(), &mut config_selector.current_config),
        5 => event5(event.clone(), &mut config_selector.current_config),
        6 => event6(config_selector),
        _ => {
            warn!("Event: {:#?} is not yet implemented", event)
        }
    }
}

fn event1(event: Event, config: &mut Config) {
    if let Events::BooleanEvent(val) = event.event {
        config.auto_ramping = val
    }
}
fn event2(event: Event, config: &mut Config) {
    if let Events::NumericalEvent(val) = event.event {
        config.hue_shift = val as i8;
    }
}
fn event3(event: Event, config: &mut Config) {
    if let Events::NumericalEvent(val) = event.event {
        config.num_of_shades = val as u8;
    }
}
fn event4(event: Event, config: &mut Config) {
    if let Events::NumericalEvent(val) = event.event {
        config.saturation_shift = val as i8;
    }
}
fn event5(event: Event, config: &mut Config) {
    if let Events::NumericalEvent(val) = event.event {
        config.value_shift = val as i8;
    }
}
fn event6(config_selector: &mut ConfigSelector) {
    config_selector.current_config.hue_shift *= -1;
    config_selector.current_config.saturation_shift *= -1;
    config_selector.current_config.value_shift *= -1;

    sync_events(
        &mut config_selector.config_selector_dom,
        &config_selector.current_config,
    );
}

pub fn sync_events(dom_controller: &mut DomController, config: &Config) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 => {
                ele.sync(Syncs::Boolean(config.auto_ramping));
            }
            2 => {
                ele.sync(Syncs::Numerical(config.hue_shift.into()));
            }
            3 => {
                ele.sync(Syncs::Numerical(config.num_of_shades.into()));
            }
            4 => {
                ele.sync(Syncs::Numerical(config.saturation_shift.into()));
            }
            5 => {
                ele.sync(Syncs::Numerical(config.value_shift.into()));
            }
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        });
}
