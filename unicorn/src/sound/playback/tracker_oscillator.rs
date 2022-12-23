use gamercade_audio::PHRASE_STEPS_PER_BEAT;

pub(crate) enum TrackerOscillatorFlow {
    Continue,
    UpdateTracker,
}

#[derive(Debug, Clone)]
pub(crate) struct TrackerOscillator {
    pub(crate) phase: f32,
    pub(crate) increment: f32,
    pub(crate) output_sample_rate: f32,
}

impl TrackerOscillator {
    pub fn new(output_sample_rate: usize) -> Self {
        Self {
            phase: 0.0,
            increment: 0.0,
            output_sample_rate: output_sample_rate as f32,
        }
    }

    pub fn stop(&mut self) {
        self.phase = 0.0;
        self.increment = 0.0;
    }

    pub fn reset_bpm(&mut self, bpm: f32) {
        self.phase = 0.0;
        self.increment =
            ((60.0 / bpm / PHRASE_STEPS_PER_BEAT as f32) * (self.output_sample_rate)).recip();
    }

    pub fn tick(&mut self) -> TrackerOscillatorFlow {
        let output = if self.phase >= 1.0 {
            self.phase -= 1.0;
            TrackerOscillatorFlow::UpdateTracker
        } else {
            TrackerOscillatorFlow::Continue
        };

        self.phase += self.increment;
        output
    }
}
