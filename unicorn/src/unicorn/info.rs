use std::time::Duration;

pub struct Info {
    pub current: Duration,
    pub milliseconds: u64,
    pub elapsed_time: f64,
}


impl Info {
    pub fn new() -> Info {
        Info {
            current: Duration::from_millis(0),
            milliseconds: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        if self.current > dt {
            let nanoseconds = dt.subsec_nanos();
            self.elapsed_time = dt.as_secs() as f64 + nanoseconds as f64 / 1000000000.0;
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
