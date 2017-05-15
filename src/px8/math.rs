use std::f64::consts::PI;

#[allow(dead_code)]
pub fn cos(x: f64) -> f64 {
    (x * (PI * 2.0)).cos()
}

#[allow(dead_code)]
pub fn sin(x: f64) -> f64 {
    (-x * (PI * 2.0)).sin()
}
