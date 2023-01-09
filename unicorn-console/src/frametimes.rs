use std::time::{Duration, Instant};

pub struct FrameTimes {
    frame_duration: Duration,
    last_time: Instant,
    target_time: Instant,
}

impl FrameTimes {
    pub fn new(frame_duration: Duration) -> FrameTimes {
        let now = Instant::now();
        FrameTimes {
            frame_duration: frame_duration,
            last_time: now,
            target_time: now + frame_duration,
        }
    }

    pub fn reset(&mut self) {
        let now = Instant::now();
        self.last_time = now;
        self.target_time = now + self.frame_duration;
    }

    pub fn update(&mut self) -> Duration {
        let now = Instant::now();
        let delta = now - self.last_time;
        self.last_time = now;
        self.target_time += self.frame_duration;
        delta
    }

    pub fn get_last_time(&mut self) -> Instant {
        self.last_time
    }
}
