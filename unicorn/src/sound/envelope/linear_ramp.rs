/// A linear ramp which travels to a target value.
#[derive(Debug, Clone)]
pub struct LinearRamp {
    sample_rate: usize,
    value: f32,
    target_value: f32,
    increment: f32,
    remaining_ticks: usize,
}

impl LinearRamp {
    /// TODO: Write docs for this
    pub fn new(sample_rate: usize) -> Self {
        Self {
            sample_rate,
            value: 0.0,
            increment: 0.0,
            remaining_ticks: 0,
            target_value: 0.0,
        }
    }

    pub fn with_value(sample_rate: usize, value: f32) -> Self {
        Self {
            sample_rate,
            value,
            target_value: 0.0,
            increment: 0.0,
            remaining_ticks: 0,
        }
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
    }

    #[allow(dead_code)]
    pub(crate) fn tick(&mut self) -> f32 {
        if !self.is_finished() {
            if self.remaining_ticks == 1 {
                self.value = self.target_value
            } else {
                self.value += self.increment;
            }

            self.remaining_ticks -= 1;
        }

        self.value
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_ticks == 0
    }

    pub fn ramp_to(&mut self, target_value: f32, time: f32) {
        let distance_to_target = target_value - self.value;

        self.target_value = target_value;
        let remaining_ticks = time * self.sample_rate as f32;

        self.increment = distance_to_target / remaining_ticks;
        self.remaining_ticks = remaining_ticks.ceil() as usize;
    }
}
