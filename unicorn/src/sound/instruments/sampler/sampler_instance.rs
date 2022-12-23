use std::sync::Arc;

use gamercade_audio::{IndexInterpolatorResult, SampleBitDepth, SampleDefinition};

use crate::{ActiveState, EnvelopeInstance, SampleOscillator};

#[derive(Debug, Clone)]
pub struct SamplerInstance {
    pub oscillator: SampleOscillator,
    definition: Arc<SampleDefinition>,
    active: ActiveState,
    envelope: EnvelopeInstance,
}

impl SamplerInstance {
    pub fn new(definition: &Arc<SampleDefinition>, output_sample_rate: usize) -> Self {
        Self {
            oscillator: SampleOscillator::from_definition(definition, output_sample_rate),
            definition: definition.clone(),
            active: ActiveState::Off,
            envelope: EnvelopeInstance::new(&definition.envelope_definition, output_sample_rate),
        }
    }

    /// Get's the current sample value
    /// This interpolates if necessary.
    /// Also increments the oscillator
    pub fn tick(&mut self) -> f32 {
        if let Some(index) = self.oscillator.tick() {
            let indices = self.oscillator.get_interpolated_indices(index);

            let output = match indices {
                IndexInterpolatorResult::Single(index) => {
                    self.definition.data[index] as f32 / SampleBitDepth::MAX as f32
                }
                IndexInterpolatorResult::Multiple(indices) => {
                    indices.into_iter().fold(0.0, |val, (index, scaling)| {
                        val + ((self.definition.data[index] as f32 / SampleBitDepth::MAX as f32)
                            * scaling)
                    })
                }
            };

            let envelope = self.envelope.tick(self.active);

            if ActiveState::Trigger == self.active {
                self.active = ActiveState::Off;
            }

            output * envelope
        } else {
            0.0
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(Some(frequency))
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = if active {
            ActiveState::On
        } else {
            ActiveState::Off
        };
        self.oscillator.reset();
    }

    pub fn trigger(&mut self) {
        self.active = ActiveState::Trigger;
        self.oscillator.reset();
    }
}
