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
            Sound { pa: portaudio::PortAudio::new().unwrap() }
        }

        pub fn init(&mut self) -> Result<(), portaudio::Error> {
            let settings =
                try!(self.pa
                         .default_output_stream_settings::<f32>(CHANNELS, SAMPLE_HZ, FRAMES));
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
                        buffer[idx] = *item; // as SampleOutput;
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
    use std::sync::{Arc, Mutex};

    /// Minimum value for playback volume parameter.
    pub const MIN_VOLUME: f64 = 0.0;

    /// Maximum value for playback volume parameter.
    pub const MAX_VOLUME: f64 = 1.0;

    pub struct SoundInternal {
        music_tracks: HashMap<String, mixer::Music>,
        sound_tracks: HashMap<String, mixer::Chunk>,
        pub csend: mpsc::Sender<Vec<u8>>,
        crecv: mpsc::Receiver<Vec<u8>>,
    }

    impl SoundInternal {
        pub fn new() -> SoundInternal {
            let (csend, crecv) = mpsc::channel();

            SoundInternal {
                music_tracks: HashMap::new(),
                sound_tracks: HashMap::new(),
                csend: csend,
                crecv: crecv,
            }
        }

        pub fn init(&mut self) {
            let _ = mixer::init(mixer::INIT_MP3 | mixer::INIT_FLAC | mixer::INIT_MOD |
                                mixer::INIT_FLUIDSYNTH |
                                mixer::INIT_MODPLUG |
                                mixer::INIT_OGG)
                    .unwrap();
            mixer::open_audio(mixer::DEFAULT_FREQUENCY,
                              mixer::DEFAULT_FORMAT,
                              mixer::DEFAULT_CHANNELS,
                              1024)
                    .unwrap();
            mixer::allocate_channels(16);
            info!("query spec => {:?}", sdl2::mixer::query_spec());
        }

        pub fn update(&mut self, sound: Arc<Mutex<Sound>>) {
            for sound_packet in self.crecv.try_iter() {
                info!("[SOUND] PACKET {:?}", sound_packet);
                match packet::read_packet(sound_packet).unwrap() {
                    // Music
                    packet::Packet::LoadMusic(res) => {
                        let filename = res.filename.clone();
                        let track = mixer::Music::from_file(filename.as_ref()).unwrap();
                        info!("[SOUND][SoundInternal] MUSIC Track {:?}", filename);
                        info!("music type => {:?}", track.get_type());
                        self.music_tracks.insert(filename, track);
                    }
                    packet::Packet::PlayMusic(res) => {
                        let filename = res.filename.clone();
                        self.music_tracks
                            .get(&filename)
                            .expect("music: Attempted to play value that is not bound to asset")
                            .play(res.loops).unwrap();
                    }
                    packet::Packet::StopMusic(_res) => {
                        sdl2::mixer::Music::halt();
                    }
                    packet::Packet::PauseMusic(_res) => {
                        sdl2::mixer::Music::pause();
                    }
                    packet::Packet::RewindMusic(_res) => {
                        sdl2::mixer::Music::rewind();
                    }
                    packet::Packet::ResumeMusic(_res) => {
                        sdl2::mixer::Music::resume();
                    }
                    packet::Packet::VolumeMusic(res) => {
                        sdl2::mixer::Music::set_volume(res.volume);
                    }

                    // Sound
                    packet::Packet::LoadSound(res) => {
                        let filename = res.filename.clone();
                        let track = mixer::Chunk::from_file(filename.as_ref()).unwrap();
                        info!("[SOUND][SoundInternal] SOUND Track {:?}", filename);
                        self.sound_tracks.insert(filename, track);
                    }
                    packet::Packet::PlaySound(res) => {
                        let filename = res.filename.clone();
                        sdl2::mixer::channel(res.channel)
                            .play(&self.sound_tracks.get(&filename).unwrap(), res.loops).unwrap();
                    }
                    packet::Packet::PauseSound(res) => {
                        sdl2::mixer::channel(res.channel)
                            .pause();
                    }
                    packet::Packet::ResumeSound(res) => {
                        sdl2::mixer::channel(res.channel)
                            .resume();
                    }
                    packet::Packet::StopSound(res) => {
                        sdl2::mixer::channel(res.channel)
                            .halt();
                    }
                    packet::Packet::VolumeSound(res) => {
                        sdl2::mixer::channel(res.channel)
                            .set_volume(res.volume);
                    }
                }
            }

            for i in 0..16 {
                sound.lock().unwrap().channels[i] = sdl2::mixer::channel(i as i32).is_playing();
            }
        }

        pub fn set_volume(&mut self, volume: f64) {
            info!("[SOUND][SoundInternal] music volume => {:?}",
                  sdl2::mixer::Music::get_volume());
            // Map 0.0 - 1.0 to 0 - 128 (sdl2::mixer::MAX_VOLUME).
            mixer::Music::set_volume((volume.max(MIN_VOLUME).min(MAX_VOLUME) *
                                      mixer::MAX_VOLUME as f64) as
                                     i32);
            info!("[SOUND][SoundInternal] music volume => {:?}",
                  sdl2::mixer::Music::get_volume());
        }
    }

    pub struct Sound {
        csend: mpsc::Sender<Vec<u8>>,
        channels: [bool; 16],
    }

    impl Sound {
        pub fn new(csend: mpsc::Sender<Vec<u8>>) -> Sound {
            Sound {
                csend: csend,
                channels: [false; 16],
            }
        }

        // Music
        pub fn load(&mut self, filename: String) -> i32 {
            info!("[SOUND] Load music {:?}", filename);
            let p = packet::LoadMusic { filename: filename };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
            0
        }

        pub fn play(&mut self, filename: String, loops: i32) {
            info!("[SOUND] Play music {:?} {:?}", filename, loops);
            let p = packet::PlayMusic {
                filename: filename,
                loops: loops,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn stop(&mut self) {
            info!("[SOUND] Stop music");
            let p = packet::StopMusic { filename: "".to_string() };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn pause(&mut self) {
            info!("[SOUND] Pause music");
            let p = packet::PauseMusic { filename: "".to_string() };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn resume(&mut self) {
            info!("[SOUND] Resume music");
            let p = packet::ResumeMusic { filename: "".to_string() };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn rewind(&mut self) {
            info!("[SOUND] Rewind music");
            let p = packet::RewindMusic { filename: "".to_string() };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn volume(&mut self, volume: i32) {
            info!("[SOUND] Volume music");
            let p = packet::VolumeMusic { volume: volume };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }


        // Sound
        pub fn load_sound(&mut self, filename: String) -> i32 {
            info!("[SOUND] Load sound {:?}", filename);
            let p = packet::LoadSound { filename: filename };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
            0
        }

        pub fn play_sound(&mut self, filename: String, loops: i32, channel: i32) {
            info!("[SOUND] Play sound {:?} {:?} {:?}", filename, loops, channel);
            let p = packet::PlaySound {
                filename: filename,
                loops: loops,
                channel: channel,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn pause_sound(&mut self, channel: i32) {
            info!("[SOUND] Pause sound {:?}", channel);
            let p = packet::PauseSound {
                channel: channel,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn resume_sound(&mut self, channel: i32) {
            info!("[SOUND] Resume sound {:?}", channel);
            let p = packet::ResumeSound {
                channel: channel,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn stop_sound(&mut self, channel: i32) {
            info!("[SOUND] Stop sound {:?}", channel);
            let p = packet::StopSound {
                channel: channel,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn volume_sound(&mut self, volume: i32, channel: i32) {
            info!("[SOUND] Volume sound {:?} {:?}", volume, channel);
            let p = packet::VolumeSound {
                volume: volume,
                channel: channel,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn isplaying_sound(&mut self, channel: i32) -> bool {
            self.channels[channel as usize]
        }
    }
}

#[cfg(all(not(feature = "sdl_audio"), not(feature = "portaudio")))]
pub mod sound {
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex};

    pub struct SoundInternal {
        pub csend: mpsc::Sender<Vec<u8>>,
    }

    impl SoundInternal {
        pub fn new() -> SoundInternal {
            let (csend, _) = mpsc::channel();

            SoundInternal { csend: csend }
        }

        pub fn init(&mut self) {}
        pub fn update(&mut self, _sound: Arc<Mutex<Sound>>) {}
    }

    pub struct Sound {}

    impl Sound {
        pub fn new(_csend: mpsc::Sender<Vec<u8>>) -> Sound {
            Sound {}
        }

        // Music
        pub fn load(&mut self, _filename: String) -> i32 {
            0
        }
        pub fn play(&mut self, _filename: String, _loops: i32) {}

        pub fn stop(&mut self) {}

        pub fn pause(&mut self) {}

        pub fn resume(&mut self) {}

        pub fn rewind(&mut self) {}

        pub fn volume(&mut self, _volume: i32) {}

        // Sound
        pub fn load_sound(&mut self, _filename: String) -> i32 {
            0
        }

        pub fn play_sound(&mut self, _filename: String, _loops: i32, _channels: i32) {
        }
        pub fn pause_sound(&mut self, _channels: i32) {
        }
        pub fn resume_sound(&mut self, _channels: i32) {
        }
        pub fn stop_sound(&mut self, _channels: i32) {
        }
        pub fn volume_sound(&mut self, _volume: i32, _channels: i32) {
        }
        pub fn isplaying_sound(&mut self, _channels: i32) -> bool {
            false
        }
    }
}
