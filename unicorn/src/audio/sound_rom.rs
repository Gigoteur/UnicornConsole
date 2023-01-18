use std::ops::Index;

use serde::{Deserialize, Serialize};

use audio::tracker::chain::{Chain, ChainId};
use audio::envelope_definition::EnvelopeDefinition;
use audio::instruments::index_interpolator::IndexInterpolator;
use audio::instruments::instrument_data_definition::InstrumentDataDefinition;
use audio::instruments::instrument_data_definition::InstrumentId;
use audio::tracker::song::{Song, SongId};
use audio::tracker::phrase::Phrase;
use audio::instruments::wavetable::wavetable_definition::WavetableDefinition;
use audio::instruments::wavetable::wavetable_generator::WavetableGenerator;
use audio::instruments::wavetable::wavetable_waveform::WavetableWaveform;

use super::envelope_definition::EnvelopeValue;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundRom {
    pub songs: Box<[Song]>,
    pub chains: Box<[Option<Chain>]>,
    pub phrases: Box<[Option<Phrase>]>,
    pub instruments: Box<[Option<InstrumentDataDefinition>]>,
    pub sfx: Box<[Sfx]>,
}

/// Represents a singular sound effect
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Sfx {
    pub bpm: f32,
    pub chain: ChainId,
}

impl Default for SoundRom {
    fn default() -> Self {
        let default_sine_wave = InstrumentDataDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Sine,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
            interpolator: IndexInterpolator::Truncate,
        });

        let default_triangle_wave = InstrumentDataDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Triangle,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
            interpolator: IndexInterpolator::Truncate,
        });

        let default_noise_wave = InstrumentDataDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Noise,
                size: 64,
            }
            .generate(),
            envelope: EnvelopeDefinition::interesting(),
            interpolator: IndexInterpolator::Truncate,
        });

        let default_square_wave = InstrumentDataDefinition::Wavetable(WavetableDefinition {
            data: WavetableGenerator {
                waveform: WavetableWaveform::Square,
                size: 8,
            }
            .generate(),
            envelope: EnvelopeDefinition::new(EnvelopeValue(255), EnvelopeValue(255), EnvelopeValue(0), EnvelopeValue(0), EnvelopeValue(255), EnvelopeValue(32)),
            interpolator: IndexInterpolator::Truncate,
        });

        let default_phrase = Phrase::c_scale(InstrumentId(0));

        let default_chain = Chain::default();

        let default_sfx = Sfx {
            bpm: 120.0,
            chain: ChainId::default(),
        };

        Self {
            songs: vec![].into_boxed_slice(),
            chains: vec![Some(default_chain)].into_boxed_slice(),
            phrases: vec![Some(default_phrase)].into_boxed_slice(),
            instruments: vec![Some(default_sine_wave), Some(default_triangle_wave), Some(default_square_wave), Some(default_noise_wave)].into_boxed_slice(),
            sfx: vec![default_sfx].into_boxed_slice(),
        }
    }
}

impl Index<SongId> for SoundRom {
    type Output = Song;

    fn index(&self, index: SongId) -> &Self::Output {
        &self.songs[index.0]
    }
}
