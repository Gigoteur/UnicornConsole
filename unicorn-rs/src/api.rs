use crate::raw;

pub fn cls(col: i8) {
    unsafe { raw::cls(col) }
}

pub fn circ(x: i32, y:i32, r: i32, col: i8) {
    unsafe { raw::circ(x, y, r, col) }
}