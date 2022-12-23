use std::sync::Arc;

use crate::audio::instruments::fm::algorithm::ModulatedBy;
use crate::audio::instruments::fm::patch_definition::PatchDefinition;
use crate::audio::instruments::fm::OPERATOR_COUNT;
use crate::sound::instruments::ActiveState;

use sound::instruments::fm::operator_instance::OperatorInstanceBundle;

#[derive(Clone, Debug)]
pub struct PatchInstance {
    operators: OperatorInstanceBundle,
    definition: Arc<PatchDefinition>,
    feedback: [f32; 2],
    active: ActiveState,
}

impl PatchInstance {
    pub fn new(definition: Arc<PatchDefinition>, output_sample_rate: usize) -> Self {
        Self {
            operators: OperatorInstanceBundle::new(&definition.operators, output_sample_rate),
            definition,
            feedback: [0.0; 2],
            active: ActiveState::Off,
        }
    }

    /// Sets the base frequency of the entire patch
    pub fn set_frequency(&mut self, frequency: f32) {
        let instances = self.operators.operators.iter_mut();
        let definitions = self.definition.operators.operators.iter();
        instances
            .zip(definitions)
            .for_each(|(instance, definition)| {
                let adjusted_frequency = definition
                    .frequency_multiplier
                    .multiply(frequency * definition.detune.as_multiplier());
                instance.set_frequency(adjusted_frequency)
            });
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

    pub fn tick(&mut self) -> f32 {
        let mut outputs = [0.0f32; OPERATOR_COUNT];
        let mut final_output = 0.0f32;

        let algorithm = self.definition.algorithm.get_definition();
        let operators = &mut self.operators.operators;
        let operator_definitions = &self.definition.operators.operators;

        // 1st Operator is always feedback
        let feedback_input = ((self.feedback[0] + self.feedback[1]) / 2.0)
            * self.definition.feedback.as_multiplier();

        outputs[0] = operators[0].tick(
            operator_definitions[0].waveform,
            feedback_input,
            self.active,
        );

        // Handle feedback
        self.feedback[1] = self.feedback[0];
        self.feedback[0] = outputs[0];

        if algorithm.carriers[0] {
            final_output += outputs[0];
        }
        // End 1st Operator

        // Handle the rest of the operators
        (1..OPERATOR_COUNT).for_each(|i| {
            let operator = &mut operators[i];
            let waveform = operator_definitions[i].waveform;
            let modulator = &algorithm.modulators[i - 1];

            let modulation = match modulator {
                ModulatedBy::None => 0.0,
                ModulatedBy::Single(modulator) => outputs[*modulator],
                ModulatedBy::Double(first, second) => outputs[*first] + outputs[*second],
                ModulatedBy::Triple(first, second, third) => {
                    outputs[*first] + outputs[*second] + outputs[*third]
                }
            };

            let result = operator.tick(waveform, modulation, self.active);

            outputs[i] = result;

            if algorithm.carriers[i] {
                final_output += result;
            }
        });

        if ActiveState::Trigger == self.active {
            self.active = ActiveState::Off;
        }

        final_output
    }

    pub(crate) fn output_sample_rate(&self) -> usize {
        self.operators.operators[0].oscillator.output_sample_rate
    }
}
