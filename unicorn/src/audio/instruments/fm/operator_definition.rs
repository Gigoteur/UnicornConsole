use serde::{Deserialize, Serialize};

use crate::audio::instruments::fm::frequency_multiplier::FrequencyMultiplier;
use crate::audio::instruments::fm::detune::Detune;
use crate::audio::envelope_definition::{EnvelopeDefinition, EnvelopeValue};
use crate::audio::instruments::fm::fm_waveform::FMWaveform;
use crate::audio::instruments::index_interpolator::IndexInterpolator;
use crate::audio::instruments::fm::OPERATOR_COUNT;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorDefinitionBundle {
    pub operators: [OperatorDefinition; OPERATOR_COUNT],
}

impl Default for OperatorDefinitionBundle {
    fn default() -> Self {
        let modulators_envelope = EnvelopeDefinition {
            total_level: EnvelopeValue::zero(),
            ..Default::default()
        };

        let silent_modulator = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: Detune(0),
            envlope_definition: modulators_envelope,
            interpolator: IndexInterpolator::default(),
        };

        let modulator_envelope = EnvelopeDefinition {
            total_level: EnvelopeValue(170), // Random value to compare against audio-test
            ..EnvelopeDefinition::interesting()
        };

        let modulator = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: Detune(0),
            envlope_definition: modulator_envelope,
            interpolator: IndexInterpolator::default(),
        };

        let carrier = OperatorDefinition {
            waveform: FMWaveform::Sine,
            frequency_multiplier: FrequencyMultiplier::one(),
            detune: Detune(0),
            envlope_definition: EnvelopeDefinition::interesting(),
            interpolator: IndexInterpolator::default(),
        };

        Self {
            operators: [
                silent_modulator.clone(),
                silent_modulator,
                modulator,
                carrier,
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OperatorDefinition {
    pub waveform: FMWaveform,
    pub frequency_multiplier: FrequencyMultiplier,
    pub detune: Detune,
    pub envlope_definition: EnvelopeDefinition,
    pub interpolator: IndexInterpolator,
}
