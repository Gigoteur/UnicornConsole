use serde::{Deserialize, Serialize};

use audio::envelope_definition::EnvelopeDefinition;
use audio::instruments::index_interpolator::IndexInterpolator;
use audio::instruments::sampler::loop_mode::LoopMode;
use audio::instruments::sampler::SampleBitDepth;


use audio::instruments::{
    de_audio_data, ser_audio_data,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SampleDefinition {
    #[serde(serialize_with = "ser_audio_data", deserialize_with = "de_audio_data")]
    pub data: Box<[SampleBitDepth]>,
    pub source_sample_rate: usize,
    pub sample_frequency: Option<f32>,
    pub envelope_definition: EnvelopeDefinition,
    pub interpolator: IndexInterpolator,
    pub loop_mode: LoopMode,
}

impl Default for SampleDefinition {
    fn default() -> Self {
        Self {
            data: vec![0].into_boxed_slice(),
            source_sample_rate: 1,
            sample_frequency: Default::default(),
            envelope_definition: Default::default(),
            interpolator: IndexInterpolator::default(),
            loop_mode: LoopMode::Oneshot,
        }
    }
}

impl SampleDefinition {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
