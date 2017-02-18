use std::collections::HashMap;

use sdl2;
use sdl2::AudioSubsystem;
use sdl2::audio::{AudioDevice, AudioQueue, AudioCallback, AudioSpecDesired, AudioSpecWAV, AudioCVT};

struct SoundFile {
    data: Vec<u8>,
    volume: f32,
    pos: usize,
}

impl AudioCallback for SoundFile {
    type Channel = u8;

    fn callback(&mut self, out: &mut [u8]) {
        for dst in out.iter_mut() {
            *dst = (*self.data.get(self.pos).unwrap_or(&0) as f32 * self.volume) as u8;
            self.pos += 1;
        }
    }
}

struct MyCallback {
    volume: f32
}
impl AudioCallback for MyCallback {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        use rand::{Rng, thread_rng};
        let mut rng = thread_rng();

        // Generate white noise
        for x in out.iter_mut() {
            *x = (rng.next_f32()*2.0 - 1.0) * self.volume;
        }
    }
}

#[derive(Copy, Clone)]
pub struct Sound {
    //sounds: HashMap<String, AudioDevice<SoundFile>>,
}

impl Sound {
    pub fn new() -> Sound {
        Sound {
          //  sounds: HashMap::new(),
        }
    }

    pub fn init(&mut self, sdl_audio: AudioSubsystem) {
       // sdl2::mixer::init(INIT_MP3 | INIT_FLAC | INIT_MOD | INIT_FLUIDSYNTH | INIT_MODPLUG | INIT_OGG).unwrap();
       // let _ = sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

    }
    pub fn load(&mut self, filename: String) {

    }

    pub fn play(&mut self, filename: String) {
    }

    pub fn stop(&mut self, filename: String) {
    }

        /*pub fn load(&mut self, filename: String) {
            let desired_spec = AudioSpecDesired {
                freq: Some(44100),
                channels: Some(2),
                samples: None,
            };

            let device = sdl_audio.open_playback(None, &desired_spec, |spec| {
                let wav = AudioSpecWAV::load_wav(filename.clone())
                    .ok()
                    .expect("Could not load test WAV file");

                let cvt = AudioCVT::new(
                    wav.format, wav.channels, wav.freq,
                    spec.format, spec.channels, spec.freq)
                    .ok()
                    .expect("Could not convert WAV file");

                let data = cvt.convert(wav.buffer().to_vec());

                // initialize the audio callback
                SoundFile {
                    data: data,
                    volume: 0.25,
                    pos: 0,
                }
            }).unwrap();


            self.sounds.insert(filename, device);
        }

        pub fn play(&mut self, filename: String) {
            match self.sounds.get(&filename) {
                Some(ref mut device) => device.resume(),
                _ => (),
            }
        }

        pub fn stop(&mut self, filename: String) {
            match self.sounds.get(&filename) {
                Some(ref mut device) => device.pause(),
                _ => (),
            }
        }*/
}