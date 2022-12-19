use bytemuck::{cast, Pod, Zeroable};

use super::input_code::*;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
// 60-64 bits aka 8 bytes when compressed
pub struct InputState {
    pub left_trigger: AnalogTrigger,
    pub right_trigger: AnalogTrigger,
    pub left_stick: AnalogStick,
    pub right_stick: AnalogStick,
    pub buttons: Buttons,
}

impl InputState {
    pub const INVALID_STATE: Self = Self {
        left_trigger: AnalogTrigger { state: -1 },
        right_trigger: AnalogTrigger { state: -1 },
        left_stick: AnalogStick {
            x_axis: 0,
            y_axis: 0,
        },
        right_stick: AnalogStick {
            x_axis: 0,
            y_axis: 0,
        },
        buttons: Buttons { state: 0 },
    };

    pub fn is_valid(self) -> bool {
        !self.left_trigger.state.is_negative() && !self.right_trigger.state.is_negative()
    }
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
// 16 bits
pub struct AnalogStick {
    x_axis: i8,
    y_axis: i8,
}

impl AnalogStick {
    pub fn set_x_axis(&mut self, value: f32) {
        assert!(value <= 1.0);
        assert!(value >= -1.0);
        self.x_axis = (value * i8::MAX as f32) as i8;
    }

    pub fn set_y_axis(&mut self, value: f32) {
        assert!(value <= 1.0);
        assert!(value >= -1.0);
        self.y_axis = (value * i8::MAX as f32) as i8;
    }

    pub fn get_x_axis(&self) -> f32 {
        self.x_axis as f32 / i8::MAX as f32
    }

    pub fn get_y_axis(&self) -> f32 {
        self.y_axis as f32 / i8::MAX as f32
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
// 7 bits
// Sign bit will be dropped/unused
pub struct AnalogTrigger {
    state: i8,
}

impl AnalogTrigger {
    const MASK: i8 = 0b0111_1111;

    pub fn get_value(&self) -> f32 {
        (self.state & Self::MASK) as f32 / Self::MASK as f32
    }

    pub fn set_value(&mut self, value: f32) {
        assert!(value <= 1.0);
        assert!(value >= 0.0);
        self.state = (value * i8::MAX as f32) as i8;
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
// 14 bits for with Analog Triggers
// 16 bits for binary triggers
pub struct Buttons {
    state: u16,
}

impl Buttons {
    pub fn enable_button(&mut self, code: ButtonCode) {
        self.state |= code.to_bit_mask();
    }

    pub fn get_button_state(&self, code: ButtonCode) -> bool {
        self.state & code.to_bit_mask() != 0
    }
}

unsafe impl Pod for Buttons {}
unsafe impl Pod for AnalogTrigger {}
unsafe impl Pod for AnalogStick {}
unsafe impl Pod for InputState {}

unsafe impl Zeroable for Buttons {}
unsafe impl Zeroable for AnalogTrigger {}
unsafe impl Zeroable for AnalogStick {}
unsafe impl Zeroable for InputState {}