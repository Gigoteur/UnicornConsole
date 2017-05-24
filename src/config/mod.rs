pub mod keys;
pub mod controllers;

use self::keys::PX8Key;
use sdl2::mouse::MouseButton;
use sdl2::keyboard::Keycode;
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
        keys.insert(PX8Key::O, false);
        keys.insert(PX8Key::X, false);
        keys.insert(PX8Key::Enter, false);
        keys.insert(PX8Key::Pause, false);

        keys_quick.insert(PX8Key::Down, false);
        keys_quick.insert(PX8Key::Up, false);
        keys_quick.insert(PX8Key::Left, false);
        keys_quick.insert(PX8Key::Right, false);
        keys_quick.insert(PX8Key::O, false);
        keys_quick.insert(PX8Key::X, false);
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
    pub akeys: HashMap<Keycode, bool>,
    pub akeys_quick: HashMap<Keycode, bool>,
    pub all_frames: HashMap<Keycode, f64>,
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
        }
    }

    pub fn set_mouse_x(&mut self, x: i32) {
        self.mouse.x = x;
    }

    pub fn set_mouse_y(&mut self, y: i32) {
        self.mouse.y = y;
    }

    pub fn mouse_button_down(&mut self, mouse_btn: MouseButton, elapsed: f64) {
        self.mouse.state = match mouse_btn {
            MouseButton::Left => 1,
            MouseButton::Right => 2,
            MouseButton::Middle => 4,
            _ => 0,
        };
        self.mouse.state_quick = self.mouse.state;
        self.mouse.delay = elapsed;
    }

    pub fn mouse_button_up(&mut self, _mouse_btn: MouseButton, _elapsed: f64) {
        self.mouse.state = 0;
        self.mouse.state_quick = 0;
    }

    pub fn update(&mut self, elapsed: f64) {
        if elapsed - self.mouse.delay > 0.01 {
            self.mouse.state = 0;
        }

        for (key_val, value) in self.akeys.iter_mut() {
            if *value {
                match self.all_frames.get(&key_val) {
                    Some(&delay_value) => {
                        if elapsed - delay_value >= 0.01 {
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
                            if elapsed - delay_value >= 0.01 {
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

    pub fn key_down(&mut self, keycode: Keycode, repeat: bool, elapsed: f64) {
        debug!("KEY {:?} {:?} {:?} -> DOWN", keycode, repeat, elapsed);

        if self.akeys.contains_key(&keycode) {
            if !self.akeys[&keycode] {
                self.akeys_quick.insert(keycode, true);
            }
        }
        self.akeys.insert(keycode, true);
        self.all_frames.insert(keycode, elapsed);

        if let (Some(key), player) = self::keys::map_keycode(keycode) {
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

    pub fn key_up(&mut self, keycode: Keycode) {
        debug!("KEYCODE {:?} UP", keycode);

        self.akeys.insert(keycode, false);
        self.akeys_quick.insert(keycode, false);

        if let (Some(key), player) = self::keys::map_keycode(keycode) {
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
                    4 if keys.keys[&PX8Key::O] => 1,
                    5 if keys.keys[&PX8Key::X] => 1,
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
                    4 if keys.keys_quick[&PX8Key::O] => 1,
                    5 if keys.keys_quick[&PX8Key::X] => 1,
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
        match Keycode::from_i32(c as i32) {
            Some(keycode) => {
                match self.akeys.get(&keycode) {
                    Some(v) => {
                        return *v;
                    }
                    None => (),
                }
            }
            None => (),
        }
        false
    }

    pub fn btnp(&mut self, player: u8, index: u8) -> bool {
        self.get_value_quick(player, index) == 1
    }

    pub fn btnp2(&mut self, c: i32) -> bool {
        match Keycode::from_i32(c as i32) {
            Some(keycode) => {
                match self.akeys_quick.get(&keycode) {
                    Some(v) => {
                        return *v;
                    }
                    None => (),
                }
            }
            None => (),
        }
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
