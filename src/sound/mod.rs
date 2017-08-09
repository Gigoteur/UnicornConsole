pub mod song;


pub mod sound {
    use std::sync::mpsc;
    use px8::packet;

    use chiptune;

    use std::collections::HashMap;
    use sdl2;
    use sdl2::mixer;
    use std::sync::{Arc, Mutex};

    pub struct SoundInternal {
        player: chiptune::Chiptune,
        chiptune_music_tracks: HashMap<String, chiptune::ChiptuneSong>,
        chiptune_sound_tracks: HashMap<String, chiptune::ChiptuneSound>,
        music_tracks: HashMap<String, mixer::Music>,
        sound_tracks: HashMap<String, mixer::Chunk>,
        pub csend: mpsc::Sender<Vec<u8>>,
        crecv: mpsc::Receiver<Vec<u8>>,
    }

    impl SoundInternal {
        pub fn new() -> SoundInternal {
            let (csend, crecv) = mpsc::channel();

            SoundInternal {
                player: chiptune::Chiptune::new(),
                chiptune_music_tracks: HashMap::new(),
                chiptune_sound_tracks: HashMap::new(),
                music_tracks: HashMap::new(),
                sound_tracks: HashMap::new(),
                csend: csend,
                crecv: crecv,
            }
        }

        pub fn init(&mut self) {
            info!("query spec => {:?}", sdl2::mixer::query_spec());
        }

        pub fn pause(&mut self) {
            info!("[SOUND] Pause");
            sdl2::mixer::Music::pause();
            sdl2::mixer::channel(-1).pause();
            self.player.pause(1);
        }

        pub fn resume(&mut self) {
            info!("[SOUND] Resume");
            sdl2::mixer::Music::resume();
            sdl2::mixer::channel(-1).resume();
            self.player.pause(0);
        }

        pub fn stop(&mut self) {
            info!("[SOUND] Stop");
            sdl2::mixer::Music::halt();
            sdl2::mixer::channel(-1).halt();
            self.player.stop();
        }


        pub fn update(&mut self, sound: Arc<Mutex<Sound>>) {
            for sound_packet in self.crecv.try_iter() {
                debug!("[SOUND] PACKET {:?}", sound_packet);
                match packet::read_packet(sound_packet).unwrap() {
                    // Chiptune
                    packet::Packet::ChiptunePlay(res) => {
                        let filename = res.filename.clone();
                        // New song -> Load it before
                        if res.filetype == 0 {
                            if !self.chiptune_music_tracks.contains_key(&filename) {
                                    let song = self.player.load_music(filename.clone());
                                    match song {
                                        Ok(chip_song) => {
                                            self.chiptune_music_tracks.insert(filename.clone(), chip_song);
                                        }

                                        Err(e) => error!("ERROR to load the song {:?}", e),
                                    }
                            }
                            match self.chiptune_music_tracks.get_mut(&filename) {
                                Some(mut song) => {
                                    self.player.play_music(&mut song, res.start_position);
                                    self.player.set_looping(res.loops);
                                }
                                None => {},
                            }
                        }

                        // New sound effect
                        if res.filetype == 1 {
                            if !self.chiptune_sound_tracks.contains_key(&filename) {
                                    let sound = self.player.load_sound(filename.clone());
                                    match sound {
                                        Ok(chip_sound) => {
                                            self.chiptune_sound_tracks.insert(filename.clone(), chip_sound);
                                        }

                                        Err(e) => error!("ERROR to load the song {:?}", e),
                                    }
                            }
                            match self.chiptune_sound_tracks.get_mut(&filename) {
                                Some(mut sound) => {
                                    self.player.play_sound(&mut sound, -1, 0, chiptune::CYD_PAN_CENTER);
                                }
                                None => {},
                            }
                        }
                    }
                    packet::Packet::ChiptuneStop(res) => {
                        if res.music == 1 && res.sound == 1 {
                            self.player.stop();
                        }

                        if res.music == 1 && res.sound == 0 {
                            self.player.stop_music();
                        }

                        if res.music == 0 && res.sound == 1 {
                            self.player.stop_sound();
                        }
                    }
                    packet::Packet::ChiptunePause(res) => {
                        if res.music == 1 && res.sound == 1 {
                            self.player.pause(1);
                        }

                        if res.music == 1 && res.sound == 0 {
                            self.player.pause_music(1);
                        }

                        if res.music == 0 && res.sound == 1 {
                            self.player.pause_sound(1);
                        }
                    }
                    packet::Packet::ChiptuneResume(res) => {
                        if res.music == 1 && res.sound == 1 {
                            self.player.pause(0);
                        }

                        if res.music == 1 && res.sound == 0 {
                            self.player.pause_music(0);
                        }

                        if res.music == 0 && res.sound == 1 {
                            self.player.pause_sound(0);
                        }
                    }                    
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

        // Chiptune
        pub fn chiptune_play(&mut self, filetype: i32, filename: String, loops: i32, start_position: i32) {
            debug!("[SOUND] Chiptune PLAY {:?}", filename);
            let p = packet::ChiptunePlay { filetype: filetype, filename: filename, loops: loops, start_position: start_position };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn chiptune_stop(&mut self, music: i32, sound: i32) {
            debug!("[SOUND] Chiptune STOP");
            let p = packet::ChiptuneStop { music: music, sound: sound };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn chiptune_pause(&mut self, music: i32, sound: i32) {
            debug!("[SOUND] Chiptune Pause");
            let p = packet::ChiptunePause { music: music, sound: sound };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn chiptune_resume(&mut self, music: i32, sound: i32) {
            debug!("[SOUND] Chiptune Resume");
            let p = packet::ChiptuneResume { music: music, sound: sound };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
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
