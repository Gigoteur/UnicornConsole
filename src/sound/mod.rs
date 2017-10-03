pub mod sound {
    use std::sync::mpsc;
    use px8::packet;

    use chiptune;

    use std::collections::HashMap;
    use sdl2;
    use std::sync::{Arc, Mutex};

    pub struct SoundInternal {
        pub player: chiptune::Chiptune,
        pub chiptune_music_tracks: HashMap<String, chiptune::ChiptuneSong>,
        pub chiptune_sound_tracks: HashMap<String, chiptune::ChiptuneSound>,
        pub csend: mpsc::Sender<Vec<u8>>,
        pub crecv: mpsc::Receiver<Vec<u8>>,
    }

    impl SoundInternal {
        pub fn new() -> SoundInternal {
            let (csend, crecv) = mpsc::channel();

            SoundInternal {
                player: chiptune::Chiptune::new(),
                chiptune_music_tracks: HashMap::new(),
                chiptune_sound_tracks: HashMap::new(),
                csend: csend,
                crecv: crecv,
            }
        }

        pub fn init(&mut self) {
        }

        pub fn pause(&mut self) {
            info!("[SOUND] Pause");
            self.player.pause(1);
        }

        pub fn resume(&mut self) {
            info!("[SOUND] Resume");
            self.player.pause(0);
        }

        pub fn stop(&mut self) {
            info!("[SOUND] Stop");
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
                                    self.player.play_sound(&mut sound, res.channel, 13312, chiptune::CYD_PAN_CENTER, 50);
                                }
                                None => {},
                            }
                        }
                    }
                    packet::Packet::ChiptuneStop(res) => {
                        self.player.stop();
                    }
                    packet::Packet::ChiptunePause(res) => {
                        self.player.pause(1);
                    }
                    packet::Packet::ChiptuneResume(res) => {
                        self.player.pause(0);
                    }
                    packet::Packet::ChiptuneVolume(res) => {
                        self.player.set_volume(res.volume);
                    }
                }
            }

            sound.lock().unwrap().chiptune_position = self.player.get_music_position();
        }
    }

    pub struct Sound {
        csend: mpsc::Sender<Vec<u8>>,
        chiptune_position: i32,
    }

    impl Sound {
        pub fn new(csend: mpsc::Sender<Vec<u8>>) -> Sound {
            Sound {
                csend: csend,
                chiptune_position: 0,
            }
        }

        // Chiptune
        pub fn chiptune_play(&mut self, filetype: i32, channel: i32, filename: String, loops: i32, start_position: i32) {
            debug!("[SOUND] Chiptune PLAY {:?}", filename);
            let p = packet::ChiptunePlay { filetype: filetype, channel: channel, filename: filename, loops: loops, start_position: start_position };
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

        pub fn chiptune_volume(&mut self, volume: i32) {
            debug!("[SOUND] Chiptune volume");
            let p = packet::ChiptuneVolume { volume: volume };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn chiptune_get_position(&mut self) -> i32 {
            self.chiptune_position
        }
    }
}
