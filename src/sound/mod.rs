#[cfg(feature = "audio")]
pub mod song;

#[cfg(feature = "audio")]
#[allow(dead_code, unused_must_use, unused_variables)]
pub mod sound {
    use sdl2;
    use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpec, AudioDevice, AudioSpecWAV,
                      AudioCVT, AudioFormat};
    use std::sync::mpsc;
    use std::sync::mpsc::{Sender, Receiver};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    pub trait SoundPlayer: Send {
        fn get_samples(&mut self, sample_count: usize, result: &mut Vec<u8>) -> u32;
    }

    pub struct PX8Player {
        data: Vec<u8>,
        idx: u32,
        tx: Sender<Vec<u8>>,
        rx: Receiver<Vec<u8>>,
    }

    impl PX8Player {
        pub fn new(tx: Sender<Vec<u8>>, rx: Receiver<Vec<u8>>) -> PX8Player {
            PX8Player {
                data: Vec::new(),
                idx: 0,
                tx: tx,
                rx: rx,
            }
        }
    }

    unsafe impl Send for PX8Player {}

    impl SoundPlayer for PX8Player {
        fn get_samples(&mut self, sample_count: usize, result: &mut Vec<u8>) -> u32 {
            self.idx = 0;

            if let Ok(incoming_data) = self.rx.try_recv() {
                self.data.append(&mut incoming_data.clone());
            }

            if self.data.len() > 0 {
                for item in result.iter_mut() {
                    if self.data.len() > 0 {
                        *item = (self.data.remove(0) as f32 * 0.25) as u8;
                        self.idx += 1;
                    }
                }
            }
            self.idx
        }
    }

    struct Player<T: 'static + Send> {
        channel_count: usize,
        frame_size: usize,
        generator_buffer: Vec<u8>,
        generator: Box<SoundPlayer>,
        receiver: Receiver<T>,
    }

    impl<T> Player<T>
        where T: Send
    {
        fn new(spec: AudioSpec,
               buffer_size: usize,
               generator: Box<SoundPlayer>,
               receiver: Receiver<T>)
               -> Player<T> {
            Player {
                channel_count: spec.channels as usize,
                frame_size: buffer_size,
                generator_buffer: vec![0; buffer_size],
                generator: generator,
                receiver: receiver,
            }
        }
    }

    impl<T> AudioCallback for Player<T>
        where T: Send
    {
        type Channel = u8;
        /// Callback routine for SDL2
        fn callback(&mut self, out: &mut [u8]) {
            if self.generator
                   .get_samples(self.frame_size, &mut self.generator_buffer) > 0 {
                let mut idx = 0;
                for item in self.generator_buffer.iter().take(self.frame_size) {
                    for _ in 0..(self.channel_count) {
                        out[idx] = *item;
                        idx += 1;
                    }
                }
            }
        }
    }


    pub struct SoundInterface<T: 'static + Send> {
        sample_rate: u32,
        channel_count: u16,
        sdl_device: AudioDevice<Player<T>>,
        pub sender: Option<Sender<T>>,
        pub data_sender: Sender<Vec<u8>>,
    }

    impl<T> SoundInterface<T>
        where T: Send
    {
        pub fn new(sdl_context: sdl2::Sdl,
                   sample_rate: u32,
                   buffer_size: usize,
                   channel_count: u16)
                   -> SoundInterface<T> {
            let sdl_audio_subsystem = sdl_context.audio().unwrap();

            let desired_spec = AudioSpecDesired {
                freq: Some(sample_rate as i32),
                channels: Some(channel_count as u8),
                samples: Some((buffer_size as u16) * channel_count),
            };

            let (tx, rx): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::channel();

            let sound_player = Box::new(PX8Player::new(tx.clone(), rx));

            let (sender, receiver) = ::std::sync::mpsc::channel();

            let sdl_device = sdl_audio_subsystem
                .open_playback(None,
                               &desired_spec,
                               |spec| Player::new(spec, buffer_size, sound_player, receiver))
                .unwrap();

            SoundInterface {
                sample_rate: sample_rate,
                channel_count: channel_count,
                sdl_device: sdl_device,
                sender: Some(sender),
                data_sender: tx.clone(),
            }
        }

        pub fn start(&mut self) {
            self.sdl_device.resume();
        }
    }

    pub struct Sound {
        pub sounds: HashMap<u32, Vec<u8>>,
        pub data_sender: Sender<Vec<u8>>,
    }

    impl Sound {
        pub fn new(data_sender: Sender<Vec<u8>>) -> Sound {
            Sound {
                sounds: HashMap::new(),
                data_sender: data_sender,
            }
        }

        pub fn load(&mut self, filename: String) -> i32 {
            info!("Load sound {:?}", filename);
            // SONG

            // WAV
            let wav = AudioSpecWAV::load_wav(filename.clone())
                .ok()
                .expect("Could not load test WAV file");

            let cvt = AudioCVT::new(wav.format,
                                    wav.channels,
                                    wav.freq,
                                    AudioFormat::U8,
                                    1,
                                    44100)
                    .ok()
                    .expect("Could not convert WAV file");

            let mut data = cvt.convert(wav.buffer().to_vec());
            info!("LOAD DATA {:?}", data.len());

            let length = self.sounds.len() as u32;
            self.sounds.insert(length, data);

            length as i32
        }

        pub fn play(&mut self, id_sound: u32) {
            info!("Play sound {:?}", id_sound);
            let data = (*self.sounds.get(&id_sound).unwrap()).clone();
            self.data_sender.send(data);
        }

        pub fn stop(&mut self, _id: u32) {}
    }
}

#[cfg(not(feature = "audio"))]
pub mod sound {
    use sdl2;
    use std::marker::PhantomData;

    pub struct Sound {}

    impl Sound {
        pub fn new(_data_sender: PhantomData<f32>) -> Sound {
            Sound {}
        }

        pub fn load(&mut self, _filename: String) -> i32 {
            0
        }
        pub fn play(&mut self, _id: u32) {}

        pub fn stop(&mut self, _id: u32) {}
    }

    pub struct SoundInterface<T: 'static + Send> {
        phantom: PhantomData<T>,
        pub data_sender: PhantomData<T>,
    }

    impl<T> SoundInterface<T>
        where T: Send
    {
        pub fn new(_sdl_context: sdl2::Sdl,
                   _sample_rate: u32,
                   _buffer_size: usize,
                   _channel_count: u16)
                   -> SoundInterface<T> {
            SoundInterface {
                phantom: PhantomData,
                data_sender: PhantomData,
            }
        }

        pub fn start(&mut self) {}
    }
}
