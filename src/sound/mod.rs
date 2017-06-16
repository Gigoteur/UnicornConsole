pub mod song;

#[cfg(feature = "portaudio")]
pub mod sound {
    use portaudio;

    const CHANNELS: i32 = 2;
    const FRAMES: u32 = 64;
    const SAMPLE_HZ: f64 = 44_100.0;

    pub struct Sound {
        pa: portaudio::PortAudio,
    }

    impl Sound {
        pub fn new() -> Sound {
            Sound {
                pa: portaudio::PortAudio::new().unwrap(),
            }
        }

        pub fn init(&mut self) -> Result<(), portaudio::Error> {
            let settings = try!(self.pa.default_output_stream_settings::<f32>(CHANNELS, SAMPLE_HZ, FRAMES));
            let mut stream = try!(self.pa.open_non_blocking_stream(settings, callback));
            try!(stream.start());

            let callback_fn = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
                if let Ok(command) = receiver.try_recv() {
                    generator.process_command(command);
                }

                generator.get_samples(frames, &mut generator_buffer);
                let mut idx = 0;
                for item in generator_buffer.iter().take(frames) {
                    for _ in 0..(channel_count as usize) {
                        buffer[idx] = *item;// as SampleOutput;
                        idx += 1;
                    }
                }

                portaudio::Continue
            };

            Ok(())
        }

        pub fn load(&mut self, _filename: String) -> i32 {
            0
        }
        pub fn play(&mut self, _id: u32) {}

        pub fn stop(&mut self, _id: u32) {}
    }

}

#[cfg(feature = "sdl_audio")]
pub mod sound {
    use std::sync::mpsc;
    use px8::packet;

    use std::collections::HashMap;
    use sdl2;
    use sdl2::mixer;

    /// Minimum value for playback volume parameter.
    pub const MIN_VOLUME: f64 = 0.0;

    /// Maximum value for playback volume parameter.
    pub const MAX_VOLUME: f64 = 1.0;

    pub struct SoundInternal {
        music_tracks: HashMap<String, mixer::Music>,
        pub csend: mpsc::Sender<Vec<u8>>,
        crecv: mpsc::Receiver<Vec<u8>>,
    }

    impl SoundInternal {
        pub fn new() -> SoundInternal {
            let (csend, crecv) = mpsc::channel();

            SoundInternal {
                music_tracks: HashMap::new(),
                csend: csend,
                crecv: crecv,
            }
        }

        pub fn init(&mut self) {
            let _ = mixer::init(mixer::INIT_MP3 | mixer::INIT_FLAC | mixer::INIT_MOD |
                mixer::INIT_FLUIDSYNTH | mixer::INIT_MODPLUG |
                mixer::INIT_OGG).unwrap();
            mixer::open_audio(44100,
                              mixer::DEFAULT_FORMAT,
                              mixer::DEFAULT_CHANNELS,
                              1024).unwrap();
            mixer::allocate_channels(16);
            info!("query spec => {:?}", sdl2::mixer::query_spec());
        }

        pub fn update(&mut self) {
            for sound_packet in self.crecv.try_iter() {
                info!("[SOUND] PACKET {:?}", sound_packet);
                match packet::read_packet(sound_packet).unwrap() {
                    packet::Packet::LoadSound(res) => {
                        let filename = res.filename.clone();
                        let track = mixer::Music::from_file(filename.as_ref()).unwrap();
                        info!("[SOUND][SoundInternal] Track {:?}", track);
                        info!("music type => {:?}", track.get_type());
                        self.music_tracks.insert(filename, track);
                    },
                    packet::Packet::PlaySound(res) => {
                        let filename = res.filename.clone();
                        self.music_tracks.get(&filename).expect("music: Attempted to play value that is not bound to asset").play(-1);
                    }
                    packet::Packet::StopSound(res) => {
                        let filename = res.filename.clone();
                        sdl2::mixer::Music::halt();
                    }
                }
            }
        }

        pub fn set_volume(&mut self, volume: f64) {
            info!("[SOUND][SoundInternal] music volume => {:?}", sdl2::mixer::Music::get_volume());
            // Map 0.0 - 1.0 to 0 - 128 (sdl2::mixer::MAX_VOLUME).
            mixer::Music::set_volume((volume.max(MIN_VOLUME).min(MAX_VOLUME) *
                mixer::MAX_VOLUME as f64) as i32);
            info!("[SOUND][SoundInternal] music volume => {:?}", sdl2::mixer::Music::get_volume());
        }

    }

    #[derive(Copy, Clone)]
    pub enum Repeat {
        /// Repeats forever.
        Forever,
        /// Repeats amount of times.
        Times(u16),
    }

    impl Repeat {
        fn to_sdl2_repeats(&self) -> i32 {
            match *self {
                Repeat::Forever => -1,
                Repeat::Times(val) => val as i32,
            }
        }
    }

    pub struct Sound {
        csend: mpsc::Sender<Vec<u8>>,
    }

    impl Sound {
        pub fn new(csend: mpsc::Sender<Vec<u8>>) -> Sound {
            Sound {
                csend: csend,
            }
        }

        pub fn load(&mut self, filename: String) -> i32 {
            info!("Load sound {:?}", filename);
            let p = packet::LoadSound {
                filename: filename,
            };
            self.csend.send(packet::write_packet(p).unwrap());
            0
        }

        pub fn play(&mut self, filename: String) {
            info!("Play sound {:?}", filename);
            let p = packet::PlaySound {
                filename: filename,
            };
            self.csend.send(packet::write_packet(p).unwrap());
        }

        pub fn stop(&mut self, filename: String) {
            info!("Stop sound {:?}", filename);
            let p = packet::StopSound {
                filename: filename,
            };
            self.csend.send(packet::write_packet(p).unwrap());
        }
    }
}

#[cfg(all(not(feature = "sdl_audio"), not(feature = "portaudio")))]
pub mod sound {
    use std::sync::mpsc;

    pub struct SoundInternal {
        pub csend: mpsc::Sender<Vec<u8>>,
    }

    impl SoundInternal {
        pub fn new() -> SoundInternal {
            let (csend, _) = mpsc::channel();

            SoundInternal {
                csend: csend,
            }
        }

        pub fn init(&mut self) {}
        pub fn update(&mut self) {}
    }

    pub struct Sound {
    }

    impl Sound {
        pub fn new(_csend: mpsc::Sender<Vec<u8>>) -> Sound {
            Sound {}
        }

        pub fn load(&mut self, _filename: String) -> i32 {
            0
        }
        pub fn play(&mut self, _filename: String) {}

        pub fn stop(&mut self, _filename: String) {}
    }
}
