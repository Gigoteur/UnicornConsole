use std::sync::Arc;

use cpal::{
    default_host,
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, SampleFormat, Stream, StreamConfig, SupportedStreamConfig,
};
use gamercade_audio::{InstrumentId, PhraseId};
use rtrb::{Consumer, Producer, RingBuffer};

use crate::{
    initialize_globals, ChainPlayback, InstrumentInstance, SfxPlayback, SongPlayback,
    SoundOutputChannels, SoundRomInstance,
};
pub use gamercade_audio::{Sfx, SongId, SFX_CHANNELS, SONG_TRACK_CHANNELS};

#[derive(Clone)]
pub struct SoundEngineData {
    pub bgm: SongPlayback,
    pub sfx: [SfxPlayback; SFX_CHANNELS],
    rom: Arc<SoundRomInstance>,
}

pub enum SoundEngineChannelType {
    SoundEngineData(Box<SoundEngineData>),
    SoundRomInstance(Arc<SoundRomInstance>),
    PianoKeyPressed {
        note_index: usize,
        instrument_index: usize,
        channel: usize,
    },
    PianoKeyReleased {
        channel: usize,
    },
    TriggerNote {
        note_index: usize,
        instrument_index: usize,
        channel: usize,
    },
    UpdateOutputProducer(Option<Producer<SoundOutputChannels>>),
    PlayPhrase {
        phrase_index: usize,
        target_bpm: f32,
    },
    PlaySfx(Sfx),
    StopSfx,
    PlayBgm(usize),
    StopBgm,
}

impl SoundEngineData {
    pub fn new(output_sample_rate: usize, rom: &Arc<SoundRomInstance>) -> Self {
        use std::array::from_fn;

        let bgm_tracks = from_fn(|_| {
            ChainPlayback::new(None, rom, InstrumentInstance::no_sound(output_sample_rate))
        });

        Self {
            bgm: SongPlayback::new(None, bgm_tracks, rom, output_sample_rate),
            sfx: from_fn(|_| {
                SfxPlayback::new(
                    None,
                    rom,
                    InstrumentInstance::no_sound(output_sample_rate),
                    output_sample_rate,
                )
            }),
            rom: rom.clone(),
        }
    }

    pub fn tick(&mut self) -> SoundOutputChannels {
        SoundOutputChannels {
            sfx_output: std::array::from_fn(|index| self.sfx[index].tick()),
            bgm_output: self.bgm.tick(),
        }
    }

    /// Sets the Bgm to be played. If None is passed in, bgm will be stopped.
    pub fn play_bgm(&mut self, song: Option<SongId>) {
        self.bgm.set_song_id(song);
    }

    /// Sets the Sfx to be played. If None is passed in, the sfx will be stopped.
    pub fn play_sfx(&mut self, sfx: Option<Sfx>, channel: usize) {
        self.sfx[channel].set_sfx_id(sfx);
    }

    pub fn play_note(&mut self, note: i32, instrument_index: usize, channel: usize) {
        let instrument = self.rom[InstrumentId(instrument_index)].as_ref();
        let channel = self.sfx.get_mut(channel);

        if let (Some(instrument), Some(channel)) = (&instrument, channel) {
            let target = &mut channel.chain_playback.phrase_playback.instrument;
            target.update_from_instrument(instrument);
            target.set_active(true);
            target.set_note(note);
        }
    }

    pub fn set_key_active(&mut self, active: bool, channel: usize) {
        if let Some(target) = self.sfx.get_mut(channel) {
            target
                .chain_playback
                .phrase_playback
                .instrument
                .set_active(active)
        }
    }

    pub fn trigger_note(&mut self, note: i32, instrument_index: usize, channel: usize) {
        let instrument = self.rom[InstrumentId(instrument_index)].as_ref();
        let channel = self.sfx.get_mut(channel);

        if let (Some(instrument), Some(channel)) = (&instrument, channel) {
            let target = &mut channel.chain_playback.phrase_playback.instrument;
            target.update_from_instrument(instrument);
            target.trigger();
            target.set_note(note);
        }
    }

    pub fn play_frequency(&mut self, frequency: f32, instrument_index: usize, channel: usize) {
        let instrument = self.rom[InstrumentId(instrument_index)].as_ref();
        let channel = self.sfx.get_mut(channel);

        if let (Some(instrument), Some(channel)) = (&instrument, channel) {
            let target = &mut channel.chain_playback.phrase_playback.instrument;
            target.update_from_instrument(instrument);
            target.set_active(true);
            target.set_frequency(frequency);
        }
    }

    pub(crate) fn fast_forward(&mut self, frames: usize) {
        (0..frames).for_each(|_| {
            self.bgm.tick();
            self.sfx.iter_mut().for_each(|sfx| {
                sfx.tick();
            });
        });
    }

    pub fn replace_sound_rom_instance(&mut self, new_rom: &Arc<SoundRomInstance>) {
        self.rom = new_rom.clone();

        self.bgm.replace_sound_rom_instance(new_rom);
        self.sfx
            .iter_mut()
            .for_each(|sfx| sfx.replace_sound_rom_instance(new_rom));
    }
}

pub struct SoundEngine {
    _stream: Stream,
    sound_frames_per_render_frame: usize,
    sound_thread_producer: Producer<SoundEngineChannelType>,
    output_sample_rate: usize,
}

impl SoundEngine {
    pub fn output_sample_rate(&self) -> usize {
        self.output_sample_rate
    }

    pub fn new(fps: usize, rom: &Arc<SoundRomInstance>, message_buffer_size: usize) -> Self {
        initialize_globals();
        let mut device = default_host().default_output_device().unwrap();

        let supported_config = device.default_output_config().unwrap();
        let output_sample_rate = supported_config.sample_rate().0 as usize;

        let (stream, producer) = SoundEngineRunner::initialize_stream(
            rom,
            &mut device,
            supported_config,
            message_buffer_size,
        );

        stream.play().unwrap();

        Self {
            sound_frames_per_render_frame: output_sample_rate / fps,
            output_sample_rate,
            _stream: stream,
            sound_thread_producer: producer,
        }
    }

    /// Fast-forwards the the SoundEngineData by generating one frame worth samples
    /// This keeps it somewhat in sync with the audio that's actually being played
    pub fn fast_forward(&mut self, data: &mut SoundEngineData) {
        data.fast_forward(self.sound_frames_per_render_frame);
    }

    pub fn sync_audio_thread(&mut self, data: &SoundEngineData) {
        self.sound_thread_producer
            .push(SoundEngineChannelType::SoundEngineData(Box::new(
                data.clone(),
            )))
            .unwrap()
    }

    pub fn send(&mut self, message: SoundEngineChannelType) {
        self.sound_thread_producer.push(message).unwrap();
    }
}

struct SoundEngineRunner {
    channels: usize,
    output_sample_rate: usize,
    consumer: Consumer<SoundEngineChannelType>,
    data: SoundEngineData,
    sound_output_producer: Option<Producer<SoundOutputChannels>>,
}

impl SoundEngineRunner {
    fn initialize_stream(
        rom: &Arc<SoundRomInstance>,
        device: &mut Device,
        config: SupportedStreamConfig,
        message_buffer_size: usize,
    ) -> (Stream, Producer<SoundEngineChannelType>) {
        let output_sample_rate = config.sample_rate().0 as usize;
        let channels = config.channels() as usize;

        let (producer, consumer) = RingBuffer::new(message_buffer_size);

        println!("Output Sample Rate: {}", output_sample_rate);
        println!("Output channels: {}", channels);

        let data = SoundEngineData::new(output_sample_rate, rom);

        (
            Self {
                channels,
                output_sample_rate,
                consumer,
                data,
                sound_output_producer: None,
            }
            .build_stream(device, config),
            producer,
        )
    }

    fn build_stream(self, device: &mut Device, config: SupportedStreamConfig) -> Stream {
        let sample_format = config.sample_format();
        let config = StreamConfig::from(config);

        match sample_format {
            SampleFormat::I16 => self.bind_output_stream::<i16>(device, config),
            SampleFormat::U16 => self.bind_output_stream::<u16>(device, config),
            SampleFormat::F32 => self.bind_output_stream::<f32>(device, config),
        }
    }

    fn bind_output_stream<T: cpal::Sample>(
        mut self,
        device: &mut Device,
        config: StreamConfig,
    ) -> Stream {
        let on_error = move |err| {
            // react to errors here.
            println!("{}", err);
        };

        device
            .build_output_stream(
                &config,
                move |frames: &mut [T], _: &cpal::OutputCallbackInfo| {
                    self.sound_engine_callback(frames);
                },
                on_error,
            )
            .unwrap()
    }

    fn sound_engine_callback<T: cpal::Sample>(&mut self, frames: &mut [T]) {
        let mut buffer_written = false;
        let data = &mut self.data;

        // Repeat indefinitely until we write a full buffer without
        // any new data inputs. If we receive a new data snapshot midway
        // during a buffer, it will cause some popping, so in this case
        // we need to just throw away whatever we have written and start again
        while !buffer_written {
            frames.chunks_exact_mut(self.channels).for_each(|frame| {
                while let Ok(next_data) = self.consumer.pop() {
                    match next_data {
                        SoundEngineChannelType::SoundEngineData(next_data) => {
                            *data = *next_data;
                            return;
                        }
                        SoundEngineChannelType::SoundRomInstance(new_rom) => {
                            data.replace_sound_rom_instance(&new_rom);
                        }
                        SoundEngineChannelType::PianoKeyPressed {
                            note_index,
                            instrument_index,
                            channel,
                        } => data.play_note(note_index as i32, instrument_index, channel),
                        SoundEngineChannelType::PianoKeyReleased { channel } => {
                            data.set_key_active(false, channel)
                        }
                        SoundEngineChannelType::TriggerNote {
                            note_index,
                            instrument_index,
                            channel,
                        } => data.trigger_note(note_index as i32, instrument_index, channel),
                        SoundEngineChannelType::UpdateOutputProducer(new_producer) => {
                            self.sound_output_producer = new_producer
                        }
                        SoundEngineChannelType::PlayPhrase {
                            phrase_index,
                            target_bpm,
                        } => {
                            let phrase = Some(PhraseId(phrase_index));
                            data.sfx[0].set_sfx_id(None);
                            data.sfx[0].oscillator.reset_bpm(target_bpm);
                            let phrase_playback = &mut data.sfx[0].chain_playback.phrase_playback;

                            // Reset the instrument to force a refresh
                            phrase_playback.instrument =
                                InstrumentInstance::no_sound(self.output_sample_rate);
                            phrase_playback.set_phrase_id(phrase);
                        }
                        SoundEngineChannelType::PlaySfx(sfx) => {
                            data.play_sfx(Some(sfx), 0);
                        }
                        SoundEngineChannelType::StopSfx => data.play_sfx(None, 0),
                        SoundEngineChannelType::PlayBgm(bgm) => {
                            // Force a refresh of all instruments
                            data.bgm.tracks.iter_mut().for_each(|track| {
                                track.phrase_playback.instrument =
                                    InstrumentInstance::no_sound(self.output_sample_rate);
                            });

                            data.play_bgm(Some(SongId(bgm)));
                        }
                        SoundEngineChannelType::StopBgm => data.play_bgm(None),
                    };
                }

                let output = data.tick();

                if let Some(sound_output_producer) = &mut self.sound_output_producer {
                    if !sound_output_producer.is_full() {
                        sound_output_producer.push(output.clone()).unwrap();
                    }
                }

                let bgm_frame = output.get_bgm_output();
                let sfx_frame = output.get_sfx_output();
                let output = (bgm_frame + sfx_frame) / (SFX_CHANNELS + SONG_TRACK_CHANNELS) as f32;

                frame.iter_mut().for_each(|channel| {
                    *channel = cpal::Sample::from::<f32>(&output);
                });
            });

            buffer_written = true;
        }
    }
}
