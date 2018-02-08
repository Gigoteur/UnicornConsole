use std::time::{Duration, Instant};

pub struct Info {
    pub start_time: Instant,
    pub milliseconds: u64,
    pub elapsed_time: f64,
}


impl Info {
    pub fn new() -> Info {
        Info {
            start_time: Instant::now(),
            milliseconds: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();

        let diff_time = now.duration_since(self.start_time);
        let nanoseconds = diff_time.subsec_nanos();
        self.elapsed_time = diff_time.as_secs() as f64 + nanoseconds as f64 / 1000000000.0;
        self.milliseconds = nanoseconds as u64 * 1000000000;
    }

    pub fn time(&mut self) -> i64 {
        self.update();
        self.milliseconds as i64
    }

    pub fn time_sec(&mut self) -> f64 {
        self.update();
        self.elapsed_time
    }
}
