use sfml::system::Vector2f;

use crate::elements::textbox::traits::TextBoxTriggeredEvent;

pub type EventId = u16;

#[derive(Clone, PartialEq, Debug)]
pub struct Event {
    pub id: EventId,
    pub event: Events,
}

impl Event {
    pub fn new(event_id: EventId, event: Events) -> Self {
        Self {
            id: event_id,
            event,
        }
    }
}

pub const EMPTY_EVENT: Event = Event {
    id: 0,
    event: Events::Null,
};

impl Default for Event {
    fn default() -> Self {
        EMPTY_EVENT
    }
}

#[derive(Clone, PartialEq, Debug, Default)]
pub enum Events {
    BooleanEvent(bool),
    NumericalEvent(f32),
    Vector2fEvent(Vector2f),
    StringEvent(String),
    TextBoxEvent(TextBoxTriggeredEvent),
    #[default]
    Null,
}
