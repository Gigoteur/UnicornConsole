use std::{mem::MaybeUninit, sync::Arc};

use crate::audio::instruments::index_interpolator::IndexInterpolatorResult;
use crate::audio::instruments::wavetable::wavetable_definition::WavetableDefinition;
use crate::audio::instruments::wavetable::WavetableBitDepth;

use crate::sound::instruments::ActiveState;
use crate::sound::envelope::envelope_instance::EnvelopeInstance;
use crate::sound::instruments::wavetable::wavetable_oscillator::WavetableOscillator;

pub(crate) static mut NO_SOUND_DEFINITION: MaybeUninit<Arc<WavetableDefinition>> =
    MaybeUninit::uninit();

#[derive(Clone, Debug)]
pub struct WavetableInstance {
    definition: Arc<WavetableDefinition>,
    envelope: EnvelopeInstance,
    pub(crate) oscillator: WavetableOscillator,
    active: ActiveState,
}

impl WavetableInstance {
    pub fn no_sound(output_sample_rate: usize) -> Self {
        let definition = unsafe { NO_SOUND_DEFINITION.assume_init_ref().clone() };
        Self {
            envelope: EnvelopeInstance::no_sound(output_sample_rate),
            oscillator: WavetableOscillator::new(1, output_sample_rate, definition.interpolator),
            definition,
            active: ActiveState::Off,
        }
    }

    /// Generates a new WavetableOscilator
    pub fn new(definition: Arc<WavetableDefinition>, output_sample_rate: usize) -> Self {
        Self {
            envelope: EnvelopeInstance::new(&definition.envelope, output_sample_rate),
            oscillator: WavetableOscillator::new(
                definition.len(),
                output_sample_rate,
                definition.interpolator,
            ),
            definition,
            active: ActiveState::Off,
        }
    }

    /// Sets the frequency
    pub fn set_frequency(&mut self, frequency: f32) {
        self.oscillator.set_frequency(frequency);
    }

    /// Get's the current sample value
    /// This interpolates between the current index and the next index
    /// Also increments the oscillator
    pub fn tick(&mut self) -> f32 {
        let index = self.oscillator.tick();

        let indices = self.oscillator.get_interpolated_indices(index);

        let output = match indices {
            IndexInterpolatorResult::Single(index) => {
                self.definition.data[index] as f32 / WavetableBitDepth::MAX as f32
            }
            IndexInterpolatorResult::Multiple(indices) => {
                indices.into_iter().fold(0.0, |val, (index, scaling)| {
                    val + ((self.definition.data[index] as f32 / WavetableBitDepth::MAX as f32)
                        * scaling)
                })
            }
        };

        let envelope = self.envelope.tick(self.active);

        if ActiveState::Trigger == self.active {
            self.active = ActiveState::Off;
        }

        output * envelope
    }

    pub fn set_active(&mut self, active: bool) {
        self.active = if active {
            ActiveState::On
        } else {
            ActiveState::Off
        };
    }

    pub fn trigger(&mut self) {
        self.active = ActiveState::Trigger;
    }
}
