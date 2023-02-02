// GFX
extern "C" {
    pub fn mode_width() -> u32;
    pub fn mode_height() -> u32;
    pub fn cls(col: i8);
    pub fn circ(x: i32, y:i32, r: i32, col: i8);
    pub fn rnd_range(x: i32, y: i32) -> i32;
    pub fn spr(n: u32, x: i32, y: i32, w: i32, h: i32, flip_x: i32, flip_y: i32, angle: f32, zoom: f32, dynamic: i32);
}