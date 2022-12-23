use std::f32::consts::PI;

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct FeedbackLevel(pub usize);

impl FeedbackLevel {
    pub fn as_multiplier(self) -> f32 {
        match self.0 {
            0 => 0.0,
            1 => PI / 128.0,
            2 => PI / 64.0,
            3 => PI / 32.0,
            4 => PI / 16.0,
            5 => PI / 8.0,
            6 => PI / 4.0,
            7 => PI / 2.0,
            8 => PI,
            9 => PI * 2.0,
            10 => PI * 4.0,
            11 => PI * 8.0,
            12 => PI * 16.0,
            13 => PI * 32.0,
            14 => PI * 64.0,
            15 => PI * 128.0,
            _ => panic!("invalid feedback level"),
        }
    }

    pub fn max() -> usize {
        15
    }
}
