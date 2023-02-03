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

pub fn cls(col: i8) {
    unsafe { raw::cls(col) }
}

pub fn circ(x: i32, y:i32, r: i32, col: i8) {
    unsafe { raw::circ(x, y, r, col) }
}

pub fn rnd_range(x: i32, y: i32) -> i32 {
    unsafe { raw::rnd_range(x, y) }
}

pub fn spr(n: u32, x: i32, y: i32, w: i32, h: i32, flip_x: bool, flip_y: bool, angle: f32, zoom: f32, dynamic: bool) {
    unsafe { raw::spr(n, x, y, w, h, bool_to_i32(flip_x), bool_to_i32(flip_y), angle, zoom, bool_to_i32(dynamic)) }
}

pub fn print(text: &str, x: i32, y:i32, col: i32) {
    let text = make_wasm_text_ptr(text);
    unsafe { raw::print(text.0, text.1, x, y, col) }
}

