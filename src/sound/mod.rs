use std::collections::HashMap;

use sdl2;
use sdl2::mixer::{AUDIO_S16};

pub struct Sound {

}

impl Sound {
    pub fn new() -> Sound {
        Sound {
        }
    }

    pub fn init(&mut self) {
        let _ = sdl2::mixer::open_audio(22050, AUDIO_S16, 2, 1024).unwrap();
        sdl2::mixer::allocate_channels(8);
    }

    pub fn load(&mut self, filename: String) {

    }

    pub fn play(&mut self, filename: String) {
    }

    pub fn stop(&mut self, filename: String) {
    }
}