use std::time::Instant;

#[derive(Debug)]
pub struct Info {
    pub previous_frame_time: Instant,
}


impl Info {
    pub fn new() -> Info {
        Info {
            previous_frame_time: Instant::now(),
        }
    }
    pub fn time(&mut self) -> u64 {
        self.previous_frame_time.elapsed().as_secs()
    }

    pub fn mtime(&mut self) -> u128 {
        self.previous_frame_time.elapsed().as_millis()
    }

    pub fn utime(&mut self) -> u128 {
        self.previous_frame_time.elapsed().as_micros()
    }

    pub fn time_sec(&mut self) -> f64 {
        self.previous_frame_time.elapsed().as_secs_f64()
    }
}
