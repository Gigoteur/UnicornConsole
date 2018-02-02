use unicorn::config::keys::PX8Key;

use sdl2::controller::{Axis, Button};
use sdl2::keyboard::Scancode;

pub fn map_button(button: Button) -> Option<PX8Key> {
    match button {
        Button::DPadRight => Some(PX8Key::Right),
        Button::DPadLeft => Some(PX8Key::Left),
        Button::DPadUp => Some(PX8Key::Up),
        Button::DPadDown => Some(PX8Key::Down),
        Button::A => Some(PX8Key::A),
        Button::B => Some(PX8Key::B),
        _ => None,
    }
}

pub fn map_button_joystick(button: u8) -> Option<PX8Key> {
    match button {
        0 => Some(PX8Key::A),
        1 => Some(PX8Key::B),
        _ => None,
    }
}

pub fn map_axis(axis: Axis, value: i16) -> Option<(PX8Key, bool)> {
    match axis {
        Axis::LeftX => {
            match value {
                -32768...-16384 => Some((PX8Key::Left, true)),
                -16383...-1 => Some((PX8Key::Left, false)),
                0...16383 => Some((PX8Key::Right, false)),
                16384...32767 => Some((PX8Key::Right, true)),
                _ => None,
            }
        }

        Axis::LeftY => {
            match value {
                -32768...-16384 => Some((PX8Key::Up, true)),
                -16383...-1 => Some((PX8Key::Up, false)),
                0...16383 => Some((PX8Key::Down, false)),
                16384...32767 => Some((PX8Key::Down, true)),
                _ => None,
            }
        }
        _ => None,
    }
}

pub fn map_axis_joystick(axis: u8, value: i16) -> Option<(PX8Key, bool)> {
    match axis {
        0 => {
            match value {
                -32768...-16384 => Some((PX8Key::Left, true)),
                -16383...-1 => Some((PX8Key::Left, false)),
                1...16383 => Some((PX8Key::Right, false)),
                16384...32767 => Some((PX8Key::Right, true)),
                _ => None,
            }
        }

        1 => {
            match value {
                -32768...-16384 => Some((PX8Key::Up, true)),
                -16383...-1 => Some((PX8Key::Up, false)),
                1...16383 => Some((PX8Key::Down, false)),
                16384...32767 => Some((PX8Key::Down, true)),
                _ => None,
            }
        }
        _ => None,
    }
}
