use std::fmt;

use sdl2::controller::{Axis, Button};
use sdl2::keyboard::Keycode;

#[derive(Eq, PartialEq, Hash)]
pub enum PX8Key {
    Right, Left, Up, Down, O, X, Pause, Enter
}

impl fmt::Debug for PX8Key {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PX8Key::*;

        write!(f, "{}", match *self {
            Right => "RIGHT", Left => "LEFT", Up => "UP", Down => "DOWN", O => "O", X => "X", Pause => "Pause", Enter => "Enter"
        })

    }
}

pub fn map_button(button: Button) -> Option<PX8Key> {
    match button {
        Button::DPadRight => Some(PX8Key::Right),
        Button::DPadLeft => Some(PX8Key::Left),
        Button::DPadUp => Some(PX8Key::Up),
        Button::DPadDown => Some(PX8Key::Down),
        Button::A => Some(PX8Key::O),
        Button::B => Some(PX8Key::X),
        _ => None
    }
}

pub fn map_button_joystick(button: u8) -> Option<PX8Key> {
    match button {
        0 => Some(PX8Key::O),
        1 => Some(PX8Key::X),
        _ => None
    }
}

pub fn map_keycode(key: Keycode) -> (Option<PX8Key>, u8) {
    match key {
        Keycode::Right => (Some(PX8Key::Right), 0),
        Keycode::Left => (Some(PX8Key::Left), 0),
        Keycode::Up => (Some(PX8Key::Up), 0),
        Keycode::Down => (Some(PX8Key::Down), 0),
        Keycode::Z => (Some(PX8Key::O), 0),
        Keycode::C => (Some(PX8Key::O), 0),
        Keycode::N => (Some(PX8Key::O), 0),
        Keycode::X => (Some(PX8Key::X), 0),
        Keycode::V => (Some(PX8Key::X), 0),
        Keycode::M => (Some(PX8Key::X), 0),

        Keycode::F => (Some(PX8Key::Right), 1),
        Keycode::S => (Some(PX8Key::Left), 1),
        Keycode::E => (Some(PX8Key::Up), 1),
        Keycode::D => (Some(PX8Key::Down), 1),

        Keycode::LShift => (Some(PX8Key::O), 1),
        Keycode::Tab => (Some(PX8Key::O), 1),

        Keycode::A => (Some(PX8Key::X), 1),
        Keycode::Q => (Some(PX8Key::X), 1),

        Keycode::P => (Some(PX8Key::Pause), 0),
        Keycode::KpEnter => (Some(PX8Key::Enter), 0),

        _ => (None, 0)
    }
}

pub fn map_axis(axis: Axis, value: i16) -> Option<(PX8Key, bool)> {
    match axis {
        Axis::LeftX => match value {
            -32768...-16384 => Some((PX8Key::Left, true)),
            -16383...-1 => Some((PX8Key::Left, false)),
            0...16383 => Some((PX8Key::Right, false)),
            16384...32767 => Some((PX8Key::Right, true)),
            _ => None
        },

        Axis::LeftY => match value {
            -32768...-16384 => Some((PX8Key::Up, true)),
            -16383...-1 => Some((PX8Key::Up, false)),
            0...16383 => Some((PX8Key::Down, false)),
            16384...32767 => Some((PX8Key::Down, true)),
            _ => None
        },
        _ => None
    }
}

pub fn map_axis_joystick(axis: u8, value: i16) -> Option<(PX8Key, bool)> {
    match axis {
        0 => match value {
            -32768...-16384 => Some((PX8Key::Left, true)),
            -16383...-1 => Some((PX8Key::Left, false)),
            0...16383 => Some((PX8Key::Right, false)),
            16384...32767 => Some((PX8Key::Right, true)),
            _ => None
        },

        1 => match value {
            -32768...-16384 => Some((PX8Key::Up, true)),
            -16383...-1 => Some((PX8Key::Up, false)),
            0...16383 => Some((PX8Key::Down, false)),
            16384...32767 => Some((PX8Key::Down, true)),
            _ => None
        },
        _ => None
    }
}