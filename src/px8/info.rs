use time::PreciseTime;

pub struct Info {
    pub dt: PreciseTime,
    pub host_time: i16,
    pub real_time: f64,
    pub seconds: f64,
    pub milliseconds: f64,
    pub elapsed_time: f64,
}


impl Info {
    pub fn new() -> Info {
        Info {
            dt: PreciseTime::now(),
            host_time: 0,
            real_time: 0.0,
            seconds: 0.0,
            milliseconds: 0.0,
            elapsed_time: 0.0,
        }
    }
}
