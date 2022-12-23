use std::f32::consts::TAU;

use gamercade_audio::{IndexInterpolator, IndexInterpolatorResult};

use crate::FM_MODULATION;

/// A wavetable oscillator. Returns table indices.
#[derive(Debug, Clone)]
pub struct WavetableOscillator {
    index: f32,
    index_increment: f32,
    table_length: usize,
    pub(crate) output_sample_rate: usize,
    interpolator: IndexInterpolator,
}

impl WavetableOscillator {
    /// Generates a new WavetableOscillator with the default value.
    pub(crate) fn new(
        table_length: usize,
        output_sample_rate: usize,
        interpolator: IndexInterpolator,
    ) -> Self {
        Self {
            index: 0.0,
            index_increment: 0.0,
            table_length,
            output_sample_rate,
            interpolator,
        }
    }

    /// Sets the frequency of the oscillator
    pub(crate) fn set_frequency(&mut self, frequency: f32) {
        let increment = frequency * self.table_length as f32;
        self.index_increment = increment / self.output_sample_rate as f32;
    }

    // Returns the modulation amount for this oscillator. Used with FM Synth
    pub(crate) fn modulation(&self, modulation: f32) -> f32 {
        FM_MODULATION * modulation * self.table_length as f32 / TAU
    }

    /// Returns the index, then
    /// Increments the oscillator by its predefined amount
    pub(crate) fn tick(&mut self) -> f32 {
        let out = self.index;
        self.index += self.index_increment;
        self.index %= self.table_length as f32;
        out
    }

    pub(crate) fn get_interpolated_indices(&self, index: f32) -> IndexInterpolatorResult {
        self.interpolator.get_indices(index, self.table_length)
    }
}
