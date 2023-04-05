use super::Config;

use tracing::warn;
use ui::{
    dom_controller::DomController,
    elements::traits::Element as ElementTrait,
    events::{Event, Events},
    syncs::Syncs,
};

pub fn perform_events(events: &Vec<Event>, config: &mut Config) {
    for event in events {
        perform_event(event, config);
    }
}

fn perform_event(event: &Event, config: &mut Config) {
    match event.id {
        0 => {}
        1 => event1(event.clone(), config),
        2 => event2(event.clone(), config),
        3 => event3(event.clone(), config),
        4 => event4(event.clone(), config),
        5 => event5(event.clone(), config),
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

use sfml::system::Vector2f;
pub fn sync_events(dom_controller: &mut DomController, config: &Config) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 => {
                ele.sync(Syncs::Boolean(config.auto_ramping));
            }
            2 => {
                ele.sync(Syncs::Vector2f(Vector2f::new(
                    config.hue_shift.into(),
                    0f32,
                )));
            }
            3 => {
                ele.sync(Syncs::Vector2f(Vector2f::new(
                    config.num_of_shades.into(),
                    0f32,
                )));
            }
            4 => {
                ele.sync(Syncs::Vector2f(Vector2f::new(
                    config.saturation_shift.into(),
                    0f32,
                )));
            }
            5 => {
                ele.sync(Syncs::Vector2f(Vector2f::new(
                    config.value_shift.into(),
                    0f32,
                )));
            }
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        });
}
