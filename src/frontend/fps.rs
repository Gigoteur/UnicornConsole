use std::collections::VecDeque;
use std::time::Instant;

const HISTORY_SIZE: usize = 128;

pub struct FpsCounter {
    history: VecDeque<f64>,
    last_time: Instant,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            history: VecDeque::with_capacity(HISTORY_SIZE),
            last_time: Instant::now(),
        }
    }

    pub fn update(&mut self, current_time: Instant) {
        let delta = current_time - self.last_time;
        let delta_s = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / 1_000_000_000.0;

        self.make_room_for_new_element();
        self.history.push_front(delta_s);

        self.last_time = current_time;
    }

    pub fn get_fps(&self) -> f64 {
        let sum = self.history.iter().fold(0.0, |acc, &item| acc + item);
        self.history.len() as f64 / sum
    }

    fn make_room_for_new_element(&mut self) {
        if self.history.len() >= HISTORY_SIZE {
            let _ = self.history.pop_back();
        }
    }
}