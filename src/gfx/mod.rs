use std::fmt;
use nalgebra::{Dynamic, Matrix, MatrixVec};

use px8;
use std::cmp;
use num::clamp;

pub const GLYPH: [[u16; 2]; 95] = [
    [0x0000, 0x0000], // space
    [0x0017, 0x0000], // !
    [0x0300, 0x0300], // "
    [0x1f0a, 0x1f00], // #
    [0x0d1f, 0x0b00], // $
    [0x1304, 0x1900], // %
    [0x1817, 0x1f00], // &
    [0x0001, 0x0200], // '
    [0x0011, 0x0e00], // (
    [0x000e, 0x1100], // )
    [0x150e, 0x1500], // *
    [0x040e, 0x0400], // +
    [0x0010, 0x2000], // ,
    [0x0404, 0x0400], // -
    [0x0010, 0x0000], // .
    [0x010e, 0x1000], // /

    [0x1f11, 0x1f00], // 0
    [0x101f, 0x1100], // 1
    [0x1715, 0x1d00], // 2
    [0x1f15, 0x1100], // 3
    [0x1f04, 0x0700], // 4
    [0x1d15, 0x1700], // 5
    [0x1d15, 0x1f00], // 6
    [0x1f01, 0x0100], // 7
    [0x1f15, 0x1f00], // 8
    [0x1f05, 0x0700], // 9

    [0x000a, 0x0000], // :
    [0x000a, 0x1000], // ;
    [0x110a, 0x0400], // <
    [0x0a0a, 0x0a00], // =
    [0x040a, 0x1100], // >
    [0x0715, 0x0100], // ?
    [0x1611, 0x0e00], // @

    [0x1f05, 0x1f00], // A
    [0x1b15, 0x1f00], // B
    [0x1111, 0x0e00], // C
    [0x1e11, 0x1f00], // D
    [0x1115, 0x1f00], // E
    [0x0105, 0x1f00], // F
    [0x1911, 0x1e00], // G
    [0x1f04, 0x1f00], // H
    [0x111f, 0x1100], // I
    [0x011f, 0x1100], // J
    [0x1b04, 0x1f00], // K
    [0x1010, 0x1f00], // L
    [0x1f03, 0x1f00], // M
    [0x1e01, 0x1f00], // N
    [0x0f11, 0x1e00], // O
    [0x0705, 0x1f00], // P
    [0x1619, 0x0e00], // Q
    [0x1b05, 0x1f00], // R
    [0x0d15, 0x1600], // S
    [0x011f, 0x0100], // T
    [0x1f10, 0x0f00], // U
    [0x0f10, 0x0f00], // V
    [0x1f18, 0x1f00], // W
    [0x1b04, 0x1b00], // X
    [0x1f14, 0x1700], // Y
    [0x1315, 0x1900], // Z

    [0x0011, 0x1F00], // [
    [0x100e, 0x0100], // \
    [0x001F, 0x1100], // ]
    [0x0201, 0x0200], // ^
    [0x1010, 0x1000], // _
    [0x0201, 0x0000], // `

    [0x1c14, 0x0800], // a
    [0x0814, 0x1f00], // b
    [0x1414, 0x0800], // c
    [0x1f14, 0x0800], // d
    [0x1414, 0x0c00], // e
    [0x051e, 0x0400], // f
    [0x3c54, 0x5800], // g
    [0x1804, 0x1f00], // h
    [0x001d, 0x0000], // i
    [0x001d, 0x2000], // j
    [0x1408, 0x1f00], // k
    [0x100f, 0x0000], // l
    [0x1c0c, 0x1c00], // m
    [0x1804, 0x1c00], // n
    [0x0814, 0x0800], // o
    [0x1814, 0x7c00], // p
    [0x7c14, 0x0c00], // q
    [0x0404, 0x1800], // r
    [0x041c, 0x1000], // s
    [0x140e, 0x0400], // t
    [0x1c10, 0x0c00], // u
    [0x0c18, 0x0c00], // v
    [0x1c18, 0x1c00], // w
    [0x1408, 0x1400], // x
    [0x3c50, 0x5c00], // y
    [0x101c, 0x0400], // z

    [0x111f, 0x0400], // {
    [0x001F, 0x0000], // |
    [0x041f, 0x1100], // }
    [0x0604, 0x0c00], // ~
];

type DMatrixu32 = Matrix<u32, Dynamic, Dynamic, MatrixVec<u32, Dynamic, Dynamic>>;

#[derive(Clone)]
pub struct DynSprite {
    pub data: DMatrixu32,
}

impl DynSprite {
    pub fn new(data: Vec<u32>, width: u32, height: u32) -> DynSprite {
        let mut d_mat = DMatrixu32::from_element(height as usize, width as usize, 0);

        let mut idx = 0;

        debug!("WIDTH {:?} HEIGHT {:?} -> {:?} {:?}",
               width,
               height,
               d_mat.ncols(),
               d_mat.nrows());

        for i in 0..width {
            for j in 0..height {
                d_mat[(i + j * width) as usize] = *data.get(idx).unwrap();
                idx += 1;
            }
        }

        DynSprite { data: d_mat.clone() }
    }

    pub fn new_from_matrix(d_mat: DMatrixu32) -> DynSprite {
        DynSprite { data: d_mat }
    }


    pub fn flip_x(&mut self) -> DMatrixu32 {
        let mut r_mat = self.data.clone();

        let n_cols = r_mat.ncols();
        let n_rows = r_mat.nrows();

        for i in 0..n_cols / 2 {
            for j in 0..n_rows {
                let tmp = r_mat[(i + j * n_cols) as usize];
                r_mat[(i + j * n_cols) as usize] = r_mat[((n_cols - (i + 1)) + j * n_cols) as
                usize];
                r_mat[((n_cols - (i + 1)) + j * n_cols) as usize] = tmp;
            }
        }
        r_mat
    }

    pub fn flip_y(&mut self) -> DMatrixu32 {
        let mut r_mat = self.data.clone();

        let n_cols = r_mat.ncols();
        let n_rows = r_mat.nrows();

        for i in 0..n_rows / 2 {
            for j in 0..n_cols {
                let tmp = r_mat[(j + i * n_cols) as usize];
                r_mat[(j + i * n_cols) as usize] = r_mat[(j + (n_rows - (i + 1)) * n_cols) as
                usize];
                r_mat[(j + (n_rows - (i + 1)) * n_cols) as usize] = tmp;
            }
        }
        r_mat
    }
}

impl fmt::Debug for DynSprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data_matrix = String::new();
        data_matrix.push('\n');

        for j in 0..self.data.nrows() {
            for i in 0..self.data.ncols() {
                data_matrix
                    .push_str(format!("{:?} ", self.data[(i + j * self.data.ncols()) as usize])
                                  .as_str());
            }
            data_matrix.push('\n');

        }

        write!(f, "{}", data_matrix)
    }
}

#[derive(Clone)]
pub struct Sprite {
    pub data: Vec<u8>,
    pub flags: u8,
}

impl Sprite {
    pub fn new(d: [u8; 8 * 8]) -> Sprite {
        let mut v = Vec::new();
        v.extend(d.iter().cloned());

        Sprite { data: v, flags: 0 }
    }

    pub fn is_flags_set(&self, value: u8) -> bool {
        let mut value = value << 1;

        if value == 0 {
            value = 1;
        }

        (self.flags & value) != 0
    }

    pub fn is_bit_flags_set(&self, value: u8) -> bool {
        (self.flags & value) != 0
    }


    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.flags |= flag << 1;
        } else {
            self.flags &= !flag << 1;
        }
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    pub fn set_data(&mut self, idx: usize, col: u8) {
        self.data[idx] = col;
    }

    pub fn get_data(&self) -> String {
        let mut data = String::new();

        for c in &self.data {
            data.push_str(&format!("{:?}", c));
        }

        data
    }

    pub fn get_line(&self, line: u32) -> String {
        let mut data = String::new();

        let mut data_clone = self.data.clone();

        let data_line: Vec<_> = data_clone
            .drain((line * 8) as usize..(line * 8 + 8) as usize)
            .collect();

        for c in data_line.clone() {
            data.push_str(&format!("{:x}", c));
        }

        data
    }

    pub fn horizontal_reflection(&self) -> [u8; 64] {
        let mut ret: [u8; 64] = self.to_u8_64_array();

        for i in 0..4 {
            for j in 0..8 {
                ret.swap((i + j * 8) as usize, ((8 - (i + 1)) + j * 8) as usize);
            }
        }

        ret
    }

    pub fn vertical_reflection(&self) -> [u8; 64] {
        let mut ret: [u8; 64] = self.to_u8_64_array();

        for i in 0..4 {
            for j in 0..8 {
                ret.swap((j + i * 8) as usize, (j + (8 - (i + 1)) * 8) as usize);
            }
        }

        ret
    }

    pub fn flip_x(&self) -> Sprite {
        Sprite::new(self.horizontal_reflection())
    }

    pub fn flip_y(&self) -> Sprite {
        Sprite::new(self.vertical_reflection())
    }

    pub fn to_u8_64_array(&self) -> [u8; 64] {
        let mut arr = [0u8; 64];
        for (place, element) in arr.iter_mut().zip(self.data.iter()) {
            *place = *element;
        }
        arr
    }
}

impl fmt::Debug for Sprite {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data_matrix = String::new();
        data_matrix.push('\n');

        for i in 0..8 {
            data_matrix.push_str(format!("{:?}", &self.data[i * 8..i * 8 + 8]).as_str());
            data_matrix.push('\n');
        }

        write!(f, "{}", data_matrix)
    }
}

// Screen scaling

#[derive(Copy, Clone)]
pub enum Scale {
    Scale1x,
    Scale2x,
    Scale3x,
    Scale4x,
    Scale5x,
    Scale6x,
    Scale8x,
    Scale10x,
}

impl Scale {
    pub fn factor(self) -> usize {
        match self {
            Scale::Scale1x => 1,
            Scale::Scale2x => 2,
            Scale::Scale3x => 3,
            Scale::Scale4x => 4,
            Scale::Scale5x => 5,
            Scale::Scale6x => 6,
            Scale::Scale8x => 8,
            Scale::Scale10x => 10,
        }
    }
}

pub struct Camera {
    pub x: i32,
    pub y: i32,
}

impl Camera {
    pub fn new() -> Camera {
        Camera { x: 0, y: 0 }
    }
}

// Clipping rectangle is exclusive of right and bottom edges
pub struct Clipping {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl Clipping {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Clipping {
        Clipping {
            left: clamp(left, 0, px8::SCREEN_WIDTH as i32),
            top: clamp(top, 0, px8::SCREEN_HEIGHT as i32),
            right: clamp(right, 0, px8::SCREEN_WIDTH as i32),
            bottom: clamp(bottom, 0, px8::SCREEN_HEIGHT as i32),
        }
    }
}

pub struct Screen {
    pub back_buffer: px8::ScreenBuffer,
    pub saved_back_buffer: Box<px8::ScreenBuffer>,
    pub buffer_rgb: Box<px8::ScreenBufferRGB>,

    pub sprites: Vec<Sprite>,
    pub dyn_sprites: Vec<DynSprite>,

    pub map: [[u32; px8::MAP_HEIGHT]; px8::MAP_WIDTH],

    pub transparency_map: [bool; 256],

    pub color: u32,
    pub color_map: [u8; 256],

    pub camera: Camera,
    pub clipping: Clipping,
}

unsafe impl Send for Screen {}
unsafe impl Sync for Screen {}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            back_buffer: px8::SCREEN_EMPTY,
            saved_back_buffer: Box::new(px8::SCREEN_EMPTY),
            buffer_rgb: Box::new([0; px8::SCREEN_PIXELS_RGB]),

            sprites: Vec::new(),
            dyn_sprites: Vec::new(),
            map: [[0; px8::MAP_HEIGHT]; px8::MAP_WIDTH],

            transparency_map: [false; 256],
            color_map: [0; 256],
            color: 0,

            camera: Camera::new(),

            clipping: Clipping::new(0, 0, px8::SCREEN_WIDTH as i32, px8::SCREEN_HEIGHT as i32),
        }
    }

    pub fn init(&mut self) {
        self._reset_colors();
        self._reset_transparency();
        self._reset_clip();
    }

    pub fn _reset_transparency(&mut self) {
        self.transparency_map = [false; 256];
        self.transparency_map[0] = true;
    }

    pub fn _reset_colors(&mut self) {
        for i in 0..256 {
            self.color_map[i] = i as u8;
        }
    }

    pub fn _reset_clip(&mut self) {
        self.clipping = Clipping::new(0, 0, px8::SCREEN_WIDTH as i32, px8::SCREEN_HEIGHT as i32);
    }

    pub fn save(&mut self) {
        for i in 0..px8::SCREEN_PIXELS {
            self.saved_back_buffer[i] = self.back_buffer[i];
        }
    }

    pub fn restore(&mut self) {
        for i in 0..px8::SCREEN_PIXELS {
            self.back_buffer[i] = self.saved_back_buffer[i];
        }
    }

    pub fn _find_color(&mut self, col: i32) -> u32 {
        if col == -1 { self.color } else { col as u32 }
    }

    pub fn camera(&mut self, x: i32, y: i32) {
        self.camera.x = x;
        self.camera.y = y;
    }

    pub fn set_sprites(&mut self, sprites: Vec<Sprite>) {
        self.sprites = sprites;
    }

    pub fn set_map(&mut self, map: [[u32; px8::MAP_HEIGHT]; px8::MAP_WIDTH]) {
        self.map = map;
    }

    pub fn set_sprites_flags(&mut self, flags: Vec<u8>) {
        if flags.len() != self.sprites.len() {
            error!("Invalid number of flags {:?} --> {:?}",
                   flags.len(),
                   self.sprites.len());
            return;
        }

        let mut idx = 0;
        while idx < flags.len() {
            self.sprites[idx].set_flags(flags[idx]);
            idx += 1;
        }
    }

    pub fn pixel_offset(x: i32, y: i32) -> usize {
        (x as usize) + ((y as usize) * px8::SCREEN_WIDTH)
    }

    pub fn putpixel_direct(&mut self, x: i32, y: i32, col: u32) {
        if x >= px8::SCREEN_WIDTH as i32 || y >= px8::SCREEN_HEIGHT as i32 {
            return;
        }
        self.back_buffer[Screen::pixel_offset(x, y)] = col as u8;
    }

    pub fn putpixel_(&mut self, x: i32, y: i32, col: u32) {
        // Make camera adjustment
        let x = x - self.camera.x;
        let y = y - self.camera.y;

        // Clip
        if x < self.clipping.left || x >= self.clipping.right || y < self.clipping.top ||
           y >= self.clipping.bottom {
            return;
        };

        if x >= px8::SCREEN_WIDTH as i32 || y >= px8::SCREEN_HEIGHT as i32 {
            return;
        }

        let draw_col = self.color_map[(col & 0xFF) as usize];

        self.back_buffer[Screen::pixel_offset(x, y)] = draw_col;
    }

    pub fn color(&mut self, col: i32) {
        if (col >= 0) && (col <= 255) {
            self.color = col as u32;
        }
    }

    pub fn putpixel(&mut self, x: i32, y: i32, col: u32) {
        self.putpixel_(x, y, col);
    }

    pub fn getpixel(&mut self, x: usize, y: usize) -> u32 {
        let x = (x as i32 - self.camera.x) as usize;
        let y = (y as i32 - self.camera.y) as usize;

        if x >= px8::SCREEN_WIDTH || y >= px8::SCREEN_HEIGHT {
            return 0;
        }

        self.back_buffer[x + y * px8::SCREEN_WIDTH] as u32
    }

    pub fn pget(&mut self, x: u32, y: u32) -> u32 {
        self.getpixel(x as usize, y as usize)
    }

    pub fn pset(&mut self, x: i32, y: i32, col: i32) {
        let color = self._find_color(col);
        self.putpixel_(x, y, color);
    }

    pub fn sget(&mut self, x: u32, y: u32) -> u8 {
        let idx_sprite = (x / 8) + 16 * (y / 8);
        let sprite = &self.sprites[idx_sprite as usize];
        sprite.data[((x % 8) + (y % 8) * 8) as usize]
    }

    pub fn sset(&mut self, x: u32, y: u32, col: i32) {
        let col = self._find_color(col);

        let idx_sprite = (x / 8) + 16 * (y / 8);
        let sprite = &mut self.sprites[idx_sprite as usize];
        sprite.set_data(((x % 8) + (y % 8) * 8) as usize, col as u8);
    }

    pub fn fget(&mut self, idx: u32, v: u8) -> bool {
        if (idx as usize) < self.sprites.len() {
            self.sprites[idx as usize].is_flags_set(v as u8)
        } else {
            false
        }
    }

    pub fn fget_all(&mut self, idx: u32) -> u8 {
        if (idx as usize) < self.sprites.len() {
            self.sprites[idx as usize].get_flags()
        } else {
            0
        }
    }

    pub fn fset(&mut self, idx: u32, flag: u8, value: bool) {
        if (idx as usize) < self.sprites.len() {
            self.sprites[idx as usize].set_flag(flag, value);
        }
    }

    pub fn fset_all(&mut self, idx: u32, flags: u8) {
        if (idx as usize) < self.sprites.len() {
            self.sprites[idx as usize].set_flags(flags);
        }
    }

    pub fn cls(&mut self) {
        self.back_buffer = px8::SCREEN_EMPTY;
    }

    pub fn force_print(&mut self, string: String, x: i32, y: i32, col: i32) {
        self._print(string, x, y, col, true);
    }

    pub fn print(&mut self, string: String, x: i32, y: i32, col: i32) {
        self._print(string, x, y, col, false);
    }

    pub fn _print(&mut self, string: String, x: i32, y: i32, col: i32, force: bool) {
        let mut x = x;
        let y = y;

        for k in 0..string.len() {
            let value = string.as_bytes()[k] as usize;

            let data = if value >= 32 && value <= 126 {
                GLYPH[value - 32]
            } else {
                /* Unknown char, replace by a space */
                [0x0000, 0x0000]
            };

            let mut idx = 1;
            let mut idx_1 = 0;

            for i in 0..32 {
                if (data[idx] & (0x1 << idx_1)) != 0 {
                    if force {
                        self.putpixel_direct(x, y + i % 8, col as u32);
                    } else {
                        self.pset(x, y + i % 8, col);
                    }
                }

                idx_1 += 1;

                if i % 8 == 7 {
                    x += 1;
                }
                if i == 15 {
                    idx = 0;
                    idx_1 = 0;
                }
            }
        }
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, col: i32) {
        debug!("LINE {:?} {:?} {:?} {:?} {:?}", x0, y0, x1, y1, col);

        let color = self._find_color(col);

        let (mut x0, mut y0) = (x0, y0);
        let (x1, y1) = (x1, y1);

        let dx = (x1 - x0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let dy: i32 = -((y1 - y0).abs());
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err: i32 = dx + dy; /* error value e_xy */

        loop {
            self.putpixel(x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            } /* e_xy+e_x > 0 */
            if e2 <= dx {
                err += dx;
                y0 += sy;
            } /* e_xy+e_y < 0 */
        }
    }

    pub fn hline(&mut self, x1: i32, x2: i32, y: i32, col: i32) {
        let x_min = cmp::min(x1, x2);
        let x_max = cmp::max(x1, x2);

        for x in x_min..(x_max + 1) {
            self.putpixel(x, y, col as u32);
        }
    }

    pub fn rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, col: i32) {
        let x_min = cmp::min(x0, x1);
        let x_max = cmp::max(x0, x1);
        let y_min = cmp::min(y0, y1);
        let y_max = cmp::max(y0, y1);

        for x in x_min..(x_max + 1) {
            self.putpixel(x, y_min, col as u32);
            self.putpixel(x, y_max, col as u32);
        }
        for y in (y_min + 1)..y_max {
            self.putpixel(x0, y, col as u32);
            self.putpixel(x1, y, col as u32);
        }
    }

    pub fn rectfill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, col: i32) {
        let x_min = cmp::min(x0, x1);
        let x_max = cmp::max(x0, x1);
        let y_min = cmp::min(y0, y1);
        let y_max = cmp::max(y0, y1);

        for y in y_min..(y_max + 1) {
            for x in x_min..(x_max + 1) {
                self.putpixel(x, y, col as u32);
            }
        }
    }

    pub fn square(&mut self, x0: i32, y0: i32, h: i32, col: i32) {
        self.rect(x0, y0, x0 + h, y0 + h, col);
    }

    pub fn squarefill(&mut self, x0: i32, y0: i32, h: i32, col: i32) {
        self.rectfill(x0, y0, x0 + h, y0 + h, col);
    }

    pub fn circ(&mut self, x: i32, y: i32, r: i32, col: i32) {
        self.ellipse(x, y, r, r, col);
    }

    pub fn circfill(&mut self, x: i32, y: i32, r: i32, col: i32) {
        self.ellipsefill(x, y, r, r, col);
    }

    pub fn clip(&mut self, x: i32, y: i32, w: i32, h: i32) {
        if x == -1 && y == -1 && w == -1 && h == -1 {
            self._reset_clip();
            return;
        }

        // Clipping rectangle is exclusive of right and bottom edges
        self.clipping = Clipping::new(x, y, x + w, y + h);
    }

    // Original algorithm from SDL2 gfx project
    pub fn ellipse(&mut self, x: i32, y: i32, rx: i32, ry: i32, col: i32) {
        if rx <= 0 || ry <= 0 {
            return;
        }

        let mut h: i32;
        let mut i: i32;
        let mut j: i32;
        let mut k: i32;

        let mut ok: i32 = 0xFFFF;
        let mut oj: i32 = 0xFFFF;
        let mut oh: i32 = 0xFFFF;
        let mut oi: i32 = 0xFFFF;

        let mut ix: i32;
        let mut iy: i32;

        let mut xmi: i32;
        let mut xpi: i32;

        let mut xmj: i32;
        let mut xpj: i32;
        let mut ymi: i32;
        let mut ypi: i32;

        let mut xmk: i32;
        let mut xpk: i32;
        let mut ymh: i32;
        let mut yph: i32;

        let mut ymj: i32;
        let mut ypj: i32;

        let mut xmh: i32;
        let mut xph: i32;

        let mut ymk: i32;
        let mut ypk: i32;

        let col = self._find_color(col);

        if rx > ry {
            ix = 0;
            iy = rx * 64;

            h = (ix + 32) >> 6;
            i = (iy + 32) >> 6;

            while i > h {
                h = (ix + 32) >> 6;
                i = (iy + 32) >> 6;
                j = (h * ry) / rx;
                k = (i * ry) / rx;

                if ((ok != k) && (oj != k)) || ((oj != j) && (ok != j)) || (k != j) {
                    xph = x + h;
                    xmh = x - h;

                    if k > 0 {
                        ypk = y + k;
                        ymk = y - k;

                        self.putpixel(xmh, ypk, col);
                        self.putpixel(xph, ypk, col);
                        self.putpixel(xmh, ymk, col);
                        self.putpixel(xph, ymk, col);
                    } else {
                        self.putpixel(xmh, y, col);
                        self.putpixel(xph, y, col);
                    }

                    ok = k;
                    xpi = x + i;
                    xmi = x - i;
                    if j > 0 {
                        ypj = y + j;
                        ymj = y - j;
                        self.putpixel(xmi, ypj, col);
                        self.putpixel(xpi, ypj, col);
                        self.putpixel(xmi, ymj, col);
                        self.putpixel(xpi, ymj, col);
                    } else {
                        self.putpixel(xmi, y, col);
                        self.putpixel(xpi, y, col);
                    }
                    oj = j;
                }

                ix += iy / rx;
                iy -= ix / rx;
            }
        } else {
            ix = 0;
            iy = ry * 64;

            h = (ix + 32) >> 6;
            i = (iy + 32) >> 6;

            while i > h {
                h = (ix + 32) >> 6;
                i = (iy + 32) >> 6;
                j = (h * rx) / ry;
                k = (i * rx) / ry;

                if ((oi != i) && (oh != i)) || ((oh != h) && (oi != h) && (i != h)) {
                    xmj = x - j;
                    xpj = x + j;
                    if i > 0 {
                        ypi = y + i;
                        ymi = y - i;
                        self.putpixel(xmj, ypi, col);
                        self.putpixel(xpj, ypi, col);
                        self.putpixel(xmj, ymi, col);
                        self.putpixel(xpj, ymi, col);
                    } else {
                        self.putpixel(xmj, y, col);
                        self.putpixel(xpj, y, col);
                    }


                    oi = i;
                    xmk = x - k;
                    xpk = x + k;
                    if h > 0 {
                        yph = y + h;
                        ymh = y - h;
                        self.putpixel(xmk, yph, col);
                        self.putpixel(xpk, yph, col);
                        self.putpixel(xmk, ymh, col);
                        self.putpixel(xpk, ymh, col);
                    } else {
                        self.putpixel(xmk, y, col);
                        self.putpixel(xpk, y, col);
                    }
                    oh = h;
                }

                ix += iy / ry;
                iy -= ix / ry;
            }
        }
    }

    // Original algorithm from SDL2 gfx project
    pub fn ellipsefill(&mut self, x: i32, y: i32, rx: i32, ry: i32, col: i32) {
        if rx <= 0 || ry <= 0 {
            return;
        }

        let mut h: i32;
        let mut i: i32;
        let mut j: i32;
        let mut k: i32;

        let mut ok: i32 = 0xFFFF;
        let mut oj: i32 = 0xFFFF;
        let mut oh: i32 = 0xFFFF;
        let mut oi: i32 = 0xFFFF;

        let mut ix: i32;
        let mut iy: i32;

        let mut xmi: i32;
        let mut xpi: i32;
        let mut xmj: i32;
        let mut xpj: i32;

        let mut xmh: i32;
        let mut xph: i32;

        let mut xmk: i32;
        let mut xpk: i32;

        if rx > ry {
            ix = 0;
            iy = rx * 64;

            h = (ix + 32) >> 6;
            i = (iy + 32) >> 6;

            while i > h {
                h = (ix + 32) >> 6;
                i = (iy + 32) >> 6;
                j = (h * ry) / rx;
                k = (i * ry) / rx;

                if (ok != k) && (oj != k) {
                    xph = x + h;
                    xmh = x - h;
                    if k > 0 {
                        self.hline(xmh, xph, y + k, col);
                        self.hline(xmh, xph, y - k, col);
                    } else {
                        self.hline(xmh, xph, y, col);
                    }
                    ok = k;
                }
                if (oj != j) && (ok != j) && (k != j) {
                    xmi = x - i;
                    xpi = x + i;
                    if j > 0 {
                        self.hline(xmi, xpi, y + j, col);
                        self.hline(xmi, xpi, y - j, col);
                    } else {
                        self.hline(xmi, xpi, y, col);
                    }
                    oj = j;
                }

                ix += iy / rx;
                iy -= ix / rx;
            }
        } else {
            ix = 0;
            iy = ry * 64;

            h = (ix + 32) >> 6;
            i = (iy + 32) >> 6;

            while i > h {
                h = (ix + 32) >> 6;
                i = (iy + 32) >> 6;
                j = (h * rx) / ry;
                k = (i * rx) / ry;

                if (oi != i) && (oh != i) {
                    xmj = x - j;
                    xpj = x + j;
                    if i > 0 {
                        self.hline(xmj, xpj, (y + i), col);
                        self.hline(xmj, xpj, (y - i), col);
                    } else {
                        self.hline(xmj, xpj, y, col);
                    }
                    oi = i;
                }
                if (oh != h) && (oi != h) && (i != h) {
                    xmk = x - k;
                    xpk = x + k;
                    if h > 0 {
                        self.hline(xmk, xpk, (y + h), col);
                        self.hline(xmk, xpk, (y - h), col);
                    } else {
                        self.hline(xmk, xpk, y, col);
                    }
                    oh = h;
                }

                ix += iy / ry;
                iy -= ix / ry;
            }
        }
    }

    pub fn trigon(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, col: i32) {
        let mut vx = Vec::new();
        let mut vy = Vec::new();

        vx.push(x1);
        vx.push(x2);
        vx.push(x3);

        vy.push(y1);
        vy.push(y2);
        vy.push(y3);

        self.polygon(vx.clone(), vy.clone(), col);
    }


    pub fn polygon(&mut self, vx: Vec<i32>, vy: Vec<i32>, col: i32) {
        if vx.len() < 3 || vy.len() < 3 {
            return;
        }

        if vx.len() != vy.len() {
            return;
        }

        let mut idx = 0;

        while idx < vx.len() - 1 {
            self.line(vx[idx], vy[idx], vx[idx + 1], vy[idx + 1], col);
            idx += 1;
        }

        self.line(vx[idx], vy[idx], vx[0], vy[0], col);
    }

    pub fn spr(&mut self, n: u32, x: i32, y: i32, w: u32, h: u32, flip_x: bool, flip_y: bool) {
        debug!("PRINT SPRITE = x:{:?} y:{:?} n:{:?} w:{:?} h:{:?} flip_x:{:?} flip_y:{:?}",
               x,
               y,
               n,
               w,
               h,
               flip_x,
               flip_y);

        let mut orig_x = x;
        let mut orig_y = y;

        for i in 0..h {
            for j in 0..w {
                let sprite_offset = ((j + n) + i * 16) as usize;
                if sprite_offset >= self.sprites.len() {
                    break;
                }

                let mut sprite = self.sprites[sprite_offset].clone();

                if flip_x {
                    sprite = sprite.flip_x();
                }
                if flip_y {
                    sprite = sprite.flip_y();
                }

                let mut new_x = orig_x;
                let mut new_y = orig_y;

                debug!("SPRITE = {:?} x:{:?} y:{:?} {:?}",
                       sprite_offset,
                       new_x,
                       new_y,
                       sprite);

                let mut index = 0;
                for c in &sprite.data {
                    if !self.is_transparent(*c as u32) {
                        self.putpixel_(new_x, new_y, *c as u32);
                    }

                    index += 1;

                    if index != 0 && index % 8 == 0 {
                        new_y += 1;
                        new_x = orig_x;
                    } else {
                        new_x += 1;
                    }
                }

                orig_x += 8;
            }
            orig_y += 8;
            orig_x = x;
        }
    }

    pub fn spr_dyn(&mut self, id: u32, x: i32, y: i32, flip_x: bool, flip_y: bool) {
        //debug!("SPR DYN {:?}: {:?} {:?}", id, x, y);

        if id as usize >= self.dyn_sprites.len() {
            return;
        }

        let mut sprite = self.dyn_sprites[id as usize].clone();

        if flip_x {
            sprite = DynSprite::new_from_matrix(sprite.flip_x());
        }

        if flip_y {
            sprite = DynSprite::new_from_matrix(sprite.flip_y());
        }

        let nrows = sprite.data.nrows();
        let ncols = sprite.data.ncols();

        for j in 0..nrows {
            for i in 0..ncols {
                let c = sprite.data[(i + j * ncols) as usize];
                if !self.is_transparent(c) {
                    self.putpixel_(i as i32 + x, j as i32 + y, c);
                }
            }
        }
    }

    pub fn spr_dyn_load(&mut self, data: Vec<u8>, width: u32, height: u32) -> i32 {
        debug!("Load dynamic sprite {:?} {:?}", width, height);

        let mut idx = 0;
        let mut v: Vec<u32> = Vec::new();

        while idx < data.len() {
            let r = data[idx];
            let g = data[idx + 1];
            let b = data[idx + 2];

            v.push(px8::PALETTE.lock().unwrap().add_color(r, g, b));

            idx += 3;
        }

        let s = DynSprite::new(v, width, height);

        self.dyn_sprites.push(s.clone());

        (self.dyn_sprites.len() as i32) - 1
    }

    pub fn map(&mut self,
               cel_x: u32,
               cel_y: u32,
               sx: i32,
               sy: i32,
               cel_w: u32,
               cel_h: u32,
               layer: u8) {
        let mut idx_x;
        let mut idx_y: i32 = 0;

        let mut cel_w = cel_w;
        if cel_w > px8::MAP_WIDTH as u32 {
            cel_w = px8::MAP_WIDTH as u32;
        }

        let mut cel_h = cel_h;
        if cel_h > px8::MAP_HEIGHT as u32 {
            cel_h = px8::MAP_HEIGHT as u32;
        }

        debug!("MAP cel_x {:?} cel_y {:?} sx {:?} sy {:?} cel_w {:?} cel_h {:?} layer {:?}",
               cel_x,
               cel_y,
               sx,
               sy,
               cel_w,
               cel_h,
               layer);

        while idx_y < cel_h as i32 {
            idx_x = 0;
            while idx_x < cel_w as i32 {
                let orig_x = sx + 8 * idx_x;

                let mut new_x = orig_x;
                let mut new_y = sy + 8 * idx_y;

                let map_x = cel_x as i32 + idx_x;
                let map_y = cel_y as i32 + idx_y;

                debug!("MAP X {:?} MAP Y {:?}", map_x, map_y);

                let idx_sprite = self.map[(map_x as usize) % px8::MAP_WIDTH][(map_y as usize) %
                px8::MAP_HEIGHT];

                // Skip the sprite 0
                if idx_sprite != 0 {
                    let sprite = self.sprites[idx_sprite as usize].clone();
                    debug!("GET SPRITE {:?}, {:?} {:?}", idx_sprite, map_x, map_y);

                    // not the correct layer
                    if layer == 0 || sprite.is_bit_flags_set(layer) {
                        let mut index = 0;

                        for c in &sprite.data {
                            if !self.is_transparent(*c as u32) {
                                self.putpixel_(new_x, new_y, *c as u32);
                            }

                            index += 1;

                            if index > 0 && index % 8 == 0 {
                                new_y += 1;
                                new_x = orig_x;
                            } else {
                                new_x += 1;
                            }
                        }
                    }
                }

                idx_x += 1;
            }

            idx_y += 1;
        }
    }

    pub fn mget(&mut self, x: i32, y: i32) -> u32 {
        debug!("MGET x {:?} y {:?}", x, y);

        if x < 0 || y < 0 {
            return 0;
        }

        if x as usize > px8::MAP_WIDTH || y as usize >= px8::MAP_HEIGHT {
            return 0;
        }

        self.map[x as usize][y as usize]
    }

    pub fn mset(&mut self, x: i32, y: i32, v: u32) {
        debug!("MSET x {:?} y {:?} v {:?}", x, y, v);

        if x < 0 || y < 0 {
            return;
        }

        if x as usize > px8::MAP_WIDTH || y as usize >= px8::MAP_HEIGHT {
            return;
        }

        self.map[x as usize][y as usize] = v;
    }

    pub fn sspr(&mut self,
                sx: u32,
                sy: u32,
                sw: u32,
                sh: u32,
                dx: i32,
                dy: i32,
                dw: u32,
                dh: u32,
                flip_x: bool,
                flip_y: bool) {
        debug!("SSPR sx {:?} sy {:?} sw {:?} sh {:?} dx {:?} dy {:?} dw {:?} dh {:?} flip_x {:?} flip_y {:?}",
               sx,
               sy,
               sw,
               sh,
               dx,
               dy,
               dw,
               dh,
               flip_x,
               flip_y);

        let mut v = Vec::new();

        for y in sy..sy + sh {
            for x in sx..sx + sw {
                v.push(self.sget(x, y));
            }
        }

        debug!("SSPR V {:?} {:?}", v.len(), v);

        let mut x2;
        let mut y2;

        let w1 = sw;
        let w2 = dw;

        let h1 = sh;
        let h2 = dh;

        let x_ratio;
        let y_ratio;

        let mut ret = Vec::with_capacity((w2 * h2) as usize);

        x_ratio = ((w1 << 16) / w2) + 1;
        y_ratio = ((h1 << 16) / h2) + 1;

        debug!("SSPR H1 {:?} W1 {:?} H2 {:?} W2 {:?} X RATIO {:?} Y RATIO {:?}",
               h1,
               w1,
               h2,
               w2,
               x_ratio,
               y_ratio);

        for i in 0..h2 {
            for j in 0..w2 {
                x2 = (j * x_ratio) >> 16;
                y2 = (i * y_ratio) >> 16;
                ret.insert((i * w2 + j) as usize, v[(y2 * w1 + x2) as usize]);
            }
        }

        debug!("SSPR OUTPUT RET {:?} {:?}", ret.len(), ret);

        if flip_x {
            for i in 0..w2 / 2 {
                for j in 0..h2 {
                    ret.swap((i + j * w2) as usize, ((w2 - (i + 1)) + j * w2) as usize);
                }
            }
        }

        if flip_y {
            for i in 0..h2 / 2 {
                for j in 0..w2 {
                    ret.swap((j + i * w2) as usize, (j + (h2 - (i + 1)) * w2) as usize);
                }
            }
        }

        let mut idx = 0;
        for j in 0..h2 {
            for i in 0..w2 {
                let d: u8 = ret[idx];
                idx += 1;
                if d != 0 {
                    if !self.is_transparent(d as u32) {
                        self.putpixel_(i as i32 + dx, j as i32 + dy, d as u32);
                    }
                }
            }
        }
    }

    pub fn is_transparent(&self, value: u32) -> bool {
        if value <= 255 {
            self.transparency_map[value as usize]
        } else {
            false
        }
    }

    pub fn pal(&mut self, c0: i32, c1: i32) {
        if c0 < 0 || c1 < 0 {
            self._reset_colors();
        } else {
            self.color_map[c0 as usize] = c1 as u8;
        }
    }

    pub fn palt(&mut self, c: i32, t: bool) {
        if c == -1 {
            self._reset_transparency();
        } else if (c >= 0) && (c <= 255) {
            self.transparency_map[c as usize] = t;
        }
    }

    pub fn peek(&mut self, addr: u32) -> u8 {
        self.back_buffer[addr as usize]
    }

    pub fn poke(&mut self, _addr: u32, _val: u16) {}

    pub fn memcpy(&mut self, dest_addr: u32, source_addr: u32, len: u32) {
        let mut idx = 0;

        let dest_addr = dest_addr * 2;
        let source_addr = source_addr * 2;

        debug!("MEMPCY dest_addr {:?}, source_addr {:?}, len {:?}",
               dest_addr,
               source_addr,
               len);

        let a = &self.back_buffer[source_addr as usize..(source_addr + len * 2) as usize].to_vec();

        while idx < len * 2 {
            let value = a[idx as usize] as u32;

            self.back_buffer[(dest_addr + idx) as usize] = value as u8;

            idx += 1;
        }
    }

    pub fn memset(&mut self, _dest_addr: u32, _val: u32, _len: u32) {}
}
