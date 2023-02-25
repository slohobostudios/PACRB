use sfml::system::Vector2f;

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Event {
    pub id: u16,
    pub event: Events,
}

impl Event {
    pub fn new(event_id: u16, event: Events) -> Self {
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

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Events {
    BooleanEvent(bool),
    NumericalEvent(f32),
    Vector2fEvent(Vector2f),
    Null,
}

impl Default for Events {
    fn default() -> Self {
        Events::Null
    }
}
