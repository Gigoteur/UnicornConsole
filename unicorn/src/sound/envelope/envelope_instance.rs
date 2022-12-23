use sound::instruments::ActiveState;
use sound::envelope::envelope_phase::EnvelopePhase;
use sound::envelope::exponential_ramp::ExponentialRamp;
use crate::audio::envelope_definition::{EnvelopeDefinition, EnvelopeValue};

/// Max length in seconds, ~4.267mins.
pub(crate) const ENVELOPE_TIME_SCALE: f32 = 256.0;

/// A running instance of an envelope.
#[derive(Clone, Debug)]
pub struct EnvelopeInstance {
    definition: EnvelopeDefinition,
    ramp: ExponentialRamp,
    state: EnvelopePhase,
}

impl EnvelopeInstance {
    pub fn no_sound(output_sample_rate: usize) -> Self {
        Self::new(&EnvelopeDefinition::default(), output_sample_rate)
    }

    /// Generates a new envelope with the given sample rate.
    pub fn new(definition: &EnvelopeDefinition, output_sample_rate: usize) -> Self {
        Self {
            definition: definition.clone(),
            ramp: ExponentialRamp::new(output_sample_rate),
            state: EnvelopePhase::Off,
        }
    }

    /// Advances the envelope forward one tick and returns the output value.
    pub fn tick(&mut self, active: ActiveState) -> f32 {
        if self.definition.total_level == EnvelopeValue(0) {
            0.0
        } else if ActiveState::Trigger == active {
            self.state = EnvelopePhase::Attack;
            self.ramp
                .set_from_envelope(EnvelopePhase::Attack, &self.definition);
            self.ramp.tick()
        } else {
            match self.state {
                EnvelopePhase::Off => {
                    if ActiveState::On == active {
                        self.state = EnvelopePhase::Attack;
                        self.ramp
                            .set_from_envelope(EnvelopePhase::Attack, &self.definition);
                        self.ramp.tick()
                    } else {
                        0.0
                    }
                }
                EnvelopePhase::Attack | EnvelopePhase::Release | EnvelopePhase::Decay => {
                    let out = self.ramp.tick();

                    if self.ramp.is_finished() {
                        self.state = self.state.next_phase();
                        self.ramp.set_from_envelope(self.state, &self.definition)
                    }

                    out
                }
                EnvelopePhase::Sustain => {
                    if ActiveState::Off == active {
                        self.state = self.state.next_phase();
                        self.ramp.set_from_envelope(self.state, &self.definition);
                    }

                    self.ramp.tick()
                }
            }
        }
    }
}
