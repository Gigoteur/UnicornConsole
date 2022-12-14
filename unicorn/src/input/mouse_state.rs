#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct MouseState(pub u64);

// Uses 29/32 bits
const MASK: u64 = 0b111_1111_1111; // 11 bits
const X_SHIFT: u64 = 0;
const Y_SHIFT: u64 = 11;
const LEFT_BUTTON_SHIFT: u64 = 22;
const RIGHT_BUTTON_SHIFT: u64 = 23;
const MIDDLE_BUTTON_SHIFT: u64 = 24;
const WHEEL_UP: u64 = 25;
const WHEEL_DOWN: u64 = 26;
const WHEEL_LEFT: u64 = 27;
const WHEEL_RIGHT: u64 = 28;

pub const MOUSE_INVALID_BIT: u64 = 31;

// Uses 32/32 bits
const DELTA_MASK: u64 = u16::MAX as u64;
const X_DELTA_SHIFT: u64 = 32;
const Y_DELTA_SHIFT: u64 = 32 + u16::BITS as u64;

impl MouseState {
    pub fn get_x_pos(self) -> u32 {
        ((self.0 & MASK << X_SHIFT) >> X_SHIFT) as u32
    }

    pub fn get_y_pos(self) -> u32 {
        ((self.0 & MASK << Y_SHIFT) >> Y_SHIFT) as u32
    }

    pub fn get_left_button_down(self) -> bool {
        self.0 & 1 << LEFT_BUTTON_SHIFT != 0
    }

    pub fn get_right_button_down(self) -> bool {
        self.0 & 1 << RIGHT_BUTTON_SHIFT != 0
    }

    pub fn get_middle_button_down(self) -> bool {
        self.0 & 1 << MIDDLE_BUTTON_SHIFT != 0
    }

    pub fn get_wheel_up(self) -> bool {
        self.0 & 1 << WHEEL_UP != 0
    }

    pub fn get_wheel_down(self) -> bool {
        self.0 & 1 << WHEEL_DOWN != 0
    }

    pub fn get_wheel_left(self) -> bool {
        self.0 & 1 << WHEEL_LEFT != 0
    }

    pub fn get_wheel_right(self) -> bool {
        self.0 & 1 << WHEEL_RIGHT != 0
    }

    pub fn get_x_delta(self) -> i32 {
        ((self.0 & DELTA_MASK << X_DELTA_SHIFT) >> X_DELTA_SHIFT) as i16 as i32
    }

    pub fn get_y_delta(self) -> i32 {
        ((self.0 & DELTA_MASK << Y_DELTA_SHIFT) >> Y_DELTA_SHIFT) as i16 as i32
    }

    pub fn set_x_pos(&mut self, value: u32) {
        self.0 &= !(MASK << X_SHIFT);
        self.0 |= (value as u64) << X_SHIFT;
    }

    pub fn set_y_pos(&mut self, value: u32) {
        self.0 &= !(MASK << Y_SHIFT);
        self.0 |= (value as u64) << Y_SHIFT;
    }

    pub fn set_left_button(&mut self, value: bool) {
        let value = value as u64;
        self.0 &= !(value << LEFT_BUTTON_SHIFT);
        self.0 |= value << LEFT_BUTTON_SHIFT;
    }

    pub fn set_middle_button(&mut self, value: bool) {
        let value = value as u64;
        self.0 &= !(value << MIDDLE_BUTTON_SHIFT);
        self.0 |= value << MIDDLE_BUTTON_SHIFT;
    }

    pub fn set_right_button(&mut self, value: bool) {
        let value = value as u64;
        self.0 &= !(value << RIGHT_BUTTON_SHIFT);
        self.0 |= value << RIGHT_BUTTON_SHIFT;
    }

    pub fn set_wheel_up(&mut self, value: bool) {
        let value = value as u64;
        self.0 &= !(value << WHEEL_LEFT);
        self.0 |= value << WHEEL_LEFT;
    }

    pub fn set_wheel_down(&mut self, value: bool) {
        let value = value as u64;
        self.0 &= !(value << WHEEL_DOWN);
        self.0 |= value << WHEEL_DOWN;
    }

    pub fn set_wheel_left(&mut self, value: bool) {
        let value = value as u64;
        self.0 &= !(value << WHEEL_LEFT);
        self.0 |= value << WHEEL_LEFT;
    }

    pub fn set_wheel_right(&mut self, value: bool) {
        let value = value as u64;
        self.0 &= !(value << WHEEL_RIGHT);
        self.0 |= value << WHEEL_RIGHT;
    }

    pub fn set_x_delta(&mut self, value: i32) {
        self.0 &= !(DELTA_MASK << X_DELTA_SHIFT);
        self.0 |= (value as u32 as u64) << X_DELTA_SHIFT;
    }

    pub fn set_y_delta(&mut self, value: i32) {
        self.0 &= !(DELTA_MASK << Y_DELTA_SHIFT);
        self.0 |= (value as u32 as u64) << Y_DELTA_SHIFT;
    }
}

#[cfg(test)]
mod tests {
    use crate::MouseState;

    #[test]
    fn test_mouse_state() {
        let mut out = MouseState::default();
        let x = 1920 - 1;
        let y = 1080 - 1;
        out.set_x_pos(x);
        out.set_y_pos(y);

        assert_eq!(out.get_x_pos(), x);
        assert_eq!(out.get_y_pos(), y);

        assert_eq!(out.get_left_button_down(), false);
        assert_eq!(out.get_middle_button_down(), false);
        assert_eq!(out.get_right_button_down(), false);

        out.set_x_pos(0);
        out.set_y_pos(0);
        assert_eq!(out.get_x_pos(), 0);
        assert_eq!(out.get_y_pos(), 0);
        assert_eq!(out.0, 0);

        out.set_left_button(false);
        out.set_middle_button(false);
        out.set_right_button(false);
        assert_eq!(out.0, 0);

        out.set_left_button(true);
        out.set_middle_button(true);
        out.set_right_button(true);
        assert_eq!(out.get_left_button_down(), true);
        assert_eq!(out.get_middle_button_down(), true);
        assert_eq!(out.get_right_button_down(), true);

        out.set_x_pos(123);
        assert_eq!(out.get_x_pos(), 123);
    }

    #[test]
    fn test_mouse_state_deltas() {
        // Same as above, but with deltas
        let mut out = MouseState::default();
        let x = -1920;
        let y = -1080;
        out.set_x_delta(x);
        out.set_y_delta(y);

        assert_eq!(out.get_x_delta(), x);
        assert_eq!(out.get_y_delta(), y);

        assert_eq!(out.get_left_button_down(), false);
        assert_eq!(out.get_middle_button_down(), false);
        assert_eq!(out.get_right_button_down(), false);

        out.set_x_delta(0);
        out.set_y_delta(0);
        assert_eq!(out.get_x_pos(), 0);
        assert_eq!(out.get_y_pos(), 0);
        assert_eq!(out.0, 0);

        out.set_left_button(false);
        out.set_middle_button(false);
        out.set_right_button(false);
        assert_eq!(out.0, 0);

        out.set_left_button(true);
        out.set_middle_button(true);
        out.set_right_button(true);
        assert_eq!(out.get_left_button_down(), true);
        assert_eq!(out.get_middle_button_down(), true);
        assert_eq!(out.get_right_button_down(), true);

        out.set_x_delta(123);
        assert_eq!(out.get_x_delta(), 123);
    }
}