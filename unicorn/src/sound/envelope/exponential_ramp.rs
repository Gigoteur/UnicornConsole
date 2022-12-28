use crate::audio::envelope_definition::EnvelopeDefinition;

use crate::sound::envelope::envelope_phase::EnvelopePhase;
use crate::sound::envelope::envelope_instance::ENVELOPE_TIME_SCALE;

const OVERSHOOT: f32 = 1.001;

/// An exponential ramp which, when ticked, travels from one value to the target one.
#[derive(Clone, Debug)]
pub struct ExponentialRamp {
    output_sample_rate: usize,
    value: f32,              // The current value
    target_value: f32,       // The "end" value
    overshoot_value: f32,    // The "overshoot" value since we are dealing a small margin of error
    decaying_increment: f32, // The increment which changes over time
    multiplier: f32,         // The multiplier for the increment
    is_constant: bool,
}

impl ExponentialRamp {
    /// Generates a new exponential ramp with the default values of 0.
    pub fn new(output_sample_rate: usize) -> Self {
        Self {
            output_sample_rate,
            value: 0.0,
            target_value: 0.0,
            overshoot_value: 0.0,
            decaying_increment: 0.0,
            multiplier: 0.0,
            is_constant: true,
        }
    }

    /// Generates a new exponential ramp to be used with the ADSR envelope.
    pub(crate) fn set_from_envelope(
        &mut self,
        phase: EnvelopePhase,
        definition: &EnvelopeDefinition,
    ) {
        match phase {
            EnvelopePhase::Attack => self.ramp_to(
                definition.total_level.to_scaled_value(),
                definition.attack_time.to_scaled_value() * ENVELOPE_TIME_SCALE,
            ),
            EnvelopePhase::Decay => self.ramp_to(
                definition.sustain_level.to_linear_value()
                    * definition.total_level.to_scaled_value(),
                definition.decay_attack_time.to_scaled_value(),
            ),
            EnvelopePhase::Sustain => {
                if definition.decay_sustain_time.is_max_value() {
                    self.set_constant_value(
                        definition.sustain_level.to_linear_value()
                            * definition.total_level.to_scaled_value(),
                    )
                } else {
                    self.ramp_to(
                        0.0,
                        definition.decay_sustain_time.to_scaled_value() * ENVELOPE_TIME_SCALE,
                    )
                }
            }
            EnvelopePhase::Release => {
                if definition.release_time.is_max_value() {
                    self.set_constant_value(
                        definition.sustain_level.to_linear_value()
                            * definition.total_level.to_scaled_value(),
                    )
                } else {
                    self.ramp_to(
                        0.0,
                        definition.release_time.to_scaled_value() * ENVELOPE_TIME_SCALE,
                    )
                }
            }
            EnvelopePhase::Off => self.set_constant_value(0.0),
        };
    }

    /// Causes the ramp to hold at the passed in value
    pub fn set_constant_value(&mut self, new_value: f32) {
        self.value = new_value;
        self.target_value = new_value;
        self.overshoot_value = new_value;
        self.decaying_increment = 0.0;
        self.multiplier = 0.0;
        self.is_constant = true;
    }

    /// Sets the next target value for the ramp and how long it should take to get there.
    pub fn ramp_to(&mut self, target_value: f32, time: f32) {
        self.target_value = target_value;

        let distance_to_target = target_value - self.value;
        self.overshoot_value = self.value + (distance_to_target * OVERSHOOT);

        self.decaying_increment = self.value - self.overshoot_value;

        let time = (-1.0 * time) / (1.0 - OVERSHOOT.recip()).ln();
        self.multiplier = f32::powf(
            f32::exp(-1.0 / time),
            (self.output_sample_rate as f32).recip(),
        );

        self.is_constant = false;
    }

    /// Ticks the ramp, advancing it forward once and returing the resulting value.
    pub(crate) fn tick(&mut self) -> f32 {
        self.value = self.overshoot_value + self.decaying_increment;

        if !self.is_finished() {
            self.decaying_increment *= self.multiplier;
        }

        self.value
    }

    /// Returns true if the ramp is done advancing.
    pub fn is_finished(&self) -> bool {
        if self.is_constant {
            false
        } else {
            // Going up
            if self.value >= self.target_value && self.value <= self.overshoot_value {
                true
            } else {
                // Going Down
                self.value <= self.target_value && self.value >= self.overshoot_value
            }
        }
    }
}
