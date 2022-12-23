use std::f32::consts::TAU;

use audio::instruments::wavetable::wavetable_waveform::WavetableWaveform;

use super::{WavetableBitDepth};

/// Use to generate wavetables based on predetermined conditions
#[derive(Debug, Clone, Default)]
pub struct WavetableGenerator {
    /// The kind of waveform to generate.
    pub waveform: WavetableWaveform,

    /// The length of the table of a single cycle of the waveform.
    pub size: usize,
}

impl WavetableGenerator {
    /// Generates the wavetable's data based on the passed in parameters
    pub fn generate(&self) -> Box<[WavetableBitDepth]> {
        (0..self.size)
            .map(|index| {
                let value = (TAU * index as f32) / self.size as f32;
                let value = self.waveform.func(value);
                let value = value * WavetableBitDepth::MAX as f32;
                value as WavetableBitDepth
            })
            .collect::<Vec<WavetableBitDepth>>()
            .into_boxed_slice()
    }
}
