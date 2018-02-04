use chrono::prelude::*;

pub struct Info {
    pub start_time: DateTime<Utc>,
    pub seconds: f64,
    pub milliseconds: i64,
    pub elapsed_time: f64,
}


impl Info {
    pub fn new() -> Info {
        Info {
            start_time: Utc::now(),
            seconds: 0.0,
            milliseconds: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let new_time = Utc::now();
        let diff_time = new_time.signed_duration_since(self.start_time);
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
                          (diff_time.num_seconds() * 1000000000) as f64;

        self.elapsed_time = diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0;
        self.milliseconds = diff_time.num_milliseconds();
    }

    pub fn time(&mut self) -> i64 {
        self.update();
        self.milliseconds
    }

    pub fn time_sec(&mut self) -> f64 {
        self.update();
        self.elapsed_time
    }
}
