use std::fmt;
use config::scancode::Scancode;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
pub enum PX8Key {
    Right,
    Left,
    Up,
    Down,
    A,
    B,
    Pause,
    Enter,
}

impl fmt::Debug for PX8Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PX8Key::*;

        write!(f,
               "{}",
               match *self {
                   Right => "RIGHT",
                   Left => "LEFT",
                   Up => "UP",
                   Down => "DOWN",
                   A => "A",
                   B => "B",
                   Pause => "Pause",
                   Enter => "Enter",
               })

    }
}


pub fn map_keycode(key: Scancode) -> (Option<PX8Key>, u8) {
    match key {
        Scancode::Right => (Some(PX8Key::Right), 0),
        Scancode::Left => (Some(PX8Key::Left), 0),
        Scancode::Up => (Some(PX8Key::Up), 0),
        Scancode::Down => (Some(PX8Key::Down), 0),

        Scancode::Z | Scancode::C | Scancode::N => (Some(PX8Key::A), 0),
        Scancode::X | Scancode::V | Scancode::M => (Some(PX8Key::B), 0),

        Scancode::F => (Some(PX8Key::Right), 1),
        Scancode::S => (Some(PX8Key::Left), 1),
        Scancode::E => (Some(PX8Key::Up), 1),
        Scancode::D => (Some(PX8Key::Down), 1),

        Scancode::LShift | Scancode::Tab => (Some(PX8Key::A), 1),

        Scancode::A | Scancode::Q => (Some(PX8Key::A), 1),
        Scancode::B => (Some(PX8Key::B), 1),

        Scancode::Escape => (Some(PX8Key::Pause), 0),

        Scancode::KpEnter | Scancode::Return => (Some(PX8Key::Enter), 0),

        _ => (None, 0),
    }
}