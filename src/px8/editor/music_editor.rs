use gfx::Screen;
use config::Players;
use std::sync::{Arc, Mutex};
use std::cmp::{max, min};
use std::collections::HashMap;
use px8::editor::State;

use px8::{PX8Cartridge, PX8Config};
use time;

use sound::sound::{SoundInternal, Sound};
use chiptune;


static KEYS_NOTE: [char; 29] = [
    'z', 's', 'x', 'd', 'c', 'v', 'g', 'b', 'h', 'n', 'j', 'm',
    'q', '2', 'w', '3', 'e', 'r', '5', 't', '6', 'y', '7', 'u',
    'i', '9', 'o', '0', 'p',
];

pub fn ChiptuneNote(sym: char, oct: i32) -> i32 {
    let mut n = 0;
    for key in KEYS_NOTE.iter() {
        if *key == sym {
            return n + oct * 12;
        }

        n += 1;
    }

    -1
}

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

pub struct MusicEditor {
    idx_sfx: u32,
    playing: bool,
    base_note: String,
    sfx: SFX,
    current_sfx_position: usize,
    selected_sounds: String,
}

impl MusicEditor {
    pub fn new(state: Arc<Mutex<State>>) -> MusicEditor {
        MusicEditor {
            idx_sfx: 0,
            playing: false,
            sfx: SFX::new(),
            current_sfx_position: 0,
            base_note: "".to_string(),
            selected_sounds: "".to_string(),
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>, screen: &mut Screen) {
        info!("[MUSIC_EDITOR] Init");
    }

    pub fn update(&mut self, cartridge: &mut PX8Cartridge, players: Arc<Mutex<Players>>, sound_internal: Arc<Mutex<SoundInternal>>, sound: Arc<Mutex<Sound>>) -> bool {
        let mut sound_internal = sound_internal.lock().unwrap();
        let mut sound = sound.lock().unwrap();

        let current_sfx = *cartridge.sound_tracks.get_mut(&cartridge.sound_tracks_name[self.idx_sfx as usize]).unwrap();
        let base_note = sound_internal.player.get_base_note(current_sfx);

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
                                self.sfx.values.push(name);
                            },
                            Err(_) => (),
                        }
                    },
                    Err(e) => println!("Error {:?}", e),
                }

            }

        self.base_note = chiptune::base_note_name(base_note).unwrap();

//        info!("BASE NOTE {:?} -> {:?}", base_note, chiptune::base_note_name(base_note));

        if players.lock().unwrap().btn2(122) {
            info!("Z");
            if !self.playing {
                sound.sfx(0, "".to_string(), 30, ChiptuneNote('z', base_note as i32) as u16, 64, 50, -1);
                self.playing = true;
            }
        } else {
            if self.playing {
                sound.music_stop();
                self.playing = false;
                self.current_sfx_position = 0;
            }
        }

        if self.playing {
            self.current_sfx_position = sound_internal.player.get_sound_position(30) as usize;
        }

        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen) {

        // Draw contour
        screen.rectfill(0, 16, 240, 24, 7);

        // Draw current SFX
        screen.print("Inst".to_string(), 0, 16, 9);
        screen.print(format!("BASE {:?}", self.base_note), 0, 24, 7);

        for idx in 0..self.sfx.programs.len() {
            if idx == self.current_sfx_position {
                screen.pset(4, 34+9*idx as i32, 8);
            }
            screen.print(format!("{:02} {:04X} {:}", idx, self.sfx.programs[idx], self.sfx.values[idx]), 8, 30+9*idx as i32, 7);
        }
    }
}
