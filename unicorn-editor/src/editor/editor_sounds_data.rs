use unicorn::audio::instruments::instrument_data_definition::InstrumentDataDefinition;
use unicorn::audio::tracker::chain::Chain;
use unicorn::audio::tracker::phrase::Phrase;

use unicorn::audio::sound_rom::Sfx;
use unicorn::audio::tracker::song::Song;
use unicorn::audio::sound_rom::SoundRom;

use unicorn::sound::sound_rom_instance::{InstrumentDefinition, InstrumentDefinitionKind};
use unicorn::sound::sound_rom_instance::SoundRomInstance;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSoundData {
    pub songs: Vec<EditorAudioDataEntry<Song>>,
    pub chains: Vec<EditorAudioDataEntry<Option<Chain>>>,
    pub phrases: Vec<EditorAudioDataEntry<Option<Phrase>>>,
    pub instruments: Vec<EditorAudioDataEntry<Option<InstrumentDataDefinition>>>,
    pub sfx: Vec<EditorAudioDataEntry<Sfx>>,
}

impl Default for EditorSoundData {
    fn default() -> Self {
        let sound_rom = SoundRom::default();
        Self {
            songs: from_rom(&sound_rom.songs, "Song"),
            chains: from_rom(&sound_rom.chains, "Chain"),
            phrases: from_rom(&sound_rom.phrases, "Phrase"),
            instruments: from_rom(&sound_rom.instruments, "Instrument"),
            sfx: from_rom(&sound_rom.sfx, "Sfx"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorAudioDataEntry<T> {
    pub name: String,
    pub data: T,
}

fn extract_data<T: Clone>(target: &[EditorAudioDataEntry<T>]) -> Box<[T]> {
    target
        .iter()
        .map(|x| x.data.clone())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn from_rom<T: Clone>(target: &[T], name: &str) -> Vec<EditorAudioDataEntry<T>> {
    target
        .iter()
        .enumerate()
        .map(|(index, item)| EditorAudioDataEntry {
            name: format!("{} {}", name, index),
            data: item.clone(),
        })
        .collect::<Vec<_>>()
}

impl From<&EditorSoundData> for SoundRom {
    fn from(data: &EditorSoundData) -> Self {
        Self {
            songs: extract_data(&data.songs),
            chains: extract_data(&data.chains),
            phrases: extract_data(&data.phrases),
            instruments: extract_data(&data.instruments),
            sfx: extract_data(&data.sfx),
        }
    }
}

impl From<&EditorSoundData> for SoundRomInstance {
    fn from(data: &EditorSoundData) -> Self {
        Self {
            songs: extract_data(&data.songs),
            chains: extract_data(&data.chains),
            phrases: extract_data(&data.phrases),
            instrument_bank: data
                .instruments
                .iter()
                .enumerate()
                .map(|(id, instrument)| {
                    instrument
                        .data
                        .as_ref()
                        .map(|instrument| InstrumentDefinition {
                            id,
                            kind: InstrumentDefinitionKind::from(instrument.clone()),
                        })
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            sfx: extract_data(&data.sfx),
        }
    }
}