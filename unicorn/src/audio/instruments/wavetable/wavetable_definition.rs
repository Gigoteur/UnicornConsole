use serde::{Deserialize, Serialize};

use super::WavetableBitDepth;
use audio::envelope_definition::EnvelopeDefinition;
use audio::instruments::index_interpolator::IndexInterpolator;
use audio::instruments::{
    de_audio_data, ser_audio_data,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavetableDefinition {
    #[serde(serialize_with = "ser_audio_data", deserialize_with = "de_audio_data")]
    pub data: Box<[WavetableBitDepth]>,
    pub envelope: EnvelopeDefinition,
    pub interpolator: IndexInterpolator,
}

impl Default for WavetableDefinition {
    fn default() -> Self {
        Self {
            data: vec![0].into_boxed_slice(),
            envelope: Default::default(),
            interpolator: IndexInterpolator::default(),
        }
    }
}

impl WavetableDefinition {
    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
