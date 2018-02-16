pub mod keys;
pub mod scancode;

use self::keys::PX8Key;
use self::scancode::{Scancode, Mod};

use std::collections::HashMap;

pub struct Mouse {
    pub x: i32,
    pub y: i32,
    pub state: u32,
    pub state_quick: u32,
    pub delay: f64,
}

impl Mouse {
    pub fn new() -> Mouse {
        Mouse {
            x: 0,
            y: 0,
            state: 0,
            state_quick: 0,
            delay: 0.,
        }
    }
}
pub struct PlayerKeys {
    frames: HashMap<PX8Key, f64>,
    keys: HashMap<PX8Key, bool>,
    keys_quick: HashMap<PX8Key, bool>,
}

impl PlayerKeys {
    pub fn new() -> PlayerKeys {
        let mut keys = HashMap::new();
        let mut keys_quick = HashMap::new();

        keys.insert(PX8Key::Down, false);
        keys.insert(PX8Key::Up, false);
        keys.insert(PX8Key::Left, false);
        keys.insert(PX8Key::Right, false);
        keys.insert(PX8Key::A, false);
        keys.insert(PX8Key::B, false);
        keys.insert(PX8Key::Enter, false);
        keys.insert(PX8Key::Pause, false);

        keys_quick.insert(PX8Key::Down, false);
        keys_quick.insert(PX8Key::Up, false);
        keys_quick.insert(PX8Key::Left, false);
        keys_quick.insert(PX8Key::Right, false);
        keys_quick.insert(PX8Key::A, false);
        keys_quick.insert(PX8Key::B, false);
        keys_quick.insert(PX8Key::Enter, false);
        keys_quick.insert(PX8Key::Pause, false);

        PlayerKeys {
            frames: HashMap::new(),
            keys: keys,
            keys_quick: keys_quick,
        }
    }
}

pub struct Players {
    pub pkeys: HashMap<u8, PlayerKeys>,
    pub mouse: Mouse,
    pub akeys: HashMap<Scancode, bool>,
    pub akeys_quick: HashMap<Scancode, bool>,
    pub all_frames: HashMap<Scancode, f64>,
    pub text: String,
    pub delta: f64,
}

impl Players {
    pub fn new() -> Players {
        let mut keys = HashMap::new();
        keys.insert(0, PlayerKeys::new());
        keys.insert(1, PlayerKeys::new());

        Players {
            pkeys: keys,
            mouse: Mouse::new(),
            akeys: HashMap::new(),
            akeys_quick: HashMap::new(),
            all_frames: HashMap::new(),
            text: "".to_string(),
            delta: 0.1,
        }
    }

    pub fn clear_text(&mut self) {
        self.text = "".to_string();
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn get_text(&mut self) -> String {
        self.text.clone()
    }

    pub fn set_mouse_x(&mut self, x: i32) {
        self.mouse.x = x;
    }

    pub fn set_mouse_y(&mut self, y: i32) {
        self.mouse.y = y;
    }

    pub fn mouse_button_down(&mut self, left: bool, right: bool, middle: bool, elapsed: f64) {
        self.mouse.state = 0;

        if left {
            self.mouse.state = 1;
        } else if right {
            self.mouse.state = 2;
        } else if middle {
            self.mouse.state = 4;
        }

        self.mouse.state_quick = self.mouse.state;
        self.mouse.delay = elapsed;
    }

    pub fn mouse_button_up(&mut self) {
        self.mouse.state = 0;
        self.mouse.state_quick = 0;
    }

    pub fn update(&mut self, elapsed: f64) {
        if elapsed - self.mouse.delay > self.delta {
            self.mouse.state = 0;
        }

        for (key_val, value) in self.akeys.iter_mut() {
            if *value {
                match self.all_frames.get(&key_val) {
                    Some(&delay_value) => {
                        if elapsed - delay_value >= self.delta {
                            self.akeys_quick.insert(*key_val, false);
                        } else {
                            self.akeys_quick.insert(*key_val, true);
                        }
                    }
                    _ => {
                        self.akeys_quick.insert(*key_val, true);
                    }
                }
            }
        }

        for (_, keys) in self.pkeys.iter_mut() {
            let ref mut current_keys = keys.keys;

            let mut modif_quick: HashMap<PX8Key, bool> = HashMap::new();

            for (key_val, value) in current_keys.iter_mut() {
                if *value {
                    match keys.frames.get(&key_val) {
                        Some(&delay_value) => {
                            if elapsed - delay_value >= self.delta {
                                modif_quick.insert(*key_val, false);
                            } else {
                                modif_quick.insert(*key_val, true);
                            }
                        }
                        _ => {
                            modif_quick.insert(*key_val, true);
                        }
                    }
                }
            }

            for (key_val, value) in modif_quick {
                keys.keys_quick.insert(key_val, value);
            }
        }
    }

    pub fn key_down(&mut self, keymod: Mod, scancode: Scancode, repeat: bool, elapsed: f64) {
        debug!("SCANCODE {:?} {:?} {:?} {:?} -> DOWN",
               keymod,
               scancode,
               repeat,
               elapsed);

        let mut scancode = scancode;

        if keymod == Mod::LCTRLMOD || keymod == Mod::RCTRLMOD || keymod == Mod::LGUIMOD ||
           keymod == Mod::RGUIMOD {
            if scancode == Scancode::C {
                scancode = Scancode::Copy;
            } else if scancode == Scancode::V {
                scancode = Scancode::Paste;
            } else if scancode == Scancode::X {
                scancode = Scancode::Cut;
            }
        }

        self.akeys.insert(scancode, true);
        self.akeys_quick.insert(scancode, true);

        self.all_frames.insert(scancode, elapsed);

        if let (Some(key), player) = self::keys::map_keycode(scancode) {
            self.key_down_direct(player, key, repeat, elapsed);
        }
    }

    pub fn key_down_direct(&mut self, player: u8, key: PX8Key, repeat: bool, elapsed: f64) {
        debug!("KEY {:?} {:?} {:?} Player {:?} -> DOWN",
               key,
               repeat,
               elapsed,
               player);

        match self.pkeys.get_mut(&player) {
            Some(keys) => {
                if !keys.keys[&key] {
                    keys.keys_quick.insert(key, true);
                }

                keys.keys.insert(key, true);
                if !repeat {
                    keys.frames.insert(key, elapsed);
                }
            }
            None => (),
        }
    }

    pub fn key_direc_hor_up(&mut self, player: u8) {
        match self.pkeys.get_mut(&player) {
            Some(keys) => {
                keys.keys.insert(PX8Key::Right, false);
                keys.keys.insert(PX8Key::Left, false);
            }
            None => (),
        }
    }

    pub fn key_direc_ver_up(&mut self, player: u8) {
        match self.pkeys.get_mut(&player) {
            Some(keys) => {
                keys.keys.insert(PX8Key::Up, false);
                keys.keys.insert(PX8Key::Down, false);
            }
            None => (),
        }
    }

    pub fn key_up(&mut self, keymod: Mod, scancode: Scancode) {
        debug!("SCANCODE {:?} UP", scancode);

        let mut scancode = scancode;

        if keymod == Mod::LCTRLMOD || keymod == Mod::RCTRLMOD || keymod == Mod::LGUIMOD ||
           keymod == Mod::RGUIMOD {
            if scancode == Scancode::C {
                scancode = Scancode::Copy;
            } else if scancode == Scancode::V {
                scancode = Scancode::Paste;
            } else if scancode == Scancode::X {
                scancode = Scancode::Cut;
            }
        }

        self.akeys.insert(scancode, false);
        self.akeys_quick.insert(scancode, false);

        if let (Some(key), player) = self::keys::map_keycode(scancode) {
            self.key_up_direct(player, key);
        }
    }

    pub fn key_up_direct(&mut self, player: u8, key: PX8Key) {
        debug!("KEY {:?} Player {:?} -> UP", key, player);

        match self.pkeys.get_mut(&player) {
            Some(keys) => {
                keys.keys.insert(key, false);
                keys.keys_quick.insert(key, false);
            }
            None => (),
        }
    }

    pub fn get_value(&self, player: u8, index: u8) -> u8 {
        match self.pkeys.get(&player) {
            Some(keys) => {
                match index {
                    0 if keys.keys[&PX8Key::Left] => 1,
                    1 if keys.keys[&PX8Key::Right] => 1,
                    2 if keys.keys[&PX8Key::Up] => 1,
                    3 if keys.keys[&PX8Key::Down] => 1,
                    4 if keys.keys[&PX8Key::A] => 1,
                    5 if keys.keys[&PX8Key::B] => 1,
                    6 if keys.keys[&PX8Key::Enter] => 1,
                    7 if keys.keys[&PX8Key::Pause] => 1,
                    _ => 0,
                }
            }
            None => 0,
        }
    }


    pub fn get_value_quick(&mut self, player: u8, index: u8) -> u8 {
        match self.pkeys.get(&player) {
            Some(keys) => {
                match index {
                    0 if keys.keys_quick[&PX8Key::Left] => 1,
                    1 if keys.keys_quick[&PX8Key::Right] => 1,
                    2 if keys.keys_quick[&PX8Key::Up] => 1,
                    3 if keys.keys_quick[&PX8Key::Down] => 1,
                    4 if keys.keys_quick[&PX8Key::A] => 1,
                    5 if keys.keys_quick[&PX8Key::B] => 1,
                    6 if keys.keys_quick[&PX8Key::Enter] => 1,
                    7 if keys.keys_quick[&PX8Key::Pause] => 1,
                    _ => 0,
                }
            }
            None => 0,
        }
    }

    pub fn btn(&mut self, player: u8, index: u8) -> bool {
        self.get_value(player, index) == 1
    }

    pub fn btn2(&mut self, c: i32) -> bool {
        // match Scancode::from_i32(c as i32) {
        // Some(scancode) => {
        // match self.akeys.get(&scancode) {
        // Some(v) => {
        // return *v;
        // }
        // None => (),
        // }
        // }
        // None => (),
        // }
        false
    }

    pub fn btn3(&mut self, scancode: Scancode) -> bool {
        match self.akeys.get(&scancode) {
            Some(v) => {
                return *v;
            }
            None => (),
        }
        false
    }

    pub fn btnp3(&mut self, scancode: Scancode) -> bool {
        match self.akeys_quick.get(&scancode) {
            Some(v) => {
                return *v;
            }
            None => (),
        }
        false
    }


    pub fn btnp(&mut self, player: u8, index: u8) -> bool {
        self.get_value_quick(player, index) == 1
    }

    pub fn btnp2(&mut self, c: i32) -> bool {
        // match Scancode::from_i32(c as i32) {
        // Some(scancode) => {
        // match self.akeys_quick.get(&scancode) {
        // Some(v) => {
        // return *v;
        // }
        // None => (),
        // }
        // }
        // None => (),
        // }
        false
    }

    pub fn mouse_coordinate(&mut self, index: u8) -> i32 {
        match index {
            0 => self.mouse.x,
            1 => self.mouse.y,
            _ => 0,
        }
    }

    pub fn mouse_state(&mut self) -> u32 {
        self.mouse.state
    }

    pub fn mouse_state_quick(&mut self) -> u32 {
        self.mouse.state_quick
    }
}