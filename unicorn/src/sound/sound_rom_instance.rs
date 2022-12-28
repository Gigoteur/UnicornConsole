use std::{ops::Index, sync::Arc};

use crate::audio::tracker::chain::{Chain, ChainId};
use crate::audio::instruments::instrument_data_definition::InstrumentDataDefinition;
use crate::audio::instruments::instrument_data_definition::InstrumentId;
use crate::audio::instruments::fm::patch_definition::PatchDefinition;
use crate::audio::tracker::phrase::{Phrase, PhraseId};
use crate::audio::instruments::sampler::sample_definition::SampleDefinition;
use crate::audio::tracker::song::Song;
use crate::audio::sound_rom::SoundRom;

use crate::audio::sound_rom::Sfx;
use crate::audio::tracker::song::SongId;
use crate::audio::instruments::wavetable::wavetable_definition::WavetableDefinition;

/// An engine loaded in memory, ready to use.
#[derive(Debug)]
pub struct SoundRomInstance {
    pub songs: Box<[Song]>,
    pub chains: Box<[Option<Chain>]>,
    pub phrases: Box<[Option<Phrase>]>,
    pub instrument_bank: Box<[Option<InstrumentDefinition>]>,
    pub sfx: Box<[Sfx]>,
}

/// An instrument stored in memory, ready to generate the pieces
/// needed to produce sounds.
#[derive(Clone, Debug)]
pub struct InstrumentDefinition {
    pub id: usize,
    pub kind: InstrumentDefinitionKind,
}

#[derive(Clone, Debug)]
pub enum InstrumentDefinitionKind {
    Wavetable(Arc<WavetableDefinition>),
    FMSynth(Arc<PatchDefinition>),
    Sampler(Arc<SampleDefinition>),
}

impl From<InstrumentDataDefinition> for InstrumentDefinitionKind {
    fn from(data: InstrumentDataDefinition) -> Self {
        match data {
            InstrumentDataDefinition::Wavetable(wavetable_def) => {
                InstrumentDefinitionKind::Wavetable(Arc::new(wavetable_def))
            }
            InstrumentDataDefinition::FMSynth(fm_def) => {
                InstrumentDefinitionKind::FMSynth(Arc::new(fm_def))
            }
            InstrumentDataDefinition::Sampler(sample) => {
                InstrumentDefinitionKind::Sampler(Arc::new(sample))
            }
        }
    }
}

impl SoundRomInstance {
    /// Generates a new sound engine. This struct is used throughout the audio system.
    /// Performs some light logic to prepare the generation of sound sources.
    pub fn new(rom: &SoundRom) -> Self {
        Self {
            songs: rom.songs.clone(),
            chains: rom.chains.clone(),
            phrases: rom.phrases.clone(),
            instrument_bank: Vec::from(rom.instruments.clone())
                .into_iter()
                .enumerate()
                .map(|(index, instrument)| {
                    instrument.map(|instrument| InstrumentDefinition {
                        id: index,
                        kind: InstrumentDefinitionKind::from(instrument),
                    })
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            sfx: rom.sfx.clone(),
        }
    }
}

impl Index<SongId> for SoundRomInstance {
    type Output = Song;

    fn index(&self, index: SongId) -> &Self::Output {
        &self.songs[index.0]
    }
}

impl Index<ChainId> for SoundRomInstance {
    type Output = Option<Chain>;

    fn index(&self, index: ChainId) -> &Self::Output {
        if let Some(chain) = self.chains.get(index.0) {
            chain
        } else {
            &None
        }
    }
}

impl Index<PhraseId> for SoundRomInstance {
    type Output = Option<Phrase>;

    fn index(&self, index: PhraseId) -> &Self::Output {
        if let Some(phrase) = self.phrases.get(index.0) {
            phrase
        } else {
            &None
        }
    }
}

impl Index<InstrumentId> for SoundRomInstance {
    type Output = Option<InstrumentDefinition>;

    fn index(&self, index: InstrumentId) -> &Self::Output {
        if let Some(instrument) = self.instrument_bank.get(index.0) {
            instrument
        } else {
            &None
        }
    }
}
