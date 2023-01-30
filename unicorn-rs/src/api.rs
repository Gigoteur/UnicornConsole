use crate::raw;

pub fn cls(col: u8) {
    unsafe { raw::cls(col) }
}