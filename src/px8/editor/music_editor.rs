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


static KEYS_NOTE: [Keycode; 30] = [
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
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: i32,
    color_activated: i32,
    active: bool,
    active2: bool,
}

impl Touch {
    pub fn new(x1: i32, y1: i32, x2: i32, y2: i32, color: i32, color_activated: i32) -> Touch {
        Touch {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            color: color,
            color_activated: color_activated,
            active: false,
            active2: false,
        }
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

    pub fn add(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32, color_activated: i32, key: Keycode) {
        self.touches.insert(key, Touch::new(x1, y1, x2, y2, color, color_activated));
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

    pub fn update(&mut self, players: Arc<Mutex<Players>>) {
        let mouse_state = players.lock().unwrap().mouse_state_quick();

        let mouse_x = players.lock().unwrap().mouse_coordinate(0);
        let mouse_y = players.lock().unwrap().mouse_coordinate(1);

        if mouse_state == 1 {
            for (key, touch) in self.touches.iter_mut() {
                if point_in_rect(mouse_x, mouse_y, touch.x1, touch.y1, touch.x2, touch.y2) {
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
            if touch.active || touch.active2 {
                screen.rectfill(touch.x1, touch.y1, touch.x2, touch.y2, touch.color_activated);
            } else {
                screen.rectfill(touch.x1, touch.y1, touch.x2, touch.y2, touch.color);
            }
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
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>, screen: &mut Screen) {
        info!("[MUSIC_EDITOR] Init");

        self.pi_key.add(0, 180, 10, 220, 7, 8, Keycode::Z);
        self.pi_key.add(12, 180, 22, 220, 7, 8, Keycode::X);
        self.pi_key.add(24, 180, 34, 220, 7, 8, Keycode::C);
        self.pi_key.add(36, 180, 46, 220, 7, 8, Keycode::V);
        self.pi_key.add(48, 180, 58, 220, 7, 8, Keycode::B);
        self.pi_key.add(60, 180, 70, 220, 7, 8, Keycode::N);
        self.pi_key.add(72, 180, 82, 220, 7, 8, Keycode::M);
    }

    pub fn update(&mut self, cartridge: &mut PX8Cartridge, players: Arc<Mutex<Players>>, sound_internal: Arc<Mutex<SoundInternal>>, sound: Arc<Mutex<Sound>>) -> bool {
        let mut sound_internal = sound_internal.lock().unwrap();
        //let mut sound = sound.lock().unwrap();

        if cartridge.sound_tracks.len() == 0 {
            return true;
        }

        self.pi_key.update(players.clone());

        let current_sfx = *cartridge.sound_tracks.get_mut(&cartridge.sound_tracks_name[self.idx_sfx as usize]).unwrap();

        self.sfx.reset();

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
        screen.print("Inst".to_string(), 0, 16, 9);
        screen.print(format!("BASE {:?} {:?}", self.base_note_name, self.base_note), 0, 24, 7);

        let mut channel_rect_idx = 64;
        for i in 0..16 {
            screen.rectfill(channel_rect_idx, 16, channel_rect_idx+4, 20, 8);

            channel_rect_idx += 6;
        }

        channel_rect_idx = 64;
        for (_, channel) in &self.sfx_channels_keys {
            if *channel != -1 {
                let idx_x = channel_rect_idx+6*(*channel - 16);
                screen.rectfill(idx_x, 16, idx_x+4, 20, 11);
            }
        }

        let mut idx_x = 8;
        let mut idx_y = 32;

        for idx in 0..self.sfx.programs.len() {
            if idx > 0 && idx % 16 == 0 {
                idx_x = 128;
                idx_y = 32;
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
