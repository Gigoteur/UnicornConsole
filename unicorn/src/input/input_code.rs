use serde::{Deserialize, Serialize};
use strum::EnumIter;

use super::AsApiCode;

#[derive(Debug, Copy, Clone, EnumIter, Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum ButtonCode {
    // DPad
    Up,
    Down,
    Left,
    Right,

    // Buttons
    ButtonA,
    ButtonB,
    ButtonC,
    ButtonD,
    Start,
    Select,
    LeftShoulder,
    RightShoulder,
    LeftStick,
    RightStick,

    // Emulated
    LeftTrigger,
    RightTrigger,
}

impl ButtonCode {
    const API_UP: u8 = 0;
    const API_DOWN: u8 = 1;
    const API_LEFT: u8 = 2;
    const API_RIGHT: u8 = 3;
    const API_A: u8 = 4;
    const API_B: u8 = 5;
    const API_C: u8 = 6;
    const API_D: u8 = 7;
    const API_START: u8 = 8;
    const API_SELECT: u8 = 9;
    const API_LEFT_SHOULDER: u8 = 10;
    const API_RIGHT_SHOULDER: u8 = 11;
    const API_LEFT_STICK: u8 = 12;
    const API_RIGHT_STICK: u8 = 13;
    const API_LEFT_TRIGGER: u8 = 14;
    const API_RIGHT_TRIGGER: u8 = 15;
}

impl AsApiCode for ButtonCode {
    fn to_api_code(&self) -> u8 {
        match self {
            Self::Up => Self::API_UP,
            Self::Down => Self::API_DOWN,
            Self::Left => Self::API_LEFT,
            Self::Right => Self::API_RIGHT,
            Self::ButtonA => Self::API_A,
            Self::ButtonB => Self::API_B,
            Self::ButtonC => Self::API_C,
            Self::ButtonD => Self::API_D,
            Self::Start => Self::API_START,
            Self::Select => Self::API_SELECT,
            Self::LeftShoulder => Self::API_LEFT_SHOULDER,
            Self::RightShoulder => Self::API_RIGHT_SHOULDER,
            Self::LeftStick => Self::API_LEFT_STICK,
            Self::RightStick => Self::API_RIGHT_STICK,
            Self::LeftTrigger => Self::API_LEFT_TRIGGER,
            Self::RightTrigger => Self::API_RIGHT_TRIGGER,
        }
    }

    fn from_api_code(code: u8) -> Option<Self> {
        match code {
            Self::API_UP => Some(Self::Up),
            Self::API_DOWN => Some(Self::Down),
            Self::API_LEFT => Some(Self::Left),
            Self::API_RIGHT => Some(Self::Right),
            Self::API_A => Some(Self::ButtonA),
            Self::API_B => Some(Self::ButtonB),
            Self::API_C => Some(Self::ButtonC),
            Self::API_D => Some(Self::ButtonD),
            Self::API_START => Some(Self::Start),
            Self::API_SELECT => Some(Self::Select),
            Self::API_LEFT_SHOULDER => Some(Self::LeftShoulder),
            Self::API_RIGHT_SHOULDER => Some(Self::RightShoulder),
            Self::API_LEFT_STICK => Some(Self::LeftStick),
            Self::API_RIGHT_STICK => Some(Self::RightStick),
            Self::API_LEFT_TRIGGER => Some(Self::LeftTrigger),
            Self::API_RIGHT_TRIGGER => Some(Self::RightTrigger),
            _ => None,
        }
    }
}

impl ToBitMask<u16> for ButtonCode {
    fn to_bit_mask(&self) -> u16 {
        match self {
            Self::Up => 0b100_0000,
            Self::Down => 0b1000_0000,
            Self::Left => 0b1_0000_0000,
            Self::Right => 0b10_0000_0000,
            Self::ButtonA => 0b1,
            Self::ButtonB => 0b10,
            Self::ButtonC => 0b100,
            Self::ButtonD => 0b1000,
            Self::Start => 0b1_0000,
            Self::Select => 0b10_0000,
            Self::LeftShoulder => 0b100_0000_0000,
            Self::RightShoulder => 0b1000_0000_0000,
            Self::LeftStick => 0b1_0000_0000_0000,
            Self::RightStick => 0b10_0000_0000_0000,
            Self::LeftTrigger => 0b100_0000_0000_0000,
            Self::RightTrigger => 0b1000_0000_0000_0000,
        }
    }
}

pub trait ToBitMask<T> {
    fn to_bit_mask(&self) -> T;
}