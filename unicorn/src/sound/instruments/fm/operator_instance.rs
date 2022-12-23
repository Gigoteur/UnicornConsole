use crate::audio::instruments::fm::fm_waveform::FMWaveform;
use crate::audio::instruments::index_interpolator::IndexInterpolatorResult;
use crate::audio::instruments::fm::operator_definition::{OperatorDefinition, OperatorDefinitionBundle};
use crate::audio::instruments::fm::OPERATOR_COUNT;

use crate::sound::instruments::ActiveState;
use crate::sound::envelope::envelope_instance::EnvelopeInstance;
use crate::sound::instruments::wavetable::wavetable_oscillator::WavetableOscillator;
use crate::sound::instruments::fm::LUT_FULL_LEN;

#[derive(Debug, Clone)]
pub struct OperatorInstance {
    pub oscillator: WavetableOscillator,
    envelope: EnvelopeInstance,
}

impl OperatorInstance {
    /// Constructs a new operator instance based on the passed in definition
    pub fn new(source: &OperatorDefinition, output_sample_rate: usize) -> Self {
        Self {
            oscillator: WavetableOscillator::new(
                LUT_FULL_LEN,
                output_sample_rate,
                source.interpolator,
            ),
            envelope: EnvelopeInstance::new(&source.envlope_definition, output_sample_rate),
        }
    }

    /// Sets the frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
    }

    /// Get's the current sample value including any modulation and
    /// interpolates between the next sample if necessary.
    /// Also ticks the operator.
    pub fn tick(&mut self, waveform: FMWaveform, modulation: f32, active: ActiveState) -> f32 {
        use crate::lookup;
        let mut index = self.oscillator.tick() + self.oscillator.modulation(modulation);

        if index.is_sign_negative() {
            let lut_len = LUT_FULL_LEN as f32;
            index = lut_len + (index % lut_len);
        }

        let indices = self.oscillator.get_interpolated_indices(index);

        let output = match indices {
            IndexInterpolatorResult::Single(index) => lookup(waveform, index),
            IndexInterpolatorResult::Multiple(indices) => {
                indices.into_iter().fold(0.0, |val, (index, scaling)| {
                    val + (lookup(waveform, index) * scaling)
                })
            }
        };

        let envelope = self.envelope.tick(active);

        output * envelope
    }
}

#[derive(Clone, Debug)]
pub struct OperatorInstanceBundle {
    pub operators: [OperatorInstance; OPERATOR_COUNT],
}

impl OperatorInstanceBundle {
    pub fn new(source: &OperatorDefinitionBundle, output_sample_rate: usize) -> Self {
        Self {
            operators: std::array::from_fn(|index| {
                OperatorInstance::new(&source.operators[index], output_sample_rate)
            }),
        }
    }
}
