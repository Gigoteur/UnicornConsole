use std::collections::HashMap;

use sdl2;
use sdl2::audio::{AudioCallback, AudioSpecDesired};


pub struct Sound {

}

impl Sound {
    pub fn new() -> Sound {
        Sound {
        }
    }

    pub fn init(&mut self) {
        let audio_subsystem = sdl_context.audio().unwrap();

        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(2),  // mono
            samples: 1024       // default sample size
        };

        let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            // Show obtained AudioSpec
            println!("{:?}", spec);

        }).unwrap();

    }

    pub fn load(&mut self, filename: String) {

    }

    pub fn play(&mut self, filename: String) {
    }

    pub fn stop(&mut self, filename: String) {
    }
}