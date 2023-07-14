use sfml::window::Event as SFMLEvent;

pub const DUMMY_MOUSE_MOVED_EVENT: SFMLEvent = SFMLEvent::MouseMoved {
    x: i32::MAX,
    y: i32::MAX,
};
