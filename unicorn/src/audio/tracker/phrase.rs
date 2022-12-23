use std::iter::FromIterator;

use arrayvec::ArrayVec;
use serde::{Deserialize, Serialize};

use super::effect::Effect;

use crate::audio::notes::note::{
    name_octave_to_index
};

use crate::audio::notes::note_name::NoteName;
use crate::audio::notes::octave::Octave;
use crate::audio::notes::note::NoteId;
use crate::audio::instruments::instrument_data_definition::InstrumentId;
use crate::audio::tracker::PhraseVolumeType;

use crate::audio::consts::{
    EFFECT_COUNT,
    PHRASE_MAX_ENTRIES,
};

/// Newtype Chain Identifier
#[derive(Debug, Copy, Clone, Serialize, Deserialize, Default)]
pub struct PhraseId(pub usize);

/// A phrase is a series of notes tied to instruments, which when combined together form a chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Phrase {
    pub entries: ArrayVec<Option<PhraseStorageType>, PHRASE_MAX_ENTRIES>,
}

impl Phrase {
    /// Generates a C scale. Useful for testing.
    pub fn c_scale(instrument: InstrumentId) -> Self {
        use std::array::from_fn;

        let notes = [
            (NoteName::C, Octave::Four),
            (NoteName::D, Octave::Four),
            (NoteName::E, Octave::Four),
            (NoteName::F, Octave::Four),
            (NoteName::G, Octave::Four),
            (NoteName::A, Octave::Five),
            (NoteName::B, Octave::Five),
            (NoteName::C, Octave::Five),
        ];
        let mut note_iter = notes.iter();

        Self {
            entries: ArrayVec::from(from_fn(|index| {
                if index % 2 == 0 {
                    let note = note_iter.next().unwrap();
                    Some(PhraseEntry {
                        note: name_octave_to_index(note.0, note.1).unwrap(),
                        volume: PhraseVolumeType::MAX,
                        instrument,
                        effects: from_fn(|_| None),
                    })
                } else {
                    None
                }
            })),
        }
    }

    /// Generates a reversed C scale. Useful for testing.
    pub fn c_scale_reverse(instrument: InstrumentId) -> Self {
        let reversed = Self::c_scale(instrument).entries.into_iter().rev();
        Self {
            entries: ArrayVec::from_iter(reversed),
        }
    }
}

impl Default for Phrase {
    fn default() -> Self {
        Self {
            entries: ArrayVec::from(std::array::from_fn(|_| None)),
        }
    }
}

pub type PhraseStorageType = PhraseEntry<NoteId, InstrumentId>;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An entry in the phrase, contains all data necessary to produce a sound
pub struct PhraseEntry<N, T> {
    pub note: N,
    pub volume: PhraseVolumeType,
    pub instrument: T,
    pub effects: [Option<Effect>; EFFECT_COUNT],
}

impl Default for PhraseStorageType {
    fn default() -> Self {
        Self {
            note: Default::default(),
            volume: PhraseVolumeType::MAX / 2,
            instrument: Default::default(),
            effects: Default::default(),
        }
    }
}
