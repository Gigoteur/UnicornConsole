use crate::audio::instruments::index_interpolator::{IndexInterpolator, IndexInterpolatorResult};
use crate::audio::instruments::sampler::loop_mode::LoopMode;
use crate::audio::instruments::sampler::sample_definition::SampleDefinition;

#[derive(Debug, Clone)]
pub struct SampleOscillator {
    // Related to the sample itself
    sample_frequency: Option<f32>,
    input_sample_rate: usize,
    pub output_sample_rate: usize,
    interpolator: IndexInterpolator,

    // For sound playback
    index: f32,
    index_increment: f32,
    table_length: usize,
    loop_mode: LoopMode,
}

impl SampleOscillator {
    /// Generates a new SampleOscillator with the default value.
    pub(crate) fn new(
        output_sample_rate: usize,
        interpolator: IndexInterpolator,
        loop_mode: LoopMode,
    ) -> Self {
        Self {
            sample_frequency: None,
            input_sample_rate: output_sample_rate,
            output_sample_rate,
            index: 0.0,
            index_increment: 0.0,
            table_length: 1,
            interpolator,
            loop_mode,
        }
    }

    pub(crate) fn from_definition(
        definition: &SampleDefinition,
        output_sample_rate: usize,
    ) -> Self {
        let mut out = Self::new(
            output_sample_rate,
            definition.interpolator,
            definition.loop_mode.clone(),
        );
        out.set_sample(definition);
        out
    }

    /// Sets the frequency of the oscillator. If passed
    /// a None, it will revert the play rate back to the default one of the sample.
    pub(crate) fn set_frequency(&mut self, frequency: Option<f32>) {
        let sample_rate_ratio = self.input_sample_rate as f32 / self.output_sample_rate as f32;

        if let (Some(base_frequency), Some(new_frequency)) = (self.sample_frequency, frequency) {
            self.index_increment = (new_frequency / base_frequency) * sample_rate_ratio
        } else {
            self.index_increment = sample_rate_ratio;
        }
    }

    /// Sets this oscillator to match the requirements of the passed
    /// in sample.
    pub(crate) fn set_sample(&mut self, sample: &SampleDefinition) {
        self.sample_frequency = sample.sample_frequency;
        self.input_sample_rate = sample.source_sample_rate;
        self.index = 0.0;
        self.index_increment = sample.source_sample_rate as f32 / self.output_sample_rate as f32;
        self.table_length = sample.data.len();
    }

    /// Returns the index, then
    /// Increments the oscillator by its predefined amount
    /// Also handles any looping logic
    pub(crate) fn tick(&mut self) -> Option<f32> {
        let out = self.index;
        self.index += self.index_increment;

        match &self.loop_mode {
            LoopMode::Oneshot => {
                if self.index > self.table_length as f32 {
                    return None;
                }
            }
            LoopMode::Loop => self.index %= self.table_length as f32,
            LoopMode::LoopRange(range) => {
                if self.index > range.end as f32 {
                    self.index = range.start as f32 + self.index.fract();
                }
            }
        }
        Some(out)
    }

    pub(crate) fn get_interpolated_indices(&self, index: f32) -> IndexInterpolatorResult {
        if let LoopMode::LoopRange(range) = &self.loop_mode {
            if self.index > range.start as f32 {
                let index = self.index - range.start as f32;

                let mut out = self
                    .interpolator
                    .get_indices(index, range.end - range.start);

                match &mut out {
                    IndexInterpolatorResult::Single(val) => *val += range.start,
                    IndexInterpolatorResult::Multiple(values) => values
                        .iter_mut()
                        .for_each(|(index, _)| *index += range.start),
                };

                return out;
            }
        }
        self.interpolator.get_indices(index, self.table_length)
    }

    /// Resets the index back to zero. Useful when retriggering the sample
    pub(crate) fn reset(&mut self) {
        self.index = 0.0;
    }
}
