use std::{iter::Cycle, ops::Range, sync::Arc};

use eframe::egui::Ui;


use unicorn::audio::tracker::chain::ChainId;
use unicorn::audio::sound_rom::Sfx;
use unicorn::audio::consts::SFX_CHANNELS;

use unicorn::sound::sound_engine::{SoundEngine, SoundEngineChannelType, SoundEngineData};
use unicorn::sound::sound_rom_instance::SoundRomInstance;

use crate::editor::editor_sounds_data::EditorSoundData;

use super::{
    AudioEditorHelp, ChainEditor, InstrumentEditor, Oscilloscope, OscilloscopeMode, PhraseEditor,
    SfxEditor, SongEditor,
};

pub struct AudioEditor {
    pub(crate) mode: AudioEditorMode,
    chain_editor: ChainEditor,
    instrument_editor: InstrumentEditor,
    phrase_editor: PhraseEditor,
    song_editor: SongEditor,
    sfx_editor: SfxEditor,

    sound_engine: SoundEngine,
    pub(crate) audio_sync_helper: AudioSyncHelper,

    audio_editor_help: AudioEditorHelp,
    oscilloscope: Oscilloscope,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AudioEditorMode {
    Instrument,
    Sfx,
    Songs,
    Chains,
    Phrases,
}

impl AudioEditor {
    pub(crate) fn new(data: &EditorSoundData) -> Self {
        let sound_rom_instance = Arc::new(SoundRomInstance::from(data));
        let mut sound_engine = SoundEngine::new(60, &sound_rom_instance, 64);

        let sound_engine_data =
            SoundEngineData::new(sound_engine.output_sample_rate(), &sound_rom_instance);

        let (producer, consumer) = rtrb::RingBuffer::new(sound_engine.output_sample_rate());

        sound_engine.send(SoundEngineChannelType::UpdateOutputProducer(Some(producer)));

        Self {
            mode: AudioEditorMode::Instrument,
            chain_editor: ChainEditor::default(),
            instrument_editor: InstrumentEditor::default(),
            phrase_editor: PhraseEditor::default(),
            song_editor: SongEditor::default(),
            sfx_editor: SfxEditor::default(),
            sound_engine,
            audio_sync_helper: AudioSyncHelper {
                sync_rom: false,
                sound_engine_data,
                channel_ticker: (0..SFX_CHANNELS).cycle(),
                command_queue: Vec::new(),
            },
            oscilloscope: Oscilloscope::new(consumer),
            audio_editor_help: AudioEditorHelp::default(),
        }
    }
}

pub(crate) enum AudioSyncCommand {
    PressedKey {
        note_index: usize,
        instrument_index: usize,
        channel: usize,
    },
    ReleasedKey {
        channel: usize,
    },
    TriggerNote {
        note_index: usize,
        instrument_index: usize,
    },
    PlayPhrase {
        phrase_index: usize,
        target_bpm: f32,
    },
    PlaySfx(Sfx),
    StopSfx,
    PlayBgm(usize),
    StopBgm,
}

pub(crate) struct AudioSyncHelper {
    sync_rom: bool,
    pub(crate) sound_engine_data: SoundEngineData,
    channel_ticker: Cycle<Range<usize>>,
    command_queue: Vec<AudioSyncCommand>,
}

impl AudioSyncHelper {
    pub(crate) fn notify_rom_changed(&mut self) {
        self.sync_rom = true;
    }

    pub(crate) fn play_note(&mut self, note_index: usize, instrument_index: usize) -> usize {
        let channel = self.channel_ticker.next().unwrap();
        self.command_queue.push(AudioSyncCommand::PressedKey {
            note_index,
            instrument_index,
            channel,
        });
        channel
    }

    pub(crate) fn play_phrase(&mut self, phrase_index: usize, target_bpm: f32) {
        self.command_queue.push(AudioSyncCommand::PlayPhrase {
            phrase_index,
            target_bpm,
        });
    }

    pub(crate) fn stop_note(&mut self, channel: usize) {
        self.command_queue
            .push(AudioSyncCommand::ReleasedKey { channel })
    }

    pub(crate) fn trigger_note(&mut self, note_index: usize, instrument_index: usize) {
        self.command_queue.push(AudioSyncCommand::TriggerNote {
            note_index,
            instrument_index,
        })
    }

    pub(crate) fn play_chain(&mut self, chain_id: usize, bpm: f32) {
        self.command_queue.push(AudioSyncCommand::PlaySfx(Sfx {
            bpm,
            chain: ChainId(chain_id),
        }))
    }

    pub(crate) fn play_sfx(&mut self, sfx: Sfx) {
        self.command_queue.push(AudioSyncCommand::PlaySfx(sfx))
    }

    pub(crate) fn stop_sfx(&mut self) {
        self.command_queue.push(AudioSyncCommand::StopSfx)
    }

    pub(crate) fn play_bgm(&mut self, song_id: usize) {
        self.command_queue.push(AudioSyncCommand::PlayBgm(song_id))
    }

    pub(crate) fn stop_bgm(&mut self) {
        self.command_queue.push(AudioSyncCommand::StopBgm)
    }

    fn push_commands(&mut self, engine: &mut SoundEngine, data: &EditorSoundData) {
        if self.sync_rom {
            self.sync_rom = false;

            let new_instance = Arc::new(SoundRomInstance::from(data));
            self.sound_engine_data
                .replace_sound_rom_instance(&new_instance);
            engine.send(SoundEngineChannelType::SoundRomInstance(new_instance));
        }

        self.command_queue
            .drain(..)
            .for_each(|command| match command {
                AudioSyncCommand::PressedKey {
                    note_index,
                    instrument_index,
                    channel,
                } => engine.send(SoundEngineChannelType::PianoKeyPressed {
                    note_index,
                    instrument_index,
                    channel,
                }),
                AudioSyncCommand::ReleasedKey { channel } => {
                    engine.send(SoundEngineChannelType::PianoKeyReleased { channel })
                }
                AudioSyncCommand::TriggerNote {
                    note_index,
                    instrument_index,
                } => engine.send(SoundEngineChannelType::TriggerNote {
                    note_index,
                    instrument_index,
                    channel: self.channel_ticker.next().unwrap(),
                }),
                AudioSyncCommand::PlayPhrase {
                    phrase_index,
                    target_bpm,
                } => engine.send(SoundEngineChannelType::PlayPhrase {
                    phrase_index,
                    target_bpm,
                }),
                AudioSyncCommand::PlaySfx(sfx) => engine.send(SoundEngineChannelType::PlaySfx(sfx)),
                AudioSyncCommand::StopSfx => engine.send(SoundEngineChannelType::StopSfx),
                AudioSyncCommand::PlayBgm(song) => {
                    engine.send(SoundEngineChannelType::PlayBgm(song))
                }
                AudioSyncCommand::StopBgm => engine.send(SoundEngineChannelType::StopBgm),
            });
    }
}

impl AudioEditor {
    pub fn draw_selector(&mut self, ui: &mut Ui) {
        ui.selectable_value(&mut self.mode, AudioEditorMode::Instrument, "Instruments");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Phrases, "Phrases");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Chains, "Chains");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Songs, "Songs");
        ui.selectable_value(&mut self.mode, AudioEditorMode::Sfx, "Sfx");

        ui.separator();

        let editor_help_open = self.audio_editor_help.open;
        ui.selectable_value(&mut self.audio_editor_help.open, !editor_help_open, "Help!");

        ui.separator();

        ui.label("Oscilloscope:");
        if ui
            .selectable_value(&mut self.oscilloscope.mode, OscilloscopeMode::Off, "Off")
            .clicked()
        {
            self.oscilloscope.open = false;
        };
        if ui
            .selectable_value(
                &mut self.oscilloscope.mode,
                OscilloscopeMode::Channels,
                "Channels",
            )
            .clicked()
        {
            self.oscilloscope.open = true
        };
        if ui
            .selectable_value(
                &mut self.oscilloscope.mode,
                OscilloscopeMode::Master,
                "Master",
            )
            .clicked()
        {
            self.oscilloscope.open = true
        };

        self.audio_editor_help.draw(ui);
        self.oscilloscope.draw(ui);
    }

    pub fn draw_contents(&mut self, ui: &mut Ui, data: &mut EditorSoundData) {
        match self.mode {
            AudioEditorMode::Instrument => {
                self.instrument_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
            AudioEditorMode::Sfx => self.sfx_editor.draw(ui, data, &mut self.audio_sync_helper),
            AudioEditorMode::Songs => self.song_editor.draw(ui, data, &mut self.audio_sync_helper),
            AudioEditorMode::Chains => {
                self.chain_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
            AudioEditorMode::Phrases => {
                self.phrase_editor
                    .draw(ui, data, &mut self.audio_sync_helper)
            }
        };

        self.audio_sync_helper
            .push_commands(&mut self.sound_engine, data);
    }

    pub fn draw_bottom_panel(&mut self, ui: &mut Ui) {
        //TODO: Write this
        ui.label("TODO!");
    }
}
