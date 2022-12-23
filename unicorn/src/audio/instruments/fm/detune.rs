use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Detune(pub i8);

impl Detune {
    pub fn as_multiplier(self) -> f32 {
        assert!(Self::max() <= 100);
        assert!(Self::min() >= -100);
        if self.0 >= 0 {
            1.0 + ((self.0 as f32 / Self::max() as f32) * 0.059_463_095)
        } else {
            1.0 + ((self.0 as f32 / Self::max() as f32) * (1.0 - 0.943_874_3))
        }
    }

    pub fn min() -> i8 {
        -100
    }

    pub fn max() -> i8 {
        100
    }
}
