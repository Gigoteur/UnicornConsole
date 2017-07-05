pub mod song;

pub mod sound {
    use std::sync::mpsc;
    use px8::packet;

    use std::collections::HashMap;
    use sdl2;
    use sdl2::mixer;
    use std::sync::{Arc, Mutex};

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

        pub fn pause(&mut self) {
            sdl2::mixer::Music::pause();
            sdl2::mixer::channel(-1).pause();
        }

        pub fn resume(&mut self) {
            sdl2::mixer::Music::resume();
            sdl2::mixer::channel(-1).resume();
        }

        pub fn stop(&mut self) {
            sdl2::mixer::Music::halt();
            sdl2::mixer::channel(-1).halt();
        }


        pub fn update(&mut self, sound: Arc<Mutex<Sound>>) {
            for sound_packet in self.crecv.try_iter() {
                debug!("[SOUND] PACKET {:?}", sound_packet);
                match packet::read_packet(sound_packet).unwrap() {
                    // Music
                    packet::Packet::LoadMusic(res) => {
                        let filename = res.filename.clone();
                        let track = mixer::Music::from_file(filename.as_ref()).unwrap();
                        debug!("[SOUND][SoundInternal] MUSIC Track {:?}", filename);
                        debug!("music type => {:?}", track.get_type());
                        self.music_tracks.insert(filename, track);
                    }
                    packet::Packet::PlayMusic(res) => {
                        let filename = res.filename.clone();
                        self.music_tracks
                            .get(&filename)
                            .expect("music: Attempted to play value that is not bound to asset")
                            .play(res.loops)
                            .unwrap();
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
                        debug!("[SOUND][SoundInternal] SOUND Track {:?}", filename);
                        self.sound_tracks.insert(filename, track);
                    }
                    packet::Packet::PlaySound(res) => {
                        let filename = res.filename.clone();
                        sdl2::mixer::channel(res.channel)
                            .play(&self.sound_tracks.get(&filename).unwrap(), res.loops);
                    }
                    packet::Packet::PauseSound(res) => {
                        sdl2::mixer::channel(res.channel).pause();
                    }
                    packet::Packet::ResumeSound(res) => {
                        sdl2::mixer::channel(res.channel).resume();
                    }
                    packet::Packet::StopSound(res) => {
                        sdl2::mixer::channel(res.channel).halt();
                    }
                    packet::Packet::VolumeSound(res) => {
                        sdl2::mixer::channel(res.channel).set_volume(res.volume);
                    }
                }
            }

            for i in 0..16 {
                sound.lock().unwrap().channels[i] = sdl2::mixer::channel(i as i32).is_playing();
            }
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
            debug!("[SOUND] Load music {:?}", filename);
            let p = packet::LoadMusic { filename: filename };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
            0
        }

        pub fn play(&mut self, filename: String, loops: i32) {
            debug!("[SOUND] Play music {:?} {:?}", filename, loops);
            let p = packet::PlayMusic {
                filename: filename,
                loops: loops,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn stop(&mut self) {
            debug!("[SOUND] Stop music");
            let p = packet::StopMusic { filename: "".to_string() };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn pause(&mut self) {
            debug!("[SOUND] Pause music");
            let p = packet::PauseMusic { filename: "".to_string() };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn resume(&mut self) {
            debug!("[SOUND] Resume music");
            let p = packet::ResumeMusic { filename: "".to_string() };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn rewind(&mut self) {
            debug!("[SOUND] Rewind music");
            let p = packet::RewindMusic { filename: "".to_string() };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn volume(&mut self, volume: i32) {
            debug!("[SOUND] Volume music");
            let p = packet::VolumeMusic { volume: volume };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }


        // Sound
        pub fn sound_load(&mut self, filename: String) -> i32 {
            debug!("[SOUND] Load sound {:?}", filename);
            let p = packet::LoadSound { filename: filename };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
            0
        }

        pub fn sound_play(&mut self, filename: String, loops: i32, channel: i32) {
            debug!("[SOUND] Play sound {:?} {:?} {:?}",
                   filename,
                   loops,
                   channel);
            let p = packet::PlaySound {
                filename: filename,
                loops: loops,
                channel: channel,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn sound_pause(&mut self, channel: i32) {
            debug!("[SOUND] Pause sound {:?}", channel);
            let p = packet::PauseSound { channel: channel };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn sound_resume(&mut self, channel: i32) {
            debug!("[SOUND] Resume sound {:?}", channel);
            let p = packet::ResumeSound { channel: channel };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn sound_stop(&mut self, channel: i32) {
            debug!("[SOUND] Stop sound {:?}", channel);
            let p = packet::StopSound { channel: channel };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn sound_volume(&mut self, volume: i32, channel: i32) {
            debug!("[SOUND] Volume sound {:?} {:?}", volume, channel);
            let p = packet::VolumeSound {
                volume: volume,
                channel: channel,
            };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn sound_isplaying(&mut self, channel: i32) -> bool {
            self.channels[channel as usize]
        }
    }
}
