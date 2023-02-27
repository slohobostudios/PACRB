use super::Config;
use crate::{
    center_of_rect,
    ui::{dom_controller::DomController, events::*},
};
use tracing::{error, warn};

pub fn perform_events(events: &Vec<Event>, config: &mut Config) {
    for event in events {
        perform_event(event, config);
    }
}

fn perform_event(event: &Event, config: &mut Config) {
    match event.id {
        0 => {}
        1 => event1(*event, config),
        2 => event2(*event, config),
        3 => event3(*event, config),
        4 => event4(*event, config),
        5 => event5(*event, config),
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

use crate::ui::elements::{traits::Element as ElementTrait, Element};
use sfml::system::Vector2;
pub fn sync_events(dom_controller: &mut DomController, config: &Config) {
    dom_controller
        .root_node
        .traverse_dom_mut(&mut |ele| match ele.sync_id() {
            0 => {}
            1 => {
                let Element::Button(ele) = ele else { error!("{:#?} Element isn't a button", ele); return; };
                if let Events::BooleanEvent(state) = ele.triggered_event().event {
                    if state ^ config.auto_ramping {
                        ele.bind_pressed(center_of_rect!(i32, ele.global_bounds()));
                    }
                }
            }
            2 => {
                let Element::Slider(slider) = ele else { error!("{:#?} Element isn't Slider", ele); return;};
                slider.set_current_slider_value(Vector2::new(config.hue_shift.into(), 0f32))
            }
            3 => {
                let Element::Slider(slider) = ele else { error!("{:#?} Element isn't Slider", ele); return;};
                slider.set_current_slider_value(Vector2::new(config.num_of_shades.into(), 0f32))
            }
            4 => {
                let Element::Slider(slider) = ele else { error!("{:#?} Element isn't Slider", ele); return;};
                slider.set_current_slider_value(Vector2::new(config.saturation_shift.into(), 0f32))
            }
            5 => {
                let Element::Slider(slider) = ele else { error!("{:#?} Element isn't Slider", ele); return;};
                slider.set_current_slider_value(Vector2::new(config.value_shift.into(), 0f32))
            }
            sync_id => {
                warn!(
                    "Synchronization with sync_id {} has not yet been implemented!",
                    sync_id
                );
            }
        });
}
