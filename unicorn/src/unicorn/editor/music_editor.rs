
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use unicorn::editor::State;
use unicorn::{UnicornCartridge, UnicornConfig};
use unicorn::editor::Widget;
use unicorn::editor::point_in_rect;
use gfx::Screen;
use config::Players;
use config::scancode::Scancode;

use sound::sound::{SoundInternal, Sound};
use unicorn::editor::{Button, ButtonSlider};
use chiptune::chiptune;

static KEYS_NOTE: [Scancode; 29] = [
    Scancode::Z,
    Scancode::S,
    Scancode::X,
    Scancode::D,
    Scancode::C,
    Scancode::V,
    Scancode::G,
    Scancode::B,
    Scancode::H,
    Scancode::N,
    Scancode::J,
    Scancode::M,
    Scancode::Q,
    Scancode::Num2,
    Scancode::W,
    Scancode::Num3,
    Scancode::E,
    Scancode::R,
    Scancode::Num5,
    Scancode::T,
    Scancode::Num6,
    Scancode::Y,
    Scancode::Num7,
    Scancode::U,
    Scancode::I,
    Scancode::Num9,
    Scancode::O,
    Scancode::Num0,
    Scancode::P,
  //  'z', 's', 'x', 'd', 'c', 'v', 'g', 'b', 'h', 'n', 'j', 'm',
  //  'q', '2', 'w', '3', 'e', 'r', '5', 't', '6', 'y', '7', 'u',
  //  'i', '9', 'o', '0', 'p',
];

static KEYS_HEXA: [Scancode; 16] = [
    Scancode::Num0,
    Scancode::Num1,
    Scancode::Num2,
    Scancode::Num3,
    Scancode::Num4,
    Scancode::Num5,
    Scancode::Num6,
    Scancode::Num7,
    Scancode::Num8,
    Scancode::Num9,
    Scancode::A,
    Scancode::B,
    Scancode::C,
    Scancode::D,
    Scancode::E,
    Scancode::F,
];

pub struct SFX {
    pub programs: Vec<i32>,
    pub values: Vec<String>,
}

impl SFX {
    pub fn new() -> SFX {
        SFX {
            programs: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        self.programs.clear();
        self.values.clear();
    }
}

pub struct Touch {
    rects: Vec<i32>,
    color: i32,
    color_activated: i32,
    active: bool,
    active2: bool,
}

impl Touch {
    pub fn new(rects: Vec<i32>, offset_x: i32, offset_y: i32, color: i32, color_activated: i32) -> Touch {
        let mut rects_new = Vec::new();
        let mut i = 0;
        for r in &rects {
            if i % 2 == 0 {
                rects_new.push(r+offset_x);
            } else {
                rects_new.push(r+offset_y);
            }

            i += 1;
        }

        Touch {
            rects: rects_new.clone(),
            color: color,
            color_activated: color_activated,
            active: false,
            active2: false,
        }
    }

    pub fn is_click(&mut self, mouse_x: i32, mouse_y: i32) -> bool {
        let mut i = 0;
        while i < self.rects.len() {
            let x = self.rects[i];
            let y = self.rects[i+1];
            let x1 = self.rects[i+2];
            let y1 = self.rects[i+3];

            if point_in_rect(mouse_x, mouse_y, x, y, x1, y1) {
                return true;
            }
            i += 4;
        }
        false
    }
}

pub struct PianoKeyboard {
    touches: HashMap<Scancode, Touch>,
}

impl PianoKeyboard {
    pub fn new() -> PianoKeyboard {
        PianoKeyboard {
            touches: HashMap::new(),
        }
    }

    pub fn add(&mut self, rects: Vec<i32>, color: i32, color_activated: i32, offset_x: i32, offset_y: i32, key: Scancode) {
        self.touches.insert(key, Touch::new(rects.clone(), offset_x, offset_y, color, color_activated));
    }

    pub fn lock(&mut self, key: Scancode) {
        if let Some(touch) = self.touches.get_mut(&key) {
            touch.active = true;
        }
    }

    pub fn release(&mut self, key: Scancode) {
        if let Some(touch) = self.touches.get_mut(&key) {
            touch.active = false;
        }
    }


    pub fn is_active2(&mut self, key: Scancode) -> bool {
        if let Some(touch) = self.touches.get(&key) {
            return touch.active2
        }
        false
    }

    pub fn is_active(&mut self, key: Scancode) -> bool {
        if let Some(touch) = self.touches.get(&key) {
            return touch.active || touch.active2
        }
        false
    }

    pub fn update(&mut self, mouse_state: u32, mouse_x: i32, mouse_y: i32, players: Arc<Mutex<Players>>) {
        if mouse_state == 1 {
            for (key, touch) in self.touches.iter_mut() {
                if touch.is_click(mouse_x, mouse_y) {
                    touch.active2 = true;
                } else {
                    touch.active2 = false;
                }
            }
        } else {
            for (key, touch) in self.touches.iter_mut() {
                touch.active2 = false;
            }


        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        for (_, touch) in &self.touches {
            let mut color = touch.color;
            if touch.active || touch.active2 {
                color = touch.color_activated;

            } 
            let mut i = 0;
            while i < touch.rects.len() {
                let x = touch.rects[i];
                let y = touch.rects[i+1];
                let x1 = touch.rects[i+2];
                let y1 = touch.rects[i+3];
                
                screen.rectfill(x, y, x1, y1, color);

                i += 4;
            }
        }
    }
}

pub struct SFXFlag {
    x: i32,
    y: i32,
    rects: Vec<i32>,
    text_color: i32,
    background_color: i32,
    background_click_color: i32,
    active: bool,
}

impl SFXFlag {
    pub fn new(x: i32, y: i32, rects: Vec<i32>, text_color: i32, background_color: i32, background_click_color: i32) -> SFXFlag {
        SFXFlag {
            x: x,
            y: y,
            rects: rects,
            text_color: text_color,
            background_color: background_color,
            background_click_color: background_click_color,
            active: false,
        }
    }

    pub fn is_click(&mut self, mouse_x: i32, mouse_y: i32) -> bool {
        let mut i = 0;
        while i < self.rects.len() {
            let x = self.rects[i];
            let y = self.rects[i+1];
            let x1 = self.rects[i+2];
            let y1 = self.rects[i+3];

            if point_in_rect(mouse_x, mouse_y, x, y, x1, y1) {
                return true;
            }
            i += 4;
        }
        false
    }

}

pub struct SFXFlags {
    flags: HashMap<String, SFXFlag>,
}

impl SFXFlags {
    pub fn new() -> SFXFlags {
        SFXFlags {
            flags: HashMap::new(),
        }
    }

    pub fn add(&mut self, text: String, x: i32, y: i32, rects: Vec<i32>, text_color: i32, background_color: i32, background_click_color: i32) {
        self.flags.insert(text, SFXFlag::new(x, y, rects, text_color, background_color, background_click_color));
    }

    pub fn update_flag(&mut self, text: String, value: bool) {
        if let Some(flag) = self.flags.get_mut(&text) {
            flag.active = value;
        }
    }

    pub fn is_active(&mut self, text: String, mouse_x: i32, mouse_y: i32) -> bool {
        if let Some(flag) = self.flags.get_mut(&text) {
            if flag.is_click(mouse_x, mouse_y) {
                flag.active = !flag.active;
                return true;
            }
        }
        false
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        for (key, flag) in &self.flags {
            let mut i = 0;
            while i < flag.rects.len() {
                let x = flag.rects[i];
                let y = flag.rects[i+1];
                let x1 = flag.rects[i+2];
                let y1 = flag.rects[i+3];
                
                if flag.active {
                    screen.rectfill(x, y, x1, y1, flag.background_click_color);
                } else {
                    screen.rectfill(x, y, x1, y1, flag.background_color);
                }

                i += 4;
            }

            screen.print(key.clone(), flag.x, flag.y, flag.text_color);
        }
    }
}

pub struct ProgamValueUnit {
    x: i32,
    y : i32,
    value: String,
    higlight: bool,
    modified: bool,
}

impl ProgamValueUnit {
    pub fn new(x: i32, y: i32, value: String) -> ProgamValueUnit {
        ProgamValueUnit {
            x: x,
            y: y,
            value: value,
            higlight: false,
            modified: false,
        }
    }
    
    pub fn set_value(&mut self, value: String) {
        self.value = value;
    }

    pub fn set(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }

    pub fn update(&mut self, mouse_state: u32, mouse_x: i32, mouse_y: i32, players: Arc<Mutex<Players>>) {
        self.modified = false;

        if point_in_rect(mouse_x, mouse_y, self.x-1, self.y-1, self.x+5, self.y+7) {
            self.higlight = true;
        } else {
            self.higlight = false;
        }

        if self.higlight {
            for key in KEYS_HEXA.iter() {
                let key = *key;
                if players.lock().unwrap().btn3(key) {
                    self.value = format!("{}", key);
                    self.modified = true;
                }
            }
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        if self.higlight {
            screen.rect(self.x-1, self.y-1, self.x+5, self.y+7, 10);
        } else {
            screen.rect(self.x-1, self.y-1, self.x+5, self.y+7, 7);
        }
        screen.print(self.value.clone(), self.x, self.y, 7);
    }
}

pub struct ProgramValue {
    x: i32,
    y: i32,
    values: Vec<ProgamValueUnit>,
}

impl ProgramValue {
    pub fn new(x: i32, y: i32) -> ProgramValue {
        ProgramValue {
            x: x,
            y: y,
            values: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        self.values.push(ProgamValueUnit::new(self.x, self.y, "F".to_string()));
        self.values.push(ProgamValueUnit::new(self.x+8, self.y, "F".to_string()));
        self.values.push(ProgamValueUnit::new(self.x+16, self.y, "F".to_string()));
        self.values.push(ProgamValueUnit::new(self.x+24, self.y, "E".to_string()));
    }

    pub fn higlight(&mut self) -> bool {
        let value = &self.values[0];
        let value_1 = &self.values[1];
        let value_2 = &self.values[2];
        let value_3 = &self.values[3];

        value.higlight || value_1.higlight || value_2.higlight || value_3.higlight
    }

    pub fn set(&mut self, value: u16) {
        self.values[0].set_value(format!("{:01X}", (value & 0xF000) >> 12));
        self.values[1].set_value(format!("{:01X}", (value & 0x0F00) >> 8));
        self.values[2].set_value(format!("{:01X}", (value & 0x00F0) >> 4));
        self.values[3].set_value(format!("{:01X}", (value & 0x000F)));
    }

    pub fn get(&mut self) -> u16 {
        let value = &self.values[0];
        let value_1 = &self.values[1];
        let value_2 = &self.values[2];
        let value_3 = &self.values[3];

        if value.higlight || value_1.higlight || value_2.higlight || value_3.higlight {
            let str_value = format!("{}{}{}{}", value.value, value_1.value, value_2.value, value_3.value);
            return u16::from_str_radix(&str_value, 16).unwrap();
        }

        0xFFFE
    }

    pub fn update(&mut self, mouse_state: u32, mouse_x: i32, mouse_y: i32, players: Arc<Mutex<Players>>) {
        for value in self.values.iter_mut() {
            value.update(mouse_state, mouse_x, mouse_y, players.clone());
        }

        if self.values[0].modified || self.values[1].modified || self.values[2].modified || self.values[3].modified {
            if self.values[0].modified && self.values[1].value == "F" && self.values[2].value == "F" && self.values[3].value == "E" {
                self.values[1].set_value("0".to_string());
                self.values[2].set_value("0".to_string());
                self.values[3].set_value("0".to_string());
            }
            if self.values[1].modified && self.values[0].value == "F" && self.values[2].value == "F" && self.values[3].value == "E" {
                self.values[0].set_value("0".to_string());
                self.values[2].set_value("0".to_string());
                self.values[3].set_value("0".to_string());
            }
            if self.values[2].modified && self.values[0].value == "F" && self.values[1].value == "F" && self.values[3].value == "E" {
                self.values[0].set_value("0".to_string());
                self.values[1].set_value("0".to_string());
                self.values[3].set_value("0".to_string());
            }
            if self.values[3].modified && self.values[0].value == "F" && self.values[1].value == "F" && self.values[2].value == "F" {
                self.values[0].set_value("0".to_string());
                self.values[1].set_value("0".to_string());
                self.values[2].set_value("0".to_string());
            }
        }
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        for value in self.values.iter_mut() {
            value.draw(screen);
        }
    }
}

pub struct TrackEditor {
    idx: i32,
}

impl TrackEditor {
    pub fn new(state: Arc<Mutex<State>>) -> TrackEditor {
        TrackEditor {
            idx: -1,
        }
    }

    pub fn init(&mut self, screen: &mut Screen) {
        info!("[EDITOR][MUSIC][TRACK] Init");
    }
    
    pub fn update(&mut self, cartridge: &mut UnicornCartridge, players: Arc<Mutex<Players>>, sound_internal: Arc<Mutex<SoundInternal>>, sound: Arc<Mutex<Sound>>) -> bool {
        let sound_internal = sound_internal.lock().unwrap();

        if cartridge.music_track.len() == 0 {
            info!("[EDITOR][MUSIC][TRACK] Create new song");
            
           // let idx = sound_internal.new_music(cartridge, "default".to_string());
           // if idx >= 0 {
           //     self.idx = idx;
           // }
        }

        true
    }
    
    pub fn draw(&mut self, screen: &mut Screen) {
    }
}

pub struct SFXEditor {
    idx_sfx: u32,
    num_sfx: u32,
    name: String,
    base_note: ButtonSlider,
    attack: ButtonSlider,
    decay: ButtonSlider,
    pi_key: PianoKeyboard,
    sfx: SFX,
    sfx_channels_keys: HashMap<Scancode, i32>,
    current_sfx_positions: HashMap<u32, usize>,
    selected_sounds: String,
    flags: SFXFlags,
    selected_program: i32,
    p_values: Vec<ProgramValue>,
    next: Button,
    prev: Button,
}

impl SFXEditor {
    pub fn new(state: Arc<Mutex<State>>) -> SFXEditor {
        SFXEditor {
            idx_sfx: 0,
            num_sfx: 0,
            name: "".to_string(),
            sfx: SFX::new(),
            current_sfx_positions: HashMap::new(),
            sfx_channels_keys: HashMap::new(),
            pi_key: PianoKeyboard::new(),
            base_note: ButtonSlider::new("BASE".to_string(), "C-4".to_string(), 0, 40, 7, 6, 5),
            attack: ButtonSlider::new("ATK".to_string(), "00".to_string(), 56, 40, 7, 6, 5),
            decay: ButtonSlider::new("DEC".to_string(), "00".to_string(), 104, 40, 7, 6, 5),
            selected_sounds: "".to_string(),
            flags: SFXFlags::new(),
            selected_program: -1,
            p_values: Vec::new(),
            prev: Button::new(0, 16, 20, 24, 5, "PREV".to_string(), false),
            next: Button::new(22, 16, 42, 24, 5, "NEXT".to_string(), false),
        }
    }

    pub fn init(&mut self, screen: &mut Screen) {
        info!("[EDITOR][MUSIC][SFX] Init");

        self.flags.add("DRUM".to_string(), 0, 32, vec![0, 32, 20, 40], 7, 8, 11);
        self.flags.add("PUL".to_string(), 22, 32, vec![22, 32, 38, 40], 7, 8, 11);
        self.flags.add("NOISE".to_string(), 40, 32, vec![40, 32, 60, 40], 7, 8, 11);
        self.flags.add("TRI".to_string(), 62, 32, vec![62, 32, 74, 40], 7, 8, 11);
        self.flags.add("VIB".to_string(), 76, 32, vec![76, 32, 88, 40], 7, 8, 11);
        self.flags.add("SAW".to_string(), 90, 32, vec![90, 32, 102, 40], 7, 8, 11);
        self.flags.add("METAL".to_string(), 104, 32, vec![104, 32, 124, 40], 7, 8, 11);

        // White -> 18
        // Black -> 10
        let offset_y = 0;
        let offset_x = 110;

        self.pi_key.add(vec![0, 180, 12, 208, 0, 208, 18, 226], 7, 8, offset_x, offset_y, Scancode::Z);
        self.pi_key.add(vec![25, 180, 33, 208, 20, 208, 38, 226], 7, 8, offset_x, offset_y, Scancode::X);
        self.pi_key.add(vec![45, 180, 58, 208, 40, 208, 58, 226], 7, 8, offset_x, offset_y, Scancode::C);

        self.pi_key.add(vec![13, 180, 23, 207], 0, 1, offset_x, offset_y, Scancode::S);
        self.pi_key.add(vec![34, 180, 44, 207], 0, 1, offset_x, offset_y, Scancode::D);

        self.pi_key.add(vec![60, 180, 72, 208, 60, 208, 78, 226], 7, 8, offset_x, offset_y, Scancode::V);
        self.pi_key.add(vec![85, 180, 93, 208, 80, 208, 98, 226], 7, 8, offset_x, offset_y, Scancode::B);
        self.pi_key.add(vec![105, 180, 113, 208, 100, 208, 118, 226], 7, 8, offset_x, offset_y, Scancode::N);
        self.pi_key.add(vec![125, 180, 138, 208, 120, 208, 138, 226], 7, 8, offset_x, offset_y, Scancode::M);

        self.pi_key.add(vec![73, 180, 83, 207], 0, 1, offset_x, offset_y, Scancode::G);
        self.pi_key.add(vec![94, 180, 104, 207], 0, 1, offset_x, offset_y, Scancode::H);
        self.pi_key.add(vec![114, 180, 124, 207], 0, 1, offset_x, offset_y, Scancode::J);


        let offset_x = 79;
        let offset_y = -50;
        self.pi_key.add(vec![0, 180, 12, 208, 0, 208, 18, 226], 7, 8, offset_x, offset_y, Scancode::Q);
        self.pi_key.add(vec![25, 180, 33, 208, 20, 208, 38, 226], 7, 8, offset_x, offset_y, Scancode::W);
        self.pi_key.add(vec![45, 180, 58, 208, 40, 208, 58, 226], 7, 8, offset_x, offset_y, Scancode::E);

        self.pi_key.add(vec![13, 180, 23, 207], 0, 1, offset_x, offset_y, Scancode::Num2);
        self.pi_key.add(vec![34, 180, 44, 207], 0, 1, offset_x, offset_y, Scancode::Num3);

        self.pi_key.add(vec![60, 180, 72, 208, 60, 208, 78, 226], 7, 8, offset_x, offset_y, Scancode::R);
        self.pi_key.add(vec![85, 180, 93, 208, 80, 208, 98, 226], 7, 8, offset_x, offset_y, Scancode::T);
        self.pi_key.add(vec![105, 180, 113, 208, 100, 208, 118, 226], 7, 8, offset_x, offset_y, Scancode::Y);
        self.pi_key.add(vec![125, 180, 138, 208, 120, 208, 138, 226], 7, 8, offset_x, offset_y, Scancode::U);

        self.pi_key.add(vec![73, 180, 83, 207], 0, 1, offset_x, offset_y, Scancode::Num5);
        self.pi_key.add(vec![94, 180, 104, 207], 0, 1, offset_x, offset_y, Scancode::Num6);
        self.pi_key.add(vec![114, 180, 124, 207], 0, 1, offset_x, offset_y, Scancode::Num7);


        let offset_x = 219;
        let offset_y = -50;
        self.pi_key.add(vec![0, 180, 12, 208, 0, 208, 18, 226], 7, 8, offset_x, offset_y, Scancode::I);
        self.pi_key.add(vec![25, 180, 33, 208, 20, 208, 38, 226], 7, 8, offset_x, offset_y, Scancode::O);
        self.pi_key.add(vec![45, 180, 58, 208, 40, 208, 58, 226], 7, 8, offset_x, offset_y, Scancode::P);

        self.pi_key.add(vec![13, 180, 23, 207], 0, 1, offset_x, offset_y, Scancode::Num9);
        self.pi_key.add(vec![34, 180, 44, 207], 0, 1, offset_x, offset_y, Scancode::Num0);

        let mut idx_x = 4;
        let mut idx_y = 55;

        if self.p_values.len() == 0 {
            for idx in 0..32 {
                if idx > 0 && idx % 8 == 0 {
                    idx_x += 88;
                    idx_y = 55;
                }

                self.p_values.push(ProgramValue::new(idx_x+10, idx_y));
                
                idx_y += 9;
            }

            for value in self.p_values.iter_mut() {
                value.init();
            }
        }
    }

    pub fn update(&mut self, cartridge: &mut UnicornCartridge, players: Arc<Mutex<Players>>, sound_internal: Arc<Mutex<SoundInternal>>, sound: Arc<Mutex<Sound>>) -> bool {
        let mouse_state_quick = players.lock().unwrap().mouse_state_quick();
        let mouse_state = players.lock().unwrap().mouse_state();

        let mouse_x = players.lock().unwrap().mouse_coordinate(0);
        let mouse_y = players.lock().unwrap().mouse_coordinate(1);

        let mut sound_internal = sound_internal.lock().unwrap();

        if cartridge.sound_tracks.len() == 0 {
            info!("[EDITOR][MUSIC][SFX] Create new sound");
            
            let idx = sound_internal.new_sfx(cartridge, "default".to_string());
            if idx >= 0 {
                self.idx_sfx = idx as u32;
            }
        }

        let current_sfx = *cartridge.sound_tracks.get_mut(&cartridge.sound_tracks_name[self.idx_sfx as usize]).unwrap();

        self.name = cartridge.sound_tracks_name[self.idx_sfx as usize].clone();
        self.num_sfx = cartridge.sound_tracks.len() as u32;

        if mouse_state == 1 {
            self.next.update(mouse_x, mouse_y);
            self.prev.update(mouse_x, mouse_y);

            if self.next.is_click() {
                self.idx_sfx = (self.idx_sfx + 1) % self.num_sfx;
            }

            if self.prev.is_click() {
                self.idx_sfx = (self.idx_sfx - 1) % self.num_sfx;
            }
        }


        for value in self.p_values.iter_mut() {
            value.update(mouse_state, mouse_x, mouse_y, players.clone());
        }

        let mut define_value = false;

        let mut idx_prog = 0;
        for value in self.p_values.iter_mut() {
            if value.higlight() {
                sound_internal.player.set_sound_program(current_sfx, value.get(), idx_prog);
                define_value = true;
            }
            idx_prog += 1;
        }

        /* BASE NOTE */
        self.base_note.update(mouse_state, mouse_x, mouse_y, players.clone());
        if self.base_note.is_minus_click() {
            let base_note = sound_internal.player.get_base_note(current_sfx);
            if base_note != 0 {
                sound_internal.player.set_base_note(current_sfx, base_note-1);
            }
        }

        if self.base_note.is_plus_click() {
            let base_note = sound_internal.player.get_base_note(current_sfx);
            if base_note != 95 {
                sound_internal.player.set_base_note(current_sfx, base_note+1);
            }
        }
        self.base_note.update_value(chiptune::base_note_name(sound_internal.player.get_base_note(current_sfx)).unwrap());

        /* ATTACK */
        self.attack.update(mouse_state, mouse_x, mouse_y, players.clone());
        let attack = sound_internal.player.get_attack(current_sfx);

        if self.attack.is_minus_click() {
            if attack != 0 {
                sound_internal.player.set_attack(current_sfx, attack-1);
            }
        }

        if self.attack.is_plus_click() {
            if attack != 0x3f {
                sound_internal.player.set_attack(current_sfx, attack+1);
            }
        }
        self.attack.update_value(format!("{:02X}", sound_internal.player.get_attack(current_sfx)));

        /* DECAY */
        self.decay.update(mouse_state, mouse_x, mouse_y, players.clone());
        let decay = sound_internal.player.get_decay(current_sfx);

        if self.decay.is_minus_click() {
            if decay != 0 {
                sound_internal.player.set_decay(current_sfx, decay-1);
            }
        }

        if self.decay.is_plus_click() {
            if decay != 0x3f {
                sound_internal.player.set_decay(current_sfx, decay+1);
            }
        }
        self.decay.update_value(format!("{:02X}", sound_internal.player.get_decay(current_sfx)));

        self.pi_key.update(mouse_state_quick, mouse_x, mouse_y, players.clone());

        self.sfx.reset();

        self.flags.update_flag("DRUM".to_string(), sound_internal.player.get_drum(current_sfx));
        self.flags.update_flag("PUL".to_string(), sound_internal.player.get_pulse(current_sfx));
        self.flags.update_flag("NOISE".to_string(), sound_internal.player.get_noise(current_sfx));
        self.flags.update_flag("TRI".to_string(), sound_internal.player.get_tri(current_sfx));
        self.flags.update_flag("VIB".to_string(), sound_internal.player.get_vib(current_sfx));
        self.flags.update_flag("SAW".to_string(), sound_internal.player.get_saw(current_sfx));
        self.flags.update_flag("METAL".to_string(), sound_internal.player.get_metal(current_sfx));

        if mouse_state == 1 {
            if self.flags.is_active("DRUM".to_string(), mouse_x, mouse_y) {
                sound_internal.player.set_drum(current_sfx);
            }
            if self.flags.is_active("PUL".to_string(), mouse_x, mouse_y) {
                sound_internal.player.set_pulse(current_sfx);
            }
            if self.flags.is_active("SAW".to_string(), mouse_x, mouse_y) {
                sound_internal.player.set_saw(current_sfx);
            }
            if self.flags.is_active("NOISE".to_string(), mouse_x, mouse_y) {
                sound_internal.player.set_noise(current_sfx);
            }
            if self.flags.is_active("TRI".to_string(), mouse_x, mouse_y) {
                sound_internal.player.set_tri(current_sfx);
            }
            if self.flags.is_active("VIB".to_string(), mouse_x, mouse_y) {
                sound_internal.player.set_vib(current_sfx);
            }
            if self.flags.is_active("METAL".to_string(), mouse_x, mouse_y) {
                sound_internal.player.set_metal(current_sfx);
            }
        }

        idx_prog = 0;

        let program = sound_internal.player.get_sound_program(current_sfx);
            for i in 0..32 {
                //println!("Program[{:?}] {:X}", i, program[i]);
                match chiptune::get_instruction(program[i] as i32) {
                    Ok(v) => {
                        //println!("{:?}", v);
                        match chiptune::notename(program[i] as i32, sound_internal.player.get_base_note(current_sfx)) {
                            Ok(name) => {
                                self.sfx.programs.push(program[i] as i32);
                                self.p_values[i as usize].set(program[i]);

                                if name == "Nop" {
                                    self.sfx.values.push("....".to_string());
                                } else {
                                    self.sfx.values.push(name);
                                }
                            },
                            Err(_) => (),
                        }
                    },
                    Err(e) => println!("Error {:?}", e),
                }

            }

        //self.base_note = sound_internal.player.get_base_note(current_sfx);
        //self.base_note_name = chiptune::base_note_name(self.base_note).unwrap();

//        info!("BASE NOTE {:?} -> {:?}", base_note, chiptune::base_note_name(base_note));

        if !define_value {
            let mut idx = 0;
            for key in KEYS_NOTE.iter() {
                let key = *key;
                if players.lock().unwrap().btn3(key) || self.pi_key.is_active2(key) {
                    let mut res_channel = -1;
                    if let Some(channel) = self.sfx_channels_keys.get_mut(&key) {
                        if *channel == -1 {
                            self.pi_key.lock(key);

                            let note = ((idx + 4 * 12) << 8) as u16;
                            res_channel = sound_internal.sfx(cartridge, sound.clone(), 
                                                             self.idx_sfx as i32, "".to_string(), -1, note, 64, 50, -1);
                        }
                    }

                    if res_channel >= 0 {
                        self.sfx_channels_keys.insert(key, res_channel);
                    }
                } else {
                    if let Some(channel) = self.sfx_channels_keys.get(&key) {
                        sound_internal.stop_chan(*channel);
                    }
                    self.sfx_channels_keys.insert(key, -1);
                    self.pi_key.release(key);
                }

                idx += 1;
            }
        }

        for (_, channel) in &self.sfx_channels_keys {
            if *channel != -1 {
                self.current_sfx_positions.insert(*channel as u32, sound_internal.player.get_sound_position(*channel) as usize);
            }
        }

        true
    }

    pub fn draw(&mut self, screen: &mut Screen) {
        /* Draw current SFX */
        screen.print(format!("INST {:?}/{:?} {}", self.idx_sfx, self.num_sfx, self.name), 64, 16, 7);
        self.next.draw(screen);
        self.prev.draw(screen);

        /* Draw flags */
        self.base_note.draw(screen);
        self.attack.draw(screen);
        self.decay.draw(screen);

        self.flags.draw(screen);

        let mut channel_rect_idx_x = 280;
        let channel_rect_idx_y = 220;

        for i in 0..16 {
            screen.rectfill(channel_rect_idx_x, channel_rect_idx_y, channel_rect_idx_x+4, channel_rect_idx_y+6, 8);
            channel_rect_idx_x += 6;
        }

        channel_rect_idx_x = 280;
        for (_, channel) in &self.sfx_channels_keys {
            if *channel != -1 {
                let idx_x = channel_rect_idx_x+6*(*channel - 16);
                screen.rectfill(idx_x, channel_rect_idx_y, idx_x+4, channel_rect_idx_y+6, 12);
            }
        }

        let mut idx_x = 4;
        let mut idx_y = 55;
        let mut idx_prog = 0;

        for idx in 0..self.sfx.programs.len() {
            if idx > 0 && idx % 8 == 0 {
                idx_x += 88;
                idx_y = 55;
            }
                
            for (_, channel) in &self.sfx_channels_keys {

                if *channel != -1 {
                    let position = self.current_sfx_positions.get(&(*channel as u32)).unwrap();
                    if *position == idx {
                        screen.pset(idx_x-4, idx_y+4, 8);
                    }
                }
            }
            screen.print(format!("{:02}", idx), idx_x, idx_y, 7);
            screen.print(format!("{:}", self.sfx.values[idx]), idx_x+50, idx_y, 7);

            idx_y += 9;
        }

        self.pi_key.draw(screen);

        for value in self.p_values.iter_mut() {
            value.draw(screen);
        }
    }
}

#[derive(Debug)]
pub enum MusicState {
    SFXEditor,
    TrackEditor,
}

pub struct MusicEditor {
    state_editor: MusicState,
    se: SFXEditor,
    te: TrackEditor,
    widgets: Vec<Arc<Mutex<Widget>>>,
}

impl MusicEditor {
    pub fn new(state: Arc<Mutex<State>>) -> MusicEditor {
        let mut widgets = Vec::new();
        let mut highlight = HashMap::new();
        highlight.insert(6, 10);

        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "TRACK".to_string(),
                                                     350,
                                                     20,
                                                     16,
                                                     16,
                                                     vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                           6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,  6,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                     highlight.clone(),
                                                     true, true))));

        widgets.push(Arc::new(Mutex::new(Widget::new(state.clone(),
                                                     "SFX".to_string(),
                                                     370,
                                                     20,
                                                     16,
                                                     16,
                                                     vec![ 6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,
                                                           6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  5,  6,  6,  6,  6,  5,  5,  5,  5,  6,  6,  6,  6,  5,  6,
                                                           6,  5,  6,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  6,  5,  6,
                                                           6,  5,  6,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  6,  5,  6,
                                                           6,  5,  6,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  6,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  6,  6,  6,  6,  6,  6,  5,  5,  5,  5,  6,
                                                           6,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  5,  6,
                                                           6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6,  6],
                                                     highlight.clone(),
                                                     false, true))));

        MusicEditor {
            state_editor: MusicState::TrackEditor,
            se: SFXEditor::new(state.clone()),
            te: TrackEditor::new(state.clone()),
            widgets: widgets,
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<UnicornConfig>>, screen: &mut Screen) {
        info!("[EDITOR][MUSIC] Init");
    }

    pub fn update(&mut self, cartridge: &mut UnicornCartridge, screen: &mut Screen, players: Arc<Mutex<Players>>, sound_internal: Arc<Mutex<SoundInternal>>, sound: Arc<Mutex<Sound>>) -> bool {
        let mut is_clickable = false;
        for widget in &self.widgets {
            is_clickable = widget.lock().unwrap().is_clickable();
            if is_clickable {
                break;
            }
        }

        if is_clickable {
            for widget in &self.widgets {
                widget.lock().unwrap().reset();
                widget.lock().unwrap().update();
                
                let is_click = widget.lock().unwrap().is_click();
                if is_click {
                    if widget.lock().unwrap().name == "TRACK" {
                        self.state_editor = MusicState::TrackEditor;
                        self.te.init(screen);
                    }
                    if widget.lock().unwrap().name == "SFX" {
                        self.state_editor = MusicState::SFXEditor;
                        self.se.init(screen);
                    }
                }
            }
        }

        match self.state_editor {
            MusicState::SFXEditor => {
                self.se.update(cartridge, players.clone(), sound_internal.clone(), sound.clone());
            }
            MusicState::TrackEditor => {
                self.te.update(cartridge, players.clone(), sound_internal.clone(), sound.clone());
            }
        }

        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {
        // Draw track or sfx editor
        match self.state_editor {
            MusicState::SFXEditor => {
                self.se.draw(screen);
            }
            MusicState::TrackEditor => {
                self.te.draw(screen);
            }
        }

        for widget in &self.widgets {
            widget.lock().unwrap().draw(screen);
        }
    }
}