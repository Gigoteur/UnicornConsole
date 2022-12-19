use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FrameRate {
    SuperSlow,
    Slow,
    Normal,
    Fast,
    SuperFast,
}

impl Default for FrameRate {
    fn default() -> Self {
        Self::Normal
    }
}

impl FrameRate {
    pub const fn frames_per_second(self) -> usize {
        match self {
            FrameRate::SuperSlow => 24,
            FrameRate::Slow => 30,
            FrameRate::Normal => 60,
            FrameRate::Fast => 120,
            FrameRate::SuperFast => 240,
        }
    }

    pub fn frame_time(self) -> f32 {
        (self.frames_per_second() as f32).recip()
    }
}