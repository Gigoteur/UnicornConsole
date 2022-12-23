use serde::{Deserialize, Serialize};

// TODO: Is it possible to automatically remove duplicates?
// 2/2, 3/3, 4/4, 5/5 are all equal
// 5/2, 10/4,
// There are ~255 unique fractions from 1-20

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct FrequencyMultiplier {
    pub top: u8,
    pub bottom: u8,
}

impl Default for FrequencyMultiplier {
    fn default() -> Self {
        Self { top: 1, bottom: 1 }
    }
}

impl FrequencyMultiplier {
    pub fn one() -> Self {
        Self { top: 1, bottom: 1 }
    }

    pub fn min_value() -> u8 {
        1
    }

    pub fn max_value() -> u8 {
        20
    }

    pub fn multiply(self, value: f32) -> f32 {
        value * (self.top as f32 / self.bottom as f32)
    }
}
