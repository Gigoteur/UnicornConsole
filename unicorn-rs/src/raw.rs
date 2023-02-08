// GFX
extern "C" {
    pub fn mode_width() -> u32;
    pub fn mode_height() -> u32;

    pub fn btnp(x: i32, p: i32) -> u32;
    pub fn mouse_left_state(p: i32) -> u32;
    pub fn mouse_left_statep(p: i32) -> u32;
    pub fn mouse_x() -> u32;
    pub fn mouse_y() -> u32;

    pub fn cls(col: i8);
    
    pub fn pset(x: i32, y:i32, col: i8);
    pub fn pset_rgba(x: i32, y:i32, r:i32, g:i32, b:i32, a:i32);
    
    pub fn circ(x: i32, y:i32, r: i32, col: i8);
    pub fn circfill(x: i32, y:i32, r: i32, col: i8);
    pub fn rectfill(x0: i32, y0:i32, x1: i32, y1:i32, col: i8);

    pub fn rnd_range(x: i32, y: i32) -> i32;
    pub fn frnd() -> f32;

    pub fn spr(n: u32, x: i32, y: i32, w: i32, h: i32, flip_x: i32, flip_y: i32, angle: f32, zoom: f32, dynamic: i32);
    pub fn print(text_ptr:i32, len:i32, x: i32, y:i32, col: i32);
    pub fn debug_print(text_ptr:i32, len:i32);
}