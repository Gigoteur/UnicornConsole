pub mod sound {
    use std::sync::mpsc;
    use px8::packet;
    use px8::PX8Cartridge;

    use chiptune;

    use std::sync::{Arc, Mutex};

    pub struct SoundInternal {
        pub player: chiptune::Chiptune,
        pub csend: mpsc::Sender<Vec<u8>>,
        pub crecv: mpsc::Receiver<Vec<u8>>,
    }

    impl SoundInternal {
        pub fn new() -> SoundInternal {
            let (csend, crecv) = mpsc::channel();

            SoundInternal {
                player: chiptune::Chiptune::new(),
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


        pub fn update(&mut self, cartridge: &mut PX8Cartridge, sound: Arc<Mutex<Sound>>) {
            for sound_packet in self.crecv.try_iter() {
                debug!("[SOUND] PACKET {:?}", sound_packet);
                match packet::read_packet(sound_packet).unwrap() {
                    packet::Packet::ChiptuneMusic(res) => {
                        let filename = res.filename.clone();
                        // New song -> Load it before
                        if !cartridge.music_tracks.contains_key(&filename) {
                                let song = self.player.load_music(filename.clone());
                                match song {
                                    Ok(chip_song) => {
                                        cartridge.music_tracks.insert(filename.clone(), chip_song);
                                    }

                                    Err(e) => error!("ERROR to load the music {:?}", e),
                                }
                        }
                        match cartridge.music_tracks.get_mut(&filename) {
                            Some(mut song) => {
                                self.player.play_music(&mut song, res.start_position);
                                self.player.set_looping(res.loops);
                            }
                            None => {},
                        }
                    }
                    packet::Packet::ChiptuneSFX(res) => {
                        let filename = res.filename.clone();

                        if !cartridge.sound_tracks.contains_key(&filename) {
                            let sound = self.player.load_sound(filename.clone());
                            match sound {
                                Ok(chip_sound) => {
                                    cartridge.sound_tracks.insert(filename.clone(), chip_sound);
                                }

                                Err(e) => error!("ERROR to load the song {:?}", e),
                            }
                        }
                        match cartridge.sound_tracks.get_mut(&filename) {
                            Some(mut sound) => {
                                self.player.play_sound(&mut sound, res.channel, res.note, res.panning, res.rate);
                            }
                            None => {},
                        }
                    }
                    packet::Packet::ChiptuneMusicState(res) => {
                        if res.stop {
                            self.player.stop();
                        } else if res.pause {
                            self.player.pause(1);
                        } else if res.resume {
                            self.player.pause(0);
                        }
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
        pub fn music(&mut self, id: i32, filename: String, channel: i32,  loops: i32, start_position: i32) {
            debug!("[SOUND] Chiptune Music PLAY {:?}", filename);
            let p = packet::ChiptuneMusic { id: id, channel: channel, filename: filename, loops: loops, start_position: start_position };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn sfx(&mut self, id: i32, filename: String, channel: i32, note: u16, panning: i32, rate: i32, loops: i32) {
            debug!("[SOUND] Chiptune SFX Play {:?}", id);
            let p = packet::ChiptuneSFX { id: id, 
                                          filename: filename,
                                          channel: channel,
                                          loops: loops,
                                          note: note,
                                          panning: panning,
                                          rate: rate };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn music_stop(&mut self) {
            debug!("[SOUND] Chiptune STOP");
            let p = packet::ChiptuneMusicState { stop: true, pause: false, resume: false };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn music_pause(&mut self) {
            debug!("[SOUND] Chiptune Pause");
            let p = packet::ChiptuneMusicState { stop: false, pause: true, resume: false };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn music_resume(&mut self) {
            debug!("[SOUND] Chiptune Resume");
            let p = packet::ChiptuneMusicState { stop: false, pause: false, resume: true };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn music_volume(&mut self, volume: i32) {
            debug!("[SOUND] Chiptune volume");
            let p = packet::ChiptuneVolume { volume: volume };
            self.csend.send(packet::write_packet(p).unwrap()).unwrap();
        }

        pub fn chiptune_get_position(&mut self) -> i32 {
            self.chiptune_position
        }
    }
}
