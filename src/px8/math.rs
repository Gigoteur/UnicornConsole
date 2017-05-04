use std::f64::consts::{PI};

pub fn cos(x: f64) -> f64 {
    (x  * (PI*2.0)).cos()
}

pub fn sin(x: f64) -> f64 {
    (-x * (PI*2.0)).sin()
}
