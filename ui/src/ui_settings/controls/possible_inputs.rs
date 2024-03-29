use serde::{Deserialize, Serialize};
use sfml::window::{
    mouse::{Button, Wheel},
    Key,
};
use utils::simple_error::SimpleError;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PossibleInputs {
    // Mouse Buttons
    ButtonLeft,
    ButtonRight,
    ButtonMiddle,
    Button1,
    Button2,
    // Mouse Wheel,
    VerticalWheel,
    HorizontalWheel,
    // Keys
    Unknown,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Escape,
    LControl,
    LShift,
    LAlt,
    LSystem,
    RControl,
    RShift,
    RAlt,
    RSystem,
    Menu,
    LBracket,
    RBracket,
    Semicolon,
    Comma,
    Period,
    Quote,
    Slash,
    Backslash,
    Tilde,
    Equal,
    Hyphen,
    Space,
    Enter,
    Backspace,
    Tab,
    PageUp,
    PageDown,
    End,
    Home,
    Insert,
    Delete,
    Add,
    Subtract,
    Multiply,
    Divide,
    Left,
    Right,
    Up,
    Down,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    Pause,
}

impl From<Button> for PossibleInputs {
    fn from(button: Button) -> Self {
        match button {
            Button::Left => Self::ButtonLeft,
            Button::Right => Self::ButtonRight,
            Button::Middle => Self::ButtonMiddle,
            Button::XButton1 => Self::Button1,
            Button::XButton2 => Self::Button2,
        }
    }
}

impl TryFrom<PossibleInputs> for Button {
    type Error = SimpleError;
    fn try_from(pi: PossibleInputs) -> Result<Button, SimpleError> {
        match pi {
            PossibleInputs::ButtonLeft => Ok(Button::Left),
            PossibleInputs::ButtonRight => Ok(Button::Right),
            PossibleInputs::ButtonMiddle => Ok(Button::Middle),
            PossibleInputs::Button1 => Ok(Button::XButton1),
            PossibleInputs::Button2 => Ok(Button::XButton2),
            _ => Err(SimpleError::new(format!("ui::ui_settings::controls::possible_inputs: Into<Button> Input {:#?} is not a mouse button!", pi))),
        }
    }
}

impl From<Wheel> for PossibleInputs {
    fn from(wheel: Wheel) -> Self {
        match wheel {
            Wheel::VerticalWheel => Self::VerticalWheel,
            Wheel::HorizontalWheel => Self::HorizontalWheel,
        }
    }
}

impl TryFrom<PossibleInputs> for Wheel {
    type Error = SimpleError;
    fn try_from(pi: PossibleInputs) -> Result<Wheel, SimpleError> {
        match pi {
            PossibleInputs::VerticalWheel => Ok(Wheel::VerticalWheel),
            PossibleInputs::HorizontalWheel => Ok(Wheel::HorizontalWheel),
            _ => Err(SimpleError::new(format!("ui::ui_settings::controls::possible_inputs: Into<Button> Input {:#?} is not a mouse button!", pi))),
        }
    }
}

impl From<Key> for PossibleInputs {
    fn from(key: Key) -> Self {
        match key {
            Key::Unknown => Self::Unknown,
            Key::A => Self::A,
            Key::B => Self::B,
            Key::C => Self::C,
            Key::D => Self::D,
            Key::E => Self::E,
            Key::F => Self::F,
            Key::G => Self::G,
            Key::H => Self::H,
            Key::I => Self::I,
            Key::J => Self::J,
            Key::K => Self::K,
            Key::L => Self::L,
            Key::M => Self::M,
            Key::N => Self::N,
            Key::O => Self::O,
            Key::P => Self::P,
            Key::Q => Self::Q,
            Key::R => Self::R,
            Key::S => Self::S,
            Key::T => Self::T,
            Key::U => Self::U,
            Key::V => Self::V,
            Key::W => Self::W,
            Key::X => Self::X,
            Key::Y => Self::Y,
            Key::Z => Self::Z,
            Key::Num0 => Self::Num0,
            Key::Num1 => Self::Num1,
            Key::Num2 => Self::Num2,
            Key::Num3 => Self::Num3,
            Key::Num4 => Self::Num4,
            Key::Num5 => Self::Num5,
            Key::Num6 => Self::Num6,
            Key::Num7 => Self::Num7,
            Key::Num8 => Self::Num8,
            Key::Num9 => Self::Num9,
            Key::Escape => Self::Escape,
            Key::LControl => Self::LControl,
            Key::LShift => Self::LShift,
            Key::LAlt => Self::LAlt,
            Key::LSystem => Self::LSystem,
            Key::RControl => Self::RControl,
            Key::RShift => Self::RShift,
            Key::RAlt => Self::RAlt,
            Key::RSystem => Self::RSystem,
            Key::Menu => Self::Menu,
            Key::LBracket => Self::LBracket,
            Key::RBracket => Self::RBracket,
            Key::Semicolon => Self::Semicolon,
            Key::Comma => Self::Comma,
            Key::Period => Self::Period,
            Key::Quote => Self::Quote,
            Key::Slash => Self::Slash,
            Key::Backslash => Self::Backslash,
            Key::Tilde => Self::Tilde,
            Key::Equal => Self::Equal,
            Key::Hyphen => Self::Hyphen,
            Key::Space => Self::Space,
            Key::Enter => Self::Enter,
            Key::Backspace => Self::Backspace,
            Key::Tab => Self::Tab,
            Key::PageUp => Self::PageUp,
            Key::PageDown => Self::PageDown,
            Key::End => Self::End,
            Key::Home => Self::Home,
            Key::Insert => Self::Insert,
            Key::Delete => Self::Delete,
            Key::Add => Self::Add,
            Key::Subtract => Self::Subtract,
            Key::Multiply => Self::Multiply,
            Key::Divide => Self::Divide,
            Key::Left => Self::Left,
            Key::Right => Self::Right,
            Key::Up => Self::Up,
            Key::Down => Self::Down,
            Key::Numpad0 => Self::Numpad0,
            Key::Numpad1 => Self::Numpad1,
            Key::Numpad2 => Self::Numpad2,
            Key::Numpad3 => Self::Numpad3,
            Key::Numpad4 => Self::Numpad4,
            Key::Numpad5 => Self::Numpad5,
            Key::Numpad6 => Self::Numpad6,
            Key::Numpad7 => Self::Numpad7,
            Key::Numpad8 => Self::Numpad8,
            Key::Numpad9 => Self::Numpad9,
            Key::F1 => Self::F1,
            Key::F2 => Self::F2,
            Key::F3 => Self::F3,
            Key::F4 => Self::F4,
            Key::F5 => Self::F5,
            Key::F6 => Self::F6,
            Key::F7 => Self::F7,
            Key::F8 => Self::F8,
            Key::F9 => Self::F9,
            Key::F10 => Self::F10,
            Key::F11 => Self::F11,
            Key::F12 => Self::F12,
            Key::F13 => Self::F13,
            Key::F14 => Self::F14,
            Key::F15 => Self::F15,
            Key::Pause => Self::Pause,
        }
    }
}

impl TryFrom<PossibleInputs> for Key {
    type Error = SimpleError;
    fn try_from(pi: PossibleInputs) -> Result<Key, SimpleError> {
        match pi {
            PossibleInputs::Unknown => Ok(Key::Unknown),
            PossibleInputs::A => Ok(Key::A),
            PossibleInputs::B => Ok(Key::B),
            PossibleInputs::C => Ok(Key::C),
            PossibleInputs::D => Ok(Key::D),
            PossibleInputs::E => Ok(Key::E),
            PossibleInputs::F => Ok(Key::F),
            PossibleInputs::G => Ok(Key::G),
            PossibleInputs::H => Ok(Key::H),
            PossibleInputs::I => Ok(Key::I),
            PossibleInputs::J => Ok(Key::J),
            PossibleInputs::K => Ok(Key::K),
            PossibleInputs::L => Ok(Key::L),
            PossibleInputs::M => Ok(Key::M),
            PossibleInputs::N => Ok(Key::N),
            PossibleInputs::O => Ok(Key::O),
            PossibleInputs::P => Ok(Key::P),
            PossibleInputs::Q => Ok(Key::Q),
            PossibleInputs::R => Ok(Key::R),
            PossibleInputs::S => Ok(Key::S),
            PossibleInputs::T => Ok(Key::T),
            PossibleInputs::U => Ok(Key::U),
            PossibleInputs::V => Ok(Key::V),
            PossibleInputs::W => Ok(Key::W),
            PossibleInputs::X => Ok(Key::X),
            PossibleInputs::Y => Ok(Key::Y),
            PossibleInputs::Z => Ok(Key::Z),
            PossibleInputs::Num0 => Ok(Key::Num0),
            PossibleInputs::Num1 => Ok(Key::Num1),
            PossibleInputs::Num2 => Ok(Key::Num2),
            PossibleInputs::Num3 => Ok(Key::Num3),
            PossibleInputs::Num4 => Ok(Key::Num4),
            PossibleInputs::Num5 => Ok(Key::Num5),
            PossibleInputs::Num6 => Ok(Key::Num6),
            PossibleInputs::Num7 => Ok(Key::Num7),
            PossibleInputs::Num8 => Ok(Key::Num8),
            PossibleInputs::Num9 => Ok(Key::Num9),
            PossibleInputs::Escape => Ok(Key::Escape),
            PossibleInputs::LControl => Ok(Key::LControl),
            PossibleInputs::LShift => Ok(Key::LShift),
            PossibleInputs::LAlt => Ok(Key::LAlt),
            PossibleInputs::LSystem => Ok(Key::LSystem),
            PossibleInputs::RControl => Ok(Key::RControl),
            PossibleInputs::RShift => Ok(Key::RShift),
            PossibleInputs::RAlt => Ok(Key::RAlt),
            PossibleInputs::RSystem => Ok(Key::RSystem),
            PossibleInputs::Menu => Ok(Key::Menu),
            PossibleInputs::LBracket => Ok(Key::LBracket),
            PossibleInputs::RBracket => Ok(Key::RBracket),
            PossibleInputs::Semicolon => Ok(Key::Semicolon),
            PossibleInputs::Comma => Ok(Key::Comma),
            PossibleInputs::Period => Ok(Key::Period),
            PossibleInputs::Quote => Ok(Key::Quote),
            PossibleInputs::Slash => Ok(Key::Slash),
            PossibleInputs::Backslash => Ok(Key::Backslash),
            PossibleInputs::Tilde => Ok(Key::Tilde),
            PossibleInputs::Equal => Ok(Key::Equal),
            PossibleInputs::Hyphen => Ok(Key::Hyphen),
            PossibleInputs::Space => Ok(Key::Space),
            PossibleInputs::Enter => Ok(Key::Enter),
            PossibleInputs::Backspace => Ok(Key::Backspace),
            PossibleInputs::Tab => Ok(Key::Tab),
            PossibleInputs::PageUp => Ok(Key::PageUp),
            PossibleInputs::PageDown => Ok(Key::PageDown),
            PossibleInputs::End => Ok(Key::End),
            PossibleInputs::Home => Ok(Key::Home),
            PossibleInputs::Insert => Ok(Key::Insert),
            PossibleInputs::Delete => Ok(Key::Delete),
            PossibleInputs::Add => Ok(Key::Add),
            PossibleInputs::Subtract => Ok(Key::Subtract),
            PossibleInputs::Multiply => Ok(Key::Multiply),
            PossibleInputs::Divide => Ok(Key::Divide),
            PossibleInputs::Left => Ok(Key::Left),
            PossibleInputs::Right => Ok(Key::Right),
            PossibleInputs::Up => Ok(Key::Up),
            PossibleInputs::Down => Ok(Key::Down),
            PossibleInputs::Numpad0 => Ok(Key::Numpad0),
            PossibleInputs::Numpad1 => Ok(Key::Numpad1),
            PossibleInputs::Numpad2 => Ok(Key::Numpad2),
            PossibleInputs::Numpad3 => Ok(Key::Numpad3),
            PossibleInputs::Numpad4 => Ok(Key::Numpad4),
            PossibleInputs::Numpad5 => Ok(Key::Numpad5),
            PossibleInputs::Numpad6 => Ok(Key::Numpad6),
            PossibleInputs::Numpad7 => Ok(Key::Numpad7),
            PossibleInputs::Numpad8 => Ok(Key::Numpad8),
            PossibleInputs::Numpad9 => Ok(Key::Numpad9),
            PossibleInputs::F1 => Ok(Key::F1),
            PossibleInputs::F2 => Ok(Key::F2),
            PossibleInputs::F3 => Ok(Key::F3),
            PossibleInputs::F4 => Ok(Key::F4),
            PossibleInputs::F5 => Ok(Key::F5),
            PossibleInputs::F6 => Ok(Key::F6),
            PossibleInputs::F7 => Ok(Key::F7),
            PossibleInputs::F8 => Ok(Key::F8),
            PossibleInputs::F9 => Ok(Key::F9),
            PossibleInputs::F10 => Ok(Key::F10),
            PossibleInputs::F11 => Ok(Key::F11),
            PossibleInputs::F12 => Ok(Key::F12),
            PossibleInputs::F13 => Ok(Key::F13),
            PossibleInputs::F14 => Ok(Key::F14),
            PossibleInputs::F15 => Ok(Key::F15),
            PossibleInputs::Pause => Ok(Key::Pause),
            _ => Err(SimpleError::new(format!(
                "ui::ui_settings::controls::possible_inputs: Into<Key> Input {:#?} is not a key!",
                pi
            ))),
        }
    }
}
