use std::fmt;
use std::collections::HashMap;

use px8;

/// Emulated screen width in pixels
pub const SCREEN_WIDTH: usize = px8::SCREEN_WIDTH;
/// Emulated screen height in ixels
pub const SCREEN_HEIGHT: usize = px8::SCREEN_HEIGHT;
/// Screen texture size in bytes
pub const SCREEN_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub const GLYPH : [[u16; 2]; 95]  = [
    [0x0000, 0x0000], // space
    [0x0000, 0x1700], // !
    [0x0003, 0x0003], // "
    [0x001f, 0x0a1f], // #
    [0x000d, 0x1f0b], // $
    [0x0013, 0x0419], // %
    [0x0018, 0x171f], // &
    [0x0000, 0x0102], // '
    [0x0000, 0x211e], // (
    [0x001e, 0x2100], // )
    [0x0015, 0x0e15], // *
    [0x0004, 0x0e04], // +
    [0x0000, 0x1020], // ,
    [0x0004, 0x0404], // -
    [0x0000, 0x2000], // .
    [0x0001, 0x1e20], // /

    [0x003e, 0x223e], // 0
    [0x0020, 0x3e22], // 1
    [0x002e, 0x2a3a], // 2
    [0x003e, 0x2a22], // 3
    [0x003e, 0x080e], // 4
    [0x003a, 0x2a2e], // 5
    [0x0038, 0x283e], // 6
    [0x003e, 0x0202], // 7
    [0x003e, 0x2a3e], // 8
    [0x003e, 0x0a0e], // 9

    [0x0000, 0x0000], // :
    [0x0000, 0x1700], // ;
    [0x0010, 0x0e01], // <
    [0x0003, 0x0003], // =
    [0x001f, 0x0a1f], // >
    [0x000d, 0x1f0b], // ?
    [0x0013, 0x0419], // @

    [0x1e09, 0x091e], // A
    [0x0a15, 0x151f], // B
    [0x0a11, 0x110e], // C
    [0x0e11, 0x111f], // D
    [0x1115, 0x151f], // E
    [0x0105, 0x051f], // F
    [0x0c15, 0x110e], // G
    [0x1f08, 0x081f], // H
    [0x1111, 0x1f11], // I
    [0x010f, 0x1108], // J
    [0x110a, 0x041f], // K
    [0x1010, 0x101f], // L
    [0x1f07, 0x071f], // M
    [0x1f04, 0x021f], // N
    [0x0e11, 0x110e], // O
    [0x0609, 0x091f], // P
    [0x1619, 0x110e], // Q
    [0x0609, 0x091f], // R
    [0x0915, 0x1512], // S
    [0x0101, 0x1f01], // T
    [0x0f10, 0x100f], // U
    [0x0304, 0x081f], // V
    [0x1f18, 0x181f], // W
    [0x1b04, 0x041b], // X
    [0x0304, 0x1c03], // Y
    [0x1315, 0x1519], // Z

    [0x0000, 0x0000], // [
    [0x0010, 0x0e01], // \
    [0x0000, 0x1700], // ]
    [0x0010, 0x0e01], // ^
    [0x0003, 0x0003], // _
    [0x001f, 0x0a1f], // `

    [0x001c, 0x1408], // a
    [0x0008, 0x141f], // b
    [0x0014, 0x1408], // c
    [0x001f, 0x1408], // d
    [0x0014, 0x140c], // e
    [0x0005, 0x1e04], // f
    [0x003c, 0x5458], // g
    [0x0018, 0x041f], // h
    [0x0000, 0x1d00], // i
    [0x0000, 0x1d20], // j
    [0x0014, 0x081f], // k
    [0x0000, 0x100f], // l
    [0x001c, 0x0c1c], // m
    [0x0018, 0x041c], // n
    [0x0008, 0x1408], // o
    [0x0018, 0x147c], // p
    [0x007c, 0x140c], // q
    [0x0004, 0x0418], // r
    [0x0004, 0x1c10], // s
    [0x0014, 0x0e04], // t
    [0x001c, 0x100c], // u
    [0x000c, 0x180c], // v
    [0x001c, 0x181c], // w
    [0x0014, 0x0814], // x
    [0x003c, 0x505c], // y
    [0x0010, 0x1c04], // z

    [0x000d, 0x1f0b], // {
    [0x000d, 0x1f0b], // |
    [0x0013, 0x0419], // }
    [0x0013, 0x0419], // ~
];

#[derive(Clone)]
pub struct Sprite {
    pub data: Vec<u8>,
}

impl Sprite {
    pub fn new(d: [u8; 8 * 8]) -> Sprite {
        let mut v = Vec::new();
        v.extend(d.iter().cloned());

        Sprite { data: v }
    }

    pub fn set_data(&mut self, idx: usize, col: u8) {
        self.data[idx] = col;
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for c in self.data.clone() {
            data.push_str(&format!("{:?}", c));
        }

        return data;
    }

    pub fn get_line(&mut self, line: u32) -> String {
        let mut data = String::new();

        let mut data_clone = self.data.clone();

        let data_line: Vec<_> = data_clone.drain((line*8) as usize..(line*8+8)as usize).collect();

        for c in data_line.clone() {
            data.push_str(&format!("{:x}", c));
        }

        return data;
    }

    pub fn transpose(&mut self) -> [u8; 64] {
        let mut ret : [u8; 64] = [0; 8 * 8];

        for i in 0..8 {
            for j in 0..8 {
                ret[(j+i*8) as usize] = self.data[(i + j * 8) as usize];
            }
        }

        return ret;
    }

    pub fn horizontal_reflection(&mut self) -> [u8; 64] {
        let mut ret: [u8; 64] = self.to_u8_64_array();


        for i in 0..4 {
            for j in 0..8 {
                let tmp = ret[(i + j * 8) as usize];
                ret[(i + j * 8) as usize] = ret[((8 - (i+1)) + j * 8) as usize];
                ret[((8 - (i+1)) + j * 8) as usize] = tmp;
            }
        }

        return ret;
    }

    pub fn vertical_reflection(&mut self) -> [u8; 64] {
        let mut ret: [u8; 64] = self.to_u8_64_array();

        for i in 0..4 {
            for j in 0..8 {
                let tmp = ret[(j + i * 8) as usize];
                ret[(j + i * 8) as usize] = ret[(j + (8 - (i+1)) * 8) as usize];
                ret[(j + (8 - (i+1)) * 8) as usize] = tmp;
            }
        }

        return ret;
    }

    pub fn flip_x(&mut self) -> Sprite {
        return Sprite::new(self.horizontal_reflection());
    }

    pub fn flip_y(&mut self) -> Sprite {
        return Sprite::new(self.vertical_reflection());
    }


    pub fn to_u8_64_array(&mut self) -> [u8;64] {
        let mut arr = [0u8;64];
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
            data_matrix.push_str(format!("{:?}", &self.data[i*8..i*8+8]).as_str());
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
        Camera {x: 0, y: 0}
    }
}

pub struct Clipping {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
    pub clipped: bool,
}

impl Clipping {
    pub fn new() -> Clipping {
        Clipping {x: 0, y: 0, w: 0, h: 0, clipped: false}
    }
}

pub struct Screen {
    pub back_buffer: Box<px8::ScreenBuffer>,
    pub saved_back_buffer: Box<px8::ScreenBuffer>,
    pub sprites: Vec<Sprite>,
    pub map: [[u32; 32]; px8::SCREEN_WIDTH],
    pub transparency: HashMap<u32, u8>,
    pub colors: HashMap<u32, u32>,
    pub camera: Camera,
    pub color: u32,
    pub clipping: Clipping,
}

unsafe impl Send for Screen {}
unsafe impl Sync for Screen {}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            back_buffer: Box::new(px8::SCREEN_EMPTY),
            saved_back_buffer: Box::new(px8::SCREEN_EMPTY),

            sprites: Vec::new(),
            map: [[0; 32]; px8::SCREEN_WIDTH],

            transparency: HashMap::new(),
            colors:  HashMap::new(),
            color: 0,

            camera: Camera::new(),

            clipping: Clipping::new(),
        }
    }

    pub fn init(&mut self) {
        self._reset_colors();
        self._reset_transparency();
    }

    pub fn _reset_transparency(&mut self) {
        self.transparency.clear();
        self.transparency.insert(0, 1);
    }

    pub fn _reset_colors(&mut self) {
        self.colors.clear();
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
        // no specified color
        if col == -1 {
            return self.color;
        }

        return col as u32;
    }

    pub fn camera(&mut self, x: i32, y: i32) {
        if x == -1 && y == -1 {
            self.camera.x = 0;
            self.camera.y = 0;
        } else {
            self.camera.x = x;
            self.camera.y = y;
        }
    }

    pub fn set_sprites(&mut self, sprites: Vec<Sprite>) {
        self.sprites = sprites;
    }

    pub fn set_map(&mut self, map: [[u32; 32]; px8::SCREEN_WIDTH]) {
        self.map = map;
    }

    pub fn putpixel_(&mut self, x: i32, y: i32, col: u32) {
        // Camera
        let x = (x as i32 - self.camera.x) as usize;
        let y = (y as i32 - self.camera.y) as usize;
        let mut col = col;

        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return;
        }

        // Clipped
        if self.clipping.clipped {
            let x = x as u32;
            let y = y as u32;

            if !(x >= self.clipping.x && x <= self.clipping.x+self.clipping.w) {
                return;
            }
            if !(y >= self.clipping.y && y <= self.clipping.y+self.clipping.h) {
                return;
            }
        }

        match self.colors.get(&col) {
            Some(&value) => col = value,
            None => (),
        }
     //   col = self.colors[col as usize];

        self.back_buffer[x + y * SCREEN_WIDTH] = col;
    }

    pub fn color(&mut self, col: i32) {
        if col != -1 {
            self.color = col as u32;
        }
    }

    pub fn set_color(&mut self, col: i32, r: u8, g: u8, b: u8) {
        if col != -1 {
            px8::set_color(col as u32, r, g, b);
        }
    }

    pub fn reset_colors(&mut self) {
        px8::reset_colors();
    }

    pub fn switch_palette(&mut self, name: String) {
      //  px8::switch_palette(name);
    }

    pub fn putpixel(&mut self, x: i32, y: i32, col: u32) {
        return self.putpixel_(x, y, col);
    }

    pub fn getpixel(&mut self, x: usize, y: usize) -> u32 {
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return 0;
        }

        return self.back_buffer[x + y * SCREEN_WIDTH] as u32;
    }

    pub fn pget(&mut self, x: u32, y: u32) -> u32 {
        let col = self.getpixel(x as usize, y as usize);
        return col;
    }


    pub fn pset(&mut self, x: i32, y: i32, col: i32) {
        let color = self._find_color(col);
        self.putpixel_(x, y, color);
    }

    pub fn sget(&mut self, x: u32, y: u32) -> u8 {
        let idx_sprite = (x / 8) + 16 * (y / 8);
        let sprite = &self.sprites[idx_sprite as usize];
        return *sprite.data.get(((x % 8) + (y % 8) * 8) as usize).unwrap();
    }

    pub fn sset(&mut self, x: u32, y: u32, col: i32) {
        let col = self._find_color(col);

        let idx_sprite = (x / 8) + 16 * (y / 8);
        let ref mut sprite = self.sprites[idx_sprite as usize];
        sprite.set_data(((x % 8) + (y % 8) * 8) as usize, col as u8);
    }

    pub fn cls(&mut self) {
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                self.putpixel(x as i32, y as i32, 0);
            }
        }
    }

    pub fn print(&mut self, string: String, x: i32, y: i32, col: i32) {
        let mut x = x;
        let y = y;

        for k in 0..string.len() {
            let value = string.as_bytes()[k] as usize;
            let data;

            if value >= 32 && value <= 126 {
                data = GLYPH[value - 32];
            } else {
                /* Unknown char, replace by a space */
                data = [0x0000, 0x0000];
            }

            let mut idx = 1;
            let mut idx_1 = 0;

            for i in 0..32 {
                if (data[idx] & (0x1 << idx_1)) != 0 {
                    self.pset(x, y + i % 8, col)
                }

                idx_1 += 1;

                if i % 8 == 7 {
                    x = x + 1;
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
        let dy: i32 = -1 * (y1 - y0).abs();
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
        self.line(x1, y, x2, y, col);
    }

    pub fn rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, col: i32) {
        self.line(x0, y0, x0, y1, col);
        self.line(x0, y0, x1, y0, col);
        self.line(x0, y1, x1, y1, col);
        self.line(x1, y0, x1, y1, col);
    }

    pub fn rectfill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, col: i32) {
        self.line(x0, y0, x0, y1, col);
        self.line(x0, y0, x1, y0, col);
        self.line(x0, y1, x1, y1, col);
        self.line(x1, y0, x1, y1, col);


        for y in y0..y1 {
            self.line(x0, y, x1, y, col)
        }
    }

    pub fn square(&mut self, x0: i32, y0: i32, h: i32, col: i32) {
        self.rect(x0, y0, x0 + h, y0 + h, col);
    }

    pub fn squarefill(&mut self, x0: i32, y0: i32, h: i32, col: i32) {
        self.rectfill(x0, y0, x0 + h, y0 + h, col);
    }

    pub fn circ(&mut self, x: i32, y: i32, r: i32, col: i32) {
        if r <= 0 {
            return;
        }

        let col = self._find_color(col);

        let mut h: i32;
        let mut i: i32;
        let mut j: i32;
        let mut k: i32;

        let mut oh: i32 = 0xFFFF;
        let mut oi: i32 = 0xFFFF;

        let mut ix: i32;
        let mut iy: i32;

        let rx: i32 = r as i32;
        let ry: i32 = r as i32;

        let mut xmj: i32;
        let mut xpj: i32;
        let mut ymi: i32;
        let mut ypi: i32;

        let mut xmk: i32;
        let mut xpk: i32;
        let mut ymh: i32;
        let mut yph: i32;

        ix = 0;
        iy = ry * 64;

        h = (ix + 32) >> 6;
        i = (iy + 32) >> 6;
        j = (h * rx) / ry;
        k = (i * rx) / ry;

        while i > h {
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

            ix = ix + iy / ry;
            iy = iy - ix / ry;

            h = (ix + 32) >> 6;
            i = (iy + 32) >> 6;
            j = (h * rx) / ry;
            k = (i * rx) / ry;
        }
    }

    pub fn circfill(&mut self, x: i32, y: i32, r: i32, col: i32) {
        if r <= 0 {
            return;
        }

        let mut h: i32;
        let mut i: i32;
        let mut j: i32;
        let mut k: i32;

        let mut oh: i32 = 0xFFFF;
        let mut oi: i32 = 0xFFFF;

        let mut ix: i32;
        let mut iy: i32;

        let rx: i32 = r as i32;
        let ry: i32 = r as i32;

        let mut xmj: i32;
        let mut xpj: i32;

        let mut xmk: i32;
        let mut xpk: i32;

        ix = 0;
        iy = ry * 64;

        h = (ix + 32) >> 6;
        i = (iy + 32) >> 6;
        j = (h * rx) / ry;
        k = (i * rx) / ry;


        while i > h {
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

            ix = ix + iy / ry;
            iy = iy - ix / ry;


            h = (ix + 32) >> 6;
            i = (iy + 32) >> 6;
            j = (h * rx) / ry;
            k = (i * rx) / ry;
        }
    }

    pub fn clip(&mut self, x: i32, y: i32, w: i32, h: i32) {
        // reset
        if x == -1 && y == -1 && w == -1 && h == -1 {
            self.clipping.clipped = false;
        }

        // invalid clipping value
        if x == -1 || y == -1 || w == -1 || h == -1 {
            return;
        }

        if x < 0 || y  < 0 || w  < 0 || h  < 0 {
            return;
        }

        self.clipping.x = x as u32;
        self.clipping.y = y as u32;
        self.clipping.w = w as u32;
        self.clipping.h = h as u32;

        self.clipping.clipped = true;
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
            self.line(vx[idx],
                      vy[idx],
                      vx[idx + 1],
                      vy[idx + 1],
                      col);


            idx += 1;
        }

        self.line(*vx.get(idx).unwrap(),
                  *vy.get(idx).unwrap(),
                  *vx.get(0).unwrap(),
                  *vy.get(0).unwrap(),
                  col);
    }

    pub fn spr(&mut self, n: u32, x: i32, y: i32, w: u32, h: u32, flip_x: bool, flip_y: bool) {
        let sprites_number = w * h;

        debug!("PRINT SPRITE = {:?} x:{:?} y:{:?} n:{:?} w:{:?} h:{:?} flip_x:{:?} flip_y:{:?}", sprites_number, x, y, n, w, h, flip_x, flip_y);

        let mut idx_w = 0;

        let mut orig_x = x;
        let mut orig_y = y;

        for i in 0..sprites_number {
            let mut sprite = self.sprites[(n + i) as usize].clone();
            if flip_x {
                sprite = sprite.flip_x();
            }
            if flip_y {
                sprite = sprite.flip_y();
            }

            let mut new_x = orig_x % SCREEN_WIDTH as i32;
            let mut new_y = orig_y;

            debug!("SPRITE = {:?} x:{:?} y:{:?} {:?}", (n + i) as usize, new_x, new_y, sprite);

            let mut index = 0;
            for c in &sprite.data {
                if ! self.is_transparent(*c as u32) {
                    self.putpixel_(new_x, new_y, *c as u32);
                }

                index = index + 1;

                if index != 0 && index % 8 == 0 {
                    new_y = new_y + 1;
                    new_x = orig_x % SCREEN_WIDTH as i32;
                } else {
                    new_x = new_x + 1;
                }
            }

            idx_w += 1;
            orig_x += 8;

            if idx_w == w {
                orig_y += 8;
                idx_w = 0;
                orig_x = 0;
            }
        }
    }

    pub fn map(&mut self, cel_x: u32, cel_y: u32, sx: i32, sy: i32, cel_w: u32, cel_h: u32) {
        let mut idx_x: i32 = 0;
        let mut idx_y: i32 = 0;

        let mut cel_w = cel_w;
        if cel_w > SCREEN_WIDTH as u32 {
            cel_w = SCREEN_WIDTH as u32;
        }

        let mut cel_h = cel_h;
        if cel_h > 32 {
            cel_h = 32;
        }

        debug!("cel_x {:?} cel_y {:?} sx {:?} sy {:?} cel_w {:?} cel_h {:?}", cel_x, cel_y, sx, sy, cel_w, cel_h);

        while idx_y < cel_h as i32 {
            idx_x = 0;
            while idx_x < cel_w as i32 {
                let orig_x = sx + 8 * idx_x;

                let mut new_x = orig_x;
                let mut new_y = sy + 8 * idx_y;

                if new_x > SCREEN_WIDTH as i32 || new_y > SCREEN_HEIGHT as i32 {
                    break
                }

                let mut map_x = cel_x as i32 + idx_x;
                let mut map_y = cel_y as i32 + idx_y;

                let idx_sprite = self.map[map_x as usize][map_y as usize];

                let sprite = self.sprites[idx_sprite as usize].clone();

                let mut index = 0;
                for c in &sprite.data {
                    if ! self.is_transparent(*c as u32) {
                        self.putpixel_(new_x, new_y, *c as u32);
                    }

                    index = index + 1;

                    if index > 0 && index % 8 == 0 {
                        new_y = new_y + 1;
                        new_x = orig_x;
                    } else {
                        new_x = new_x + 1;
                    }
                }

                idx_x += 1;
            }

            idx_y += 1;
        }
    }

    pub fn mget(&mut self, x: u32, y: u32) -> u32 {
        if x as usize > px8::SCREEN_WIDTH || y as usize > px8::SCREEN_WIDTH {
            return 0;
        }

        let value = self.map[x as usize][y as usize];

        return value;
    }

    pub fn mset(&mut self, x: u32, y: u32, v: u32) {
        if x as usize > px8::SCREEN_WIDTH || y as usize > px8::SCREEN_WIDTH {
            return;
        }

        self.map[x as usize][y as usize] = v;
    }

    pub fn sspr(&mut self, sx: u32, sy: u32, sw: u32, sh: u32, dx: i32, dy: i32, dw: u32, dh: u32, flip_x: bool, flip_y: bool) {
        let mut v = Vec::new();

        for x in sx..sx+sw {
            for y in sy..sy+sh {
                v.push(self.sget(x, y));
            }
        }

        let mut x2;
        let mut y2;

        let w1 = sw;
        let w2 = dw;

        let h1 = sh;
        let h2 = dh;

        let mut x_ratio;
        let mut y_ratio;

        let mut ret = Vec::with_capacity((w2 * h2) as usize);

        x_ratio = ((w1 << 16)/w2) + 1;
        y_ratio = ((h1 << 16)/h2) + 1;

        for i in 0..h2 {
            for j in 0..w2 {
                x2 = (j * x_ratio)>>16;
                y2 = (i * y_ratio)>>16;

                ret.insert((i*w2+j) as usize, *v.get((y2*w1+x2) as usize).unwrap());
            }
        }

        if flip_x {
            for i in 0..w2/2 {
                for j in 0..h2 {
                    let tmp = ret[(i + j * w2) as usize];
                    ret[(i + j * w2) as usize] = ret[((w2 - (i+1)) + j * w2) as usize];
                    ret[((w2 - (i+1)) + j * w2) as usize] = tmp;
                }
            }
        }

        if flip_y {
            for i in 0..h2/2 {
                for j in 0..w2 {
                    let tmp = ret[(j + i * w2) as usize];
                    ret[(j + i * w2) as usize] = ret[(j + (h2 - (i+1)) * w2) as usize];
                    ret[(j + (h2 - (i+1)) * w2) as usize] = tmp;
                }
            }
        }

        let mut idx = 0;
        for i in 0..h2 {
            for j in 0..w2 {
                let d:u8 = *ret.get(idx).unwrap();
                idx += 1;
                if d != 0 {
                    if ! self.is_transparent(d as u32) {
                        self.putpixel_(i as i32 + dx, j as i32 + dy, d as u32);
                    }
                    //if self.transparency[d as usize] == 0 {
                    //    self.putpixel_(i as i32 + dx, j as i32 + dy, px8::Color::from_u8(d));
                   // }
                }
            }
        }
    }

    pub fn is_transparent(&mut self, value: u32) -> bool {
        match self.transparency.get(&(value as u32)) {
            Some(&1) => {
                return true;
            },
            Some(&_) => (),
            None => (),
        }
        return false;
    }

    pub fn pal(&mut self, c0: i32, c1: i32) {
        if c0 < 0 || c1 < 0 {
            self._reset_colors();
        } else {
            self.colors.insert(c0 as u32, c1 as u32);
        }
    }

    pub fn palt(&mut self, c: i32, t: bool) {
        if c == -1 {
            self._reset_transparency();
        } else {
            self.transparency.insert(c as u32, t as u8);
        }
    }
}
