use serde::{Deserialize, Serialize};

/// The integer type used to store envelopes.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct EnvelopeValue(pub EnvelopeValueType);
pub type EnvelopeValueType = u8;
pub const ENVELOPE_EXPONENTIAL_SCALING: i32 = 3;

impl EnvelopeValue {
    /// Returns a cubic scaled value
    pub fn to_scaled_value(self) -> f32 {
        let val = (self.0 as f32).powi(ENVELOPE_EXPONENTIAL_SCALING);
        let max = (EnvelopeValueType::MAX as f32).powi(ENVELOPE_EXPONENTIAL_SCALING);
        val / max
    }

    /// Returns a linear scaled value between 0 and MAX
    pub fn to_linear_value(self) -> f32 {
        let val = self.0 as f32;
        let max = EnvelopeValueType::MAX as f32;
        val / max
    }

    pub fn max() -> Self {
        Self(EnvelopeValueType::MAX)
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn is_max_value(self) -> bool {
        self.0 == EnvelopeValueType::MAX
    }
}

/// Definition of an Envelope. Controls the ADSR and volume levels.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EnvelopeDefinition {
    /// The max level
    pub total_level: EnvelopeValue,

    /// The level decayed to after the initial attack
    pub sustain_level: EnvelopeValue,

    /// The length of time to finish the attack phase. Between zero and total_level
    pub attack_time: EnvelopeValue,

    /// The length of time to finish the decay phase. Between total_level and sustain_level
    pub decay_attack_time: EnvelopeValue,

    /// The length of time to finish sustain. Between sustain_level and zero.
    pub decay_sustain_time: EnvelopeValue,

    /// The the length of decay after the key is released.
    pub release_time: EnvelopeValue,
}

impl Default for EnvelopeDefinition {
    fn default() -> Self {
        Self {
            total_level: EnvelopeValue::max(),
            sustain_level: EnvelopeValue::max(),

            attack_time: EnvelopeValue::zero(),
            decay_attack_time: EnvelopeValue::zero(),
            decay_sustain_time: EnvelopeValue::max(),
            release_time: EnvelopeValue::zero(),
        }
    }
}

impl EnvelopeDefinition {
    pub fn new(total_level: EnvelopeValue, sustain_level: EnvelopeValue, attack_time: EnvelopeValue, decay_attack_time: EnvelopeValue, decay_sustain_time: EnvelopeValue, release_time: EnvelopeValue) -> Self {
        Self {
            total_level: total_level,
            sustain_level: sustain_level,
            attack_time: attack_time,
            decay_attack_time: decay_attack_time,
            decay_sustain_time: decay_sustain_time,
            release_time: release_time
        }
    }
    pub fn always_on() -> Self {
        Self {
            total_level: EnvelopeValue::max(),
            sustain_level: EnvelopeValue::max(),

            attack_time: EnvelopeValue::zero(),
            decay_attack_time: EnvelopeValue::max(),
            decay_sustain_time: EnvelopeValue::max(),
            release_time: EnvelopeValue::max(),
        }
    }

    /// Silence
    pub fn silence() -> Self {
        Self {
            total_level: EnvelopeValue::zero(),
            sustain_level: EnvelopeValue::zero(),

            attack_time: EnvelopeValue::zero(),
            decay_attack_time: EnvelopeValue::zero(),
            decay_sustain_time: EnvelopeValue::zero(),
            release_time: EnvelopeValue::zero(),
        }
    }

    /// A slightly more interesting envelope compared to the default one.
    pub fn interesting() -> Self {
        Self {
            total_level: EnvelopeValue::max(),
            sustain_level: EnvelopeValue::max(),

            attack_time: EnvelopeValue(16),
            decay_attack_time: EnvelopeValue::zero(),
            decay_sustain_time: EnvelopeValue(64),
            release_time: EnvelopeValue(64),
        }
    }
}
