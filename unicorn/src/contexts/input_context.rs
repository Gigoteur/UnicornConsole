use paste::paste;

use crate::input::{ButtonCode, InputState};

use crate::input::PlayerInputEntry;

#[derive(Clone, Copy)]
/// A Raw Input State. Contains buttons and analog stick data.
pub struct RawInputState(pub i64);

impl RawInputState {
    fn is_valid(self) -> bool {
        self.0 & 1 << 63 == 0
    }
}

#[derive(Clone, Copy)]
/// A Raw Mouse state. Contains X/Y positions, deltas, and button states.
pub struct RawMouseState(pub i64);

impl RawMouseState {
    fn is_valid(self) -> bool {
        self.0 & 1 << 31 == 0
    }
}


macro_rules! derive_bind_input_api {
    (
        Buttons { $($btn_name:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
        Mouse {
            Buttons { $($mbtn_name:ident,)* },
            Axis { $($maxis_name:ident,)* },
            Wheel { $($mwheel_name:ident,)* },
         },
    ) => {
        paste! {
            pub trait InputApi {
                $(
                    fn [<button_ $btn_name _pressed>](&self, player_id: i32) -> i32;
                    fn [<button_ $btn_name _released>](&self, player_id: i32) -> i32;
                    fn [<button_ $btn_name _held>](&self, player_id: i32) -> i32;
                )*

                $(
                    fn [<analog_ $anlg_name _x>](&self, player_id: i32) -> f32;
                    fn [<analog_ $anlg_name _y>](&self, player_id: i32) -> f32;
                )*

                $(
                    fn [<trigger_ $trg_name>](&self, player_id: i32) -> f32;
                )*

                $(
                    fn [<mouse_ $mbtn_name _pressed>](&self, player_id: i32) -> i32;
                    fn [<mouse_ $mbtn_name _released>](&self, player_id: i32) -> i32;
                    fn [<mouse_ $mbtn_name _held>](&self, player_id: i32) -> i32;
                )*

                $(
                    fn [<mouse_ $maxis_name _pos>](&self, player_id: i32) -> i32;
                    fn [<mouse_ $maxis_name _delta>](&self, player_id: i32) -> i32;
                )*

                $(
                    fn [<mouse_wheel_ $mwheel_name>](&self, player_id: i32) -> i32;
                )*

                fn lock_mouse(&mut self, locked: i32);

                fn raw_input_state(&self, player_id: i32) -> i64;
                fn raw_mouse_state(&self, player_id: i32) -> i64;
            }

            pub trait InputApiBinding {
                $(
                    fn [<bind_button_ $btn_name _pressed>](&mut self);
                    fn [<bind_button_ $btn_name _released>](&mut self);
                    fn [<bind_button_ $btn_name _held>](&mut self);
                )*

                $(
                    fn [<bind_analog_ $anlg_name _x>](&mut self);
                    fn [<bind_analog_ $anlg_name _y>](&mut self);
                )*

                $(
                    fn [<bind_trigger_ $trg_name>](&mut self);
                )*

                $(
                    fn [<bind_mouse_ $mbtn_name _pressed>](&mut self);
                    fn [<bind_mouse_ $mbtn_name _released>](&mut self);
                    fn [<bind_mouse_ $mbtn_name _held>](&mut self);
                )*

                $(
                    fn [<bind_mouse_ $maxis_name _pos>](&mut self);
                    fn [<bind_mouse_ $maxis_name _delta>](&mut self);
                )*

                $(
                    fn [<bind_mouse_wheel_ $mwheel_name>](&mut self);
                )*

                fn bind_raw_input_state(&mut self);
                fn bind_raw_mouse_state(&mut self);
                fn bind_lock_mouse(&mut self);

                fn bind_input_api(&mut self) {
                    $(
                        self.[<bind_button_ $btn_name _pressed>]();
                        self.[<bind_button_ $btn_name _released>]();
                        self.[<bind_button_ $btn_name _held>]();
                    )*

                    $(
                        self.[<bind_analog_ $anlg_name _x>]();
                        self.[<bind_analog_ $anlg_name _y>]();
                    )*

                    $(
                        self.[<bind_trigger_ $trg_name>]();
                    )*

                    $(
                        self.[<bind_mouse_ $mbtn_name _pressed>]();
                        self.[<bind_mouse_ $mbtn_name _released>]();
                        self.[<bind_mouse_ $mbtn_name _held>]();
                    )*

                    $(
                        self.[<bind_mouse_ $maxis_name _pos>]();
                        self.[<bind_mouse_ $maxis_name _delta>]();
                    )*

                    $(
                        self.[<bind_mouse_wheel_ $mwheel_name>]();
                    )*

                    self.bind_lock_mouse();
                    self.bind_raw_input_state();
                    self.bind_raw_mouse_state();
                }
            }
        }
    };
}

derive_bind_input_api! {
    Buttons {
        a,
        b,
        c,
        d,
        up,
        down,
        left,
        right,
        start,
        select,
        left_shoulder,
        right_shoulder,
        left_stick,
        right_stick,
        left_trigger,
        right_trigger,
    },
    Analogs {
        left,
        right,
    },
    Triggers {
        left,
        right,
    },
    Mouse {
        Buttons {
            left,
            right,
            middle,
        },
        Axis {
            x,
            y,
        },
        Wheel {
            up,
            down,
            left,
            right,
        },
    },
}

#[derive(Clone)]
pub struct InputContext {
    pub input_entries: Box<[PlayerInputEntry]>,
    pub mouse_locked: bool,
}

impl InputContext {
    pub fn new(num_players: usize) -> Self {
        Self {
            input_entries: (0..num_players)
                .map(|_| PlayerInputEntry::default())
                .collect(),
            mouse_locked: false,
        }
    }

    pub fn btn(&mut self, player: u8, index: u8) -> bool {
        match index {
            0 if self.button_left_held(player as i32) == 1 => true,
            1 if self.button_right_held(player as i32) == 1 => true,
            2 if self.button_up_held(player as i32) == 1 => true,
            3 if self.button_down_held(player as i32) == 1 => true,
            4 if self.button_a_held(player as i32) == 1 => true,
            5 if self.button_b_held(player as i32) == 1 => true,
            6 if self.button_start_held(player as i32) == 1 => true,
            7 if self.button_select_held(player as i32) == 1 => true,
            8 if self.button_c_held(player as i32) == 1 => true,
            9 if self.button_d_held(player as i32) == 1 => true,
            _ => false,
        }
    }

    pub fn btnp(&mut self, player: u8, index: u8) -> bool {
        match index {
            0 if self.button_left_pressed(player as i32) == 1 => true,
            1 if self.button_right_pressed(player as i32) == 1 => true,
            2 if self.button_up_pressed(player as i32) == 1 => true,
            3 if self.button_down_pressed(player as i32) == 1 => true,
            4 if self.button_a_pressed(player as i32) == 1 => true,
            5 if self.button_b_pressed(player as i32) == 1 => true,
            6 if self.button_start_pressed(player as i32) == 1 => true,
            7 if self.button_select_pressed(player as i32) == 1 => true,
            8 if self.button_c_pressed(player as i32) == 1 => true,
            9 if self.button_d_pressed(player as i32) == 1 => true,
            _ => false,
        }
    }

    pub fn btn_mouse(&mut self, player: u8, index: u8) -> i32 {
        match index {
            0 => self.mouse_x_pos(player as i32),
            1 => self.mouse_y_pos(player as i32),
            _ => self.mouse_x_pos(player as i32),
        }
    }
}

/// This file automatically derives the various "get input" or "check input"
/// types of functions based on the macro at the bottom. This would otherwise be a
/// long and error prone process.

macro_rules! derive_generate_input_api {
    (
        Buttons { $($btn_name:ident: $btn_code:ident,)* },
        Analogs { $($anlg_name:ident,)* },
        Triggers { $($trg_name:ident,)* },
        Mouse {
            Buttons { $($mbtn_name:ident,)* },
            Axis { $($maxis_name:ident,)* },
            Wheel { $($mwheel_name:ident,)* },
         },
    ) => {
        paste! {
            impl InputApi for InputContext {
                $(
                    fn [<button_ $btn_name _pressed>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous.get_button_state(ButtonCode::$btn_code);
                            let curr = player_input.current.buttons.get_button_state(ButtonCode::$btn_code);
                            (prev == false && curr == true) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<button_ $btn_name _released>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous.get_button_state(ButtonCode::$btn_code);
                            let curr = player_input.current.buttons.get_button_state(ButtonCode::$btn_code);
                            (prev == true && curr == false) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<button_ $btn_name _held>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.buttons.get_button_state(ButtonCode::$btn_code) as i32
                        } else {
                            -1
                        }
                    }
                )*

                $(
                    fn [<analog_ $anlg_name _x>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$anlg_name _stick>].get_x_axis()
                        } else {
                            f32::NAN
                        }
                    }

                    fn [<analog_ $anlg_name _y>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$anlg_name _stick>].get_y_axis()
                        } else {
                            f32::NAN
                        }
                    }
                )*

                $(
                    fn [<trigger_ $trg_name>](&self, player_id: i32) -> f32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current.[<$trg_name _trigger>].get_value()
                        } else {
                            f32::NAN
                        }
                    }
                )*

                $(
                    fn [<mouse_ $mbtn_name _pressed>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous_mouse.[<get_ $mbtn_name _button_down>]();
                            let curr = player_input.current_mouse.[<get_ $mbtn_name _button_down>]();
                            (prev == false && curr == true) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $mbtn_name _released>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            let prev = player_input.previous_mouse.[<get_ $mbtn_name _button_down>]();
                            let curr = player_input.current_mouse.[<get_ $mbtn_name _button_down>]();
                            (prev == true && curr == false) as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $mbtn_name _held>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $mbtn_name _button_down>]() as i32
                        } else {
                            -1
                        }
                    }
                )*

                $(
                    fn [<mouse_ $maxis_name _pos>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $maxis_name _pos>]() as i32
                        } else {
                            -1
                        }
                    }

                    fn [<mouse_ $maxis_name _delta>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_ $maxis_name _delta>]() as i32
                        } else {
                            i32::MIN
                        }
                    }
                )*

                $(
                    fn [<mouse_wheel_ $mwheel_name>](&self, player_id: i32) -> i32 {
                        if let Some(player_input) = &self.input_entries.get(player_id as usize) {
                            player_input.current_mouse.[<get_wheel_ $mwheel_name>]() as i32
                        } else {
                            -1
                        }
                    }
                )*

                fn raw_mouse_state(&self, player_id: i32) -> i64 {
                    if let Some(player_input) = self.input_entries.get(player_id as usize) {
                        player_input.current_mouse.state as i64
                    } else {
                        1 << crate::input::MOUSE_INVALID_BIT
                    }
                }

                fn raw_input_state(&self, player_id: i32) -> i64 {
                    let state = if let Some(player_input) = self.input_entries.get(player_id as usize) {
                        player_input.current
                    } else {
                        InputState::INVALID_STATE
                    };

                    unsafe { std::mem::transmute(state) }
                }

                fn lock_mouse(&mut self, locked: i32) {
                    if locked != 0 {
                        self.mouse_locked = true
                    } else {
                        self.mouse_locked = false
                    };
                }
            }
        }
    }
}

derive_generate_input_api! {
    Buttons {
        a: ButtonA,
        b: ButtonB,
        c: ButtonC,
        d: ButtonD,
        up: Up,
        down: Down,
        left: Left,
        right: Right,
        start: Start,
        select: Select,
        left_shoulder: LeftShoulder,
        right_shoulder: RightShoulder,
        left_stick: LeftStick,
        right_stick: RightStick,
        left_trigger: LeftTrigger,
        right_trigger: RightTrigger,
    },
    Analogs {
        left,
        right,
    },
    Triggers {
        left,
        right,
    },
    Mouse {
        Buttons {
            left,
            right,
            middle,
        },
        Axis {
            x,
            y,
        },
        Wheel {
            up,
            down,
            left,
            right,
        },
    },
}