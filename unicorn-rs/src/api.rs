use crate::raw;

pub(crate) fn bool_to_i32(val: bool) -> i32 {
    match val {
        true => 1,
        false => 0,
    }
}

fn make_wasm_text_ptr(text: &str) -> (i32, i32) {
    (text.as_ptr() as i32, text.len() as i32)
}

pub fn mode_height() -> u32 {
    unsafe { raw::mode_height() }
}

pub fn mode_width() -> u32 {
    unsafe { raw::mode_width() }
}

pub fn btnp(x: i32, p: i32) -> bool {
    unsafe { raw::btnp(x, p) == 1 }
}

pub fn mouse_left_statep(p: i32) -> bool {
    unsafe { raw::mouse_left_statep(p) == 1 }
}

pub fn mouse_x() -> u32 {
    unsafe { raw::mouse_x() }
}

pub fn mouse_y() -> u32 {
    unsafe { raw::mouse_y() }
}

pub fn cls(col: i8) {
    unsafe { raw::cls(col) }
}

pub fn pset(x: i32, y:i32, col: i8) {
    unsafe { raw::pset(x, y, col) }
}

pub fn pset_rgba(x: i32, y:i32, r: u8, g: u8, b: u8, a: u8 ) {
    unsafe { raw::pset_rgba(x, y, r as i32, g as i32, b as i32, a as i32) }
}

pub fn circ(x: i32, y:i32, r: i32, col: i8) {
    unsafe { raw::circ(x, y, r, col) }
}

pub fn frnd() -> f32 {
    unsafe { raw::frnd() }
}

pub fn rnd_range(x: i32, y: i32) -> i32 {
    unsafe { raw::rnd_range(x, y) }
}

pub fn spr(n: u32, x: i32, y: i32, w: i32, h: i32, flip_x: bool, flip_y: bool, angle: f32, zoom: f32, dynamic: bool) {
    unsafe { raw::spr(n, x, y, w, h, bool_to_i32(flip_x), bool_to_i32(flip_y), angle, zoom, bool_to_i32(dynamic)) }
}
pub fn debug_print(text: &str) {
    let text = make_wasm_text_ptr(text);
    unsafe { raw::debug_print(text.0, text.1) }
}

pub fn print(text: &str, x: i32, y:i32, col: i32) {
    let text = make_wasm_text_ptr(text);
    unsafe { raw::print(text.0, text.1, x, y, col) }
}

