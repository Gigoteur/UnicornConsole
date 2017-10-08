use gfx::Screen;
use config::Players;
use std::sync::{Arc, Mutex};
use std::cmp::{max, min};
use std::collections::HashMap;
use px8::editor::State;

use px8::PX8Config;
use time;

use sound::sound::SoundInternal;


static KEYS_NOTE: [char; 29] = [
    'z', 's', 'x', 'd', 'c', 'v', 'g', 'b', 'h', 'n', 'j', 'm',
    'q', '2', 'w', '3', 'e', 'r', '5', 't', '6', 'y', '7', 'u',
    'i', '9', 'o', '0', 'p',
];

pub struct MusicEditor {
    selected_sounds: String,
}

impl MusicEditor {
    pub fn new(state: Arc<Mutex<State>>) -> MusicEditor {
        MusicEditor {
            selected_sounds: "".to_string(),
        }
    }

    pub fn init(&mut self, config: Arc<Mutex<PX8Config>>, screen: &mut Screen) {
        info!("[MUSIC_EDITOR] Init");
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) -> bool {
        true
    }

    pub fn draw(&mut self, players: Arc<Mutex<Players>>, screen: &mut Screen, sound: Arc<Mutex<SoundInternal>>) {

        // Draw contour
        screen.rectfill(0, 16, 240, 24, 7);
        screen.print("Inst".to_string(), 0, 16, 9);
       // for (name, track) in &sound.lock().unwrap().chiptune_sound_tracks {

       // }
    }
}
