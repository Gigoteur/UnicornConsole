use gfx::Screen;
use config::Players;
use std::sync::{Arc, Mutex};
use std::cmp::{max, min};
use std::collections::HashMap;
use px8::editor::State;
use sdl2::keyboard::Keycode;

use px8::{PX8Cartridge, PX8Config};
use time;
use px8::editor::point_in_rect;

use sound::sound::{SoundInternal, Sound};
use chiptune;


static KEYS_NOTE: [Keycode; 29] = [
    Keycode::Z,
    Keycode::S,
    Keycode::X,
    Keycode::D,
    Keycode::C,
    Keycode::V,
    Keycode::G,
    Keycode::B,
    Keycode::H,
    Keycode::N,
    Keycode::J,
    Keycode::M,
    Keycode::Q,
    Keycode::Num2,
    Keycode::W,
    Keycode::Num3,
    Keycode::E,
    Keycode::R,
    Keycode::Num5,
    Keycode::T,
    Keycode::Num6,
    Keycode::Y,
    Keycode::Num7,
    Keycode::U,
    Keycode::I,
    Keycode::Num9,
    Keycode::O,
    Keycode::Num0,
    Keycode::P,

    
  //  'z', 's', 'x', 'd', 'c', 'v', 'g', 'b', 'h', 'n', 'j', 'm',
  //  'q', '2', 'w', '3', 'e', 'r', '5', 't', '6', 'y', '7', 'u',
  //  'i', '9', 'o', '0', 'p',
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
    touches: HashMap<Keycode, Touch>,
}

impl PianoKeyboard {
    pub fn new() -> PianoKeyboard {
        PianoKeyboard {
            touches: HashMap::new(),
        }
    }

    pub fn add(&mut self, rects: Vec<i32>, color: i32, color_activated: i32, offset_x: i32, offset_y: i32, key: Keycode) {
        self.touches.insert(key, Touch::new(rects.clone(), offset_x, offset_y, color, color_activated));
    }

    pub fn lock(&mut self, key: Keycode) {
        if let Some(touch) = self.touches.get_mut(&key) {
            touch.active = true;
        }
    }

    pub fn release(&mut self, key: Keycode) {
        if let Some(touch) = self.touches.get_mut(&key) {
            touch.active = false;
        }
    }


    pub fn is_active2(&mut self, key: Keycode) -> bool {
        if let Some(touch) = self.touches.get(&key) {
            return touch.active2
        }
        false
    }

    pub fn is_active(&mut self, key: Keycode) -> bool {
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

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {
        for (keycode, touch) in &self.touches {
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

pub struct MusicEditor {
    idx_sfx: u32,
    base_note: u8,
    base_note_name: String,
    pi_key: PianoKeyboard,
    sfx: SFX,
    sfx_channels_keys: HashMap<Keycode, i32>,
    current_sfx_positions: HashMap<u32, usize>,
    selected_sounds: String,
    flags: SFXFlags,
}

impl MusicEditor {
    pub fn new(state: Arc<Mutex<State>>) -> MusicEditor {
        MusicEditor {
            idx_sfx: 0,
            sfx: SFX::new(),
            current_sfx_positions: HashMap::new(),
            sfx_channels_keys: HashMap::new(),
            pi_key: PianoKeyboard::new(),
            base_note: 0,
            base_note_name: "".to_string(),
            selected_sounds: "".to_string(),
            flags: SFXFlags::new(),
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>, screen: &mut Screen) {
        info!("[MUSIC_EDITOR] Init");

        self.flags.add("DRUM".to_string(), 0, 16, vec![0, 16, 20, 24], 7, 8, 11);
        self.flags.add("PUL".to_string(), 22, 16, vec![22, 16, 38, 24], 7, 8, 11);
        self.flags.add("NOISE".to_string(), 40, 16, vec![40, 16, 60, 24], 7, 8, 11);
        self.flags.add("TRI".to_string(), 62, 16, vec![62, 16, 74, 24], 7, 8, 11);
        self.flags.add("VIB".to_string(), 76, 16, vec![76, 16, 88, 24], 7, 8, 11);
        self.flags.add("SAW".to_string(), 90, 16, vec![90, 16, 102, 24], 7, 8, 11);
        self.flags.add("METAL".to_string(), 104, 16, vec![104, 16, 124, 24], 7, 8, 11);

        // White -> 18
        // Black -> 10
        let offset_y = 0;
        let offset_x = 44;

        self.pi_key.add(vec![0, 180, 12, 208, 0, 208, 18, 226], 7, 8, offset_x, offset_y, Keycode::Z);
        self.pi_key.add(vec![25, 180, 33, 208, 20, 208, 38, 226], 7, 8, offset_x, offset_y, Keycode::X);
        self.pi_key.add(vec![45, 180, 58, 208, 40, 208, 58, 226], 7, 8, offset_x, offset_y, Keycode::C);

        self.pi_key.add(vec![13, 180, 23, 207], 0, 1, offset_x, offset_y, Keycode::S);
        self.pi_key.add(vec![34, 180, 44, 207], 0, 1, offset_x, offset_y, Keycode::D);

        self.pi_key.add(vec![60, 180, 72, 208, 60, 208, 78, 226], 7, 8, offset_x, offset_y, Keycode::V);
        self.pi_key.add(vec![85, 180, 93, 208, 80, 208, 98, 226], 7, 8, offset_x, offset_y, Keycode::B);
        self.pi_key.add(vec![105, 180, 113, 208, 100, 208, 118, 226], 7, 8, offset_x, offset_y, Keycode::N);
        self.pi_key.add(vec![125, 180, 138, 208, 120, 208, 138, 226], 7, 8, offset_x, offset_y, Keycode::M);

        self.pi_key.add(vec![73, 180, 83, 207], 0, 1, offset_x, offset_y, Keycode::G);
        self.pi_key.add(vec![94, 180, 104, 207], 0, 1, offset_x, offset_y, Keycode::H);
        self.pi_key.add(vec![114, 180, 124, 207], 0, 1, offset_x, offset_y, Keycode::J);


        let offset_x = 19;
        let offset_y = -50;
        self.pi_key.add(vec![0, 180, 12, 208, 0, 208, 18, 226], 7, 8, offset_x, offset_y, Keycode::Q);
        self.pi_key.add(vec![25, 180, 33, 208, 20, 208, 38, 226], 7, 8, offset_x, offset_y, Keycode::W);
        self.pi_key.add(vec![45, 180, 58, 208, 40, 208, 58, 226], 7, 8, offset_x, offset_y, Keycode::E);

        self.pi_key.add(vec![13, 180, 23, 207], 0, 1, offset_x, offset_y, Keycode::Num2);
        self.pi_key.add(vec![34, 180, 44, 207], 0, 1, offset_x, offset_y, Keycode::Num3);

        self.pi_key.add(vec![60, 180, 72, 208, 60, 208, 78, 226], 7, 8, offset_x, offset_y, Keycode::R);
        self.pi_key.add(vec![85, 180, 93, 208, 80, 208, 98, 226], 7, 8, offset_x, offset_y, Keycode::T);
        self.pi_key.add(vec![105, 180, 113, 208, 100, 208, 118, 226], 7, 8, offset_x, offset_y, Keycode::Y);
        self.pi_key.add(vec![125, 180, 138, 208, 120, 208, 138, 226], 7, 8, offset_x, offset_y, Keycode::U);

        self.pi_key.add(vec![73, 180, 83, 207], 0, 1, offset_x, offset_y, Keycode::Num5);
        self.pi_key.add(vec![94, 180, 104, 207], 0, 1, offset_x, offset_y, Keycode::Num6);
        self.pi_key.add(vec![114, 180, 124, 207], 0, 1, offset_x, offset_y, Keycode::Num7);


        let offset_x = 159;
        let offset_y = -50;
        self.pi_key.add(vec![0, 180, 12, 208, 0, 208, 18, 226], 7, 8, offset_x, offset_y, Keycode::I);
        self.pi_key.add(vec![25, 180, 33, 208, 20, 208, 38, 226], 7, 8, offset_x, offset_y, Keycode::O);
        self.pi_key.add(vec![45, 180, 58, 208, 40, 208, 58, 226], 7, 8, offset_x, offset_y, Keycode::P);

        self.pi_key.add(vec![13, 180, 23, 207], 0, 1, offset_x, offset_y, Keycode::Num9);
        self.pi_key.add(vec![34, 180, 44, 207], 0, 1, offset_x, offset_y, Keycode::Num0);
    }

    pub fn update(&mut self, cartridge: &mut PX8Cartridge, players: Arc<Mutex<Players>>, sound_internal: Arc<Mutex<SoundInternal>>, sound: Arc<Mutex<Sound>>) -> bool {
        let mouse_state_quick = players.lock().unwrap().mouse_state_quick();
        let mouse_state = players.lock().unwrap().mouse_state();

        let mouse_x = players.lock().unwrap().mouse_coordinate(0);
        let mouse_y = players.lock().unwrap().mouse_coordinate(1);

        let mut sound_internal = sound_internal.lock().unwrap();

        if cartridge.sound_tracks.len() == 0 {
            return true;
        }

        self.pi_key.update(mouse_state_quick, mouse_x, mouse_y, players.clone());

        let current_sfx = *cartridge.sound_tracks.get_mut(&cartridge.sound_tracks_name[self.idx_sfx as usize]).unwrap();

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

        let program = sound_internal.player.get_sound_program(current_sfx);
            for i in 0..32 {
                //println!("Program[{:?}] {:X}", i, program[i]);
                match chiptune::get_instruction(program[i] as i32) {
                    Ok(v) => {
                        //println!("{:?}", v);
                        match chiptune::notename(program[i] as i32, sound_internal.player.get_base_note(current_sfx)) {
                            Ok(name) => {
                                self.sfx.programs.push(program[i] as i32);
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

        self.base_note = sound_internal.player.get_base_note(current_sfx);
        self.base_note_name = chiptune::base_note_name(self.base_note).unwrap();

//        info!("BASE NOTE {:?} -> {:?}", base_note, chiptune::base_note_name(base_note));

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
                                                         0, "".to_string(), -1, note, 64, 50, -1);
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

        for (_, channel) in &self.sfx_channels_keys {
            if *channel != -1 {
                self.current_sfx_positions.insert(*channel as u32, sound_internal.player.get_sound_position(*channel) as usize);
            }
        }

        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {

        // Draw contour
        screen.rectfill(0, 16, 240, 24, 7);

        // Draw current SFX
        screen.print("Inst".to_string(), 0, 8, 9);
        screen.print(format!("BASE {:?} {:?}", self.base_note_name, self.base_note), 0, 24, 7);

        self.flags.draw(screen);

        let mut channel_rect_idx = 64;
        for i in 0..16 {
            screen.rectfill(channel_rect_idx, 230, channel_rect_idx+4, 236, 8);
            channel_rect_idx += 6;
        }

        channel_rect_idx = 64;
        for (_, channel) in &self.sfx_channels_keys {
            if *channel != -1 {
                let idx_x = channel_rect_idx+6*(*channel - 16);
                screen.rectfill(idx_x, 230, idx_x+4, 236, 12);
            }
        }

        let mut idx_x = 4;
        let mut idx_y = 48;

        for idx in 0..self.sfx.programs.len() {
            if idx > 0 && idx % 8 == 0 {
                idx_x += 58;
                idx_y = 48;
            }
                
            for (_, channel) in &self.sfx_channels_keys {

                if *channel != -1 {
                    let position = self.current_sfx_positions.get(&(*channel as u32)).unwrap();
                    if *position == idx {
                        screen.pset(idx_x-4, idx_y+4, 8);
                    }
                }
            }

            screen.print(format!("{:02} {:04X} {:}", idx, self.sfx.programs[idx], self.sfx.values[idx]), idx_x, idx_y, 7);
            idx_y += 9;
        }

        self.pi_key.draw(players.clone(), screen);
    }
}
