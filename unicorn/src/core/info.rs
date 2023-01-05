use std::time::Duration;
use std::time::Instant;

#[derive(Debug)]
pub struct Info {
    pub current: Duration,
    pub milliseconds: u64,
    pub elapsed_time: f64,
    pub previous_frame_time: Instant,
}


impl Info {
    pub fn new() -> Info {
        Info {
            current: Duration::from_millis(0),
            milliseconds: 0,
            elapsed_time: 0.0,
            previous_frame_time: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let dt = now.duration_since(self.previous_frame_time);
        self.previous_frame_time = now;

        if self.current > dt {
            let nanoseconds = dt.subsec_nanos();
            self.elapsed_time += dt.as_secs() as f64 + nanoseconds as f64 / 1000000000.0;
            self.milliseconds = nanoseconds as u64 * 1000000000;
        }
        
        self.current = self.current + dt;
    }

    pub fn time(&mut self) -> i64 {
        self.milliseconds as i64
    }

    pub fn time_sec(&mut self) -> f64 {
        self.elapsed_time
    }
}
