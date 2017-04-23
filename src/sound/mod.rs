#[cfg(feature = "audio")]
pub mod sound {
    use std::collections::HashMap;

    use sdl2;
    use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioSpec, AudioDevice, AudioSpecWAV, AudioCVT, AudioFormat};
    use std::sync::mpsc::{Sender, Receiver};

    pub trait SoundPlayer: Send {
        fn get_samples(&mut self, sample_count: usize, result: &mut Vec<u8>);
    }

    pub struct PX8Player {
        data: Vec<u8>,
        idx: u32,
    }

    impl PX8Player {
        pub fn new() -> PX8Player {
            PX8Player {
                data: Vec::new(),
                idx: 0,
            }
        }
    }

    unsafe impl Send for PX8Player {}

    impl SoundPlayer for PX8Player {
        fn get_samples(&mut self, sample_count: usize, result: &mut Vec<u8>) {
            if self.data.len() > 0 {
                for item in result.iter_mut() {
                    *item = (self.data.remove(0) as f32 * 0.25) as u8;
                    self.idx += 1;
                }
            }
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
            self.generator.get_samples(self.frame_size, &mut self.generator_buffer);
            let mut idx = 0;
            for item in self.generator_buffer.iter().take(self.frame_size) {
                for _ in 0..(self.channel_count) {
                    out[idx] = *item;
                    idx += 1;
                }
            }
        }
    }


    pub struct SoundInterface<T: 'static + Send> {
        sample_rate: u32,
        channel_count: u16,
        sdl_device: AudioDevice<Player<T>>,
        sender: Option<Sender<T>>,
    }

    impl<T> SoundInterface<T>
        where T: Send {
        pub fn new(sdl_context: sdl2::Sdl,
                   sample_rate: u32,
                   buffer_size: usize,
                   channel_count: u16) -> SoundInterface<T> {
            let sdl_audio_subsystem = sdl_context.audio().unwrap();

            let desired_spec = AudioSpecDesired {
                freq: Some(sample_rate as i32),
                channels: Some(channel_count as u8),
                samples: Some((buffer_size as u16) * channel_count),
            };

            let sound_player = Box::new(PX8Player::new());

            let (sender, receiver) = ::std::sync::mpsc::channel();

            let sdl_device = sdl_audio_subsystem.open_playback(None,
                                                               &desired_spec,
                                                               |spec| Player::new(spec, buffer_size, sound_player, receiver)).unwrap();

            SoundInterface {
                sample_rate: sample_rate,
                channel_count: channel_count,
                sdl_device: sdl_device,
                sender: Some(sender),
            }
        }

        pub fn start(&mut self) {
            self.sdl_device.resume();
        }
    }

    pub struct Sound {}

    impl Sound {
        pub fn new() -> Sound {
            Sound {}
        }

        pub fn load(&mut self, filename: String) -> i32 {
            info!("Load sound {:?}", filename);
            // SONG

            // WAV
            let wav = AudioSpecWAV::load_wav(filename.clone())
                .ok()
                .expect("Could not load test WAV file");

            let cvt = AudioCVT::new(
                wav.format, wav.channels, wav.freq,
                AudioFormat::U8, 2, 44100)
                .ok()
                .expect("Could not convert WAV file");

            let data = cvt.convert(wav.buffer().to_vec());

            0
        }

        pub fn play(&mut self, id: u32) {}

        pub fn stop(&mut self, id: u32) {}
    }
}

#[cfg(not(feature = "audio"))]
pub mod sound {
    use sdl2;
    use std::marker::PhantomData;

    pub struct Sound {}

    impl Sound {
        pub fn new() -> Sound {
            Sound {}
        }

        pub fn load(&mut self, filename: String) -> i32 {
            0
        }
        pub fn play(&mut self, id: u32) {}

        pub fn stop(&mut self, id: u32) {}
    }

    pub struct SoundInterface<T: 'static + Send> {
        phantom: PhantomData<T>,
    }

    impl<T> SoundInterface<T>
        where T: Send {
        pub fn new(sdl_context: sdl2::Sdl,
                   sample_rate: u32,
                   buffer_size: usize,
                   channel_count: u16) -> SoundInterface<T> {
            SoundInterface {
                phantom: PhantomData,
            }
        }

        pub fn start(&mut self) {}
    }
}