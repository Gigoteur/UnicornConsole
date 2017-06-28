use time::PreciseTime;
use time;

pub struct Info {
    pub start_time: time::Tm,
    pub dt: PreciseTime,
    pub seconds: f64,
    pub milliseconds: i64,
    pub elapsed_time: f64,
}


impl Info {
    pub fn new() -> Info {
        Info {
            start_time: time::now(),
            dt: PreciseTime::now(),
            seconds: 0.0,
            milliseconds: 0,
            elapsed_time: 0.0,
        }
    }

    pub fn update(&mut self) {
        let new_time = time::now();
        let diff_time = new_time - self.start_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
        (diff_time.num_seconds() * 1000000000) as f64;

        self.elapsed_time = diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0;
        self.milliseconds = diff_time.num_milliseconds();
    }

    pub fn get_milliseconds(&mut self) -> i64 {
        self.update();
        self.milliseconds
    }
}
