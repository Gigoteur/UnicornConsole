pub mod fps;

use std::fmt;

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
//

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

pub struct Screen {
    pub back_buffer: Box<px8::ScreenBuffer>,
    pub sprites: Vec<Sprite>,
    pub map: [[u32; 32]; px8::SCREEN_WIDTH],
    pub transparency: [u8; 16],
    pub colors: [px8::Color; 16],
    pub camera: Camera,
    pub color: px8::Color,
}

unsafe impl Send for Screen {}
unsafe impl Sync for Screen {}

impl Screen {
    pub fn new() -> Screen {
        Screen { back_buffer: Box::new(px8::SCREEN_EMPTY),
                 sprites: Vec::new(),
                 map: [[0; 32]; px8::SCREEN_WIDTH],
                 transparency: [0; 16],
                 colors: [px8::Color::Black; 16],
                 camera: Camera::new(),
                 color: px8::Color::Black,
        }
    }

    pub fn init(&mut self) {
        self._reset_colors();
        self.palt(0, true);
    }

    pub fn _reset_colors(&mut self) {
        self.colors[0] = px8::Color::Black;
        self.colors[1] = px8::Color::DarkBlue;
        self.colors[2] = px8::Color::DarkPurple;
        self.colors[3] = px8::Color::DarkGreen;
        self.colors[4] = px8::Color::Brown;
        self.colors[5] = px8::Color::DarkGray;
        self.colors[6] = px8::Color::LightGray;
        self.colors[7] = px8::Color::White;
        self.colors[8] = px8::Color::Red;
        self.colors[9] = px8::Color::Orange;
        self.colors[10] = px8::Color::Yellow;
        self.colors[11] = px8::Color::Green;
        self.colors[12] = px8::Color::Blue;
        self.colors[13] = px8::Color::Indigo;
        self.colors[14] = px8::Color::Pink;
        self.colors[15] = px8::Color::Peach;
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

    pub fn putpixel_(&mut self, x: usize, y: usize, col: px8::Color) {
        //debug!("PUTPIXEL x:{:?} y:{:?} -> {:?}", x, y, col);

        let x = (x as i32 - self.camera.x) as usize;
        let y = (y as i32 - self.camera.y) as usize;
        let mut col = col;

        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return;
        }

        col = self.colors[col as usize];

        self.back_buffer[x + y * SCREEN_WIDTH] = col;
    }

    pub fn color(&mut self, col: px8::Color) {
        self.color = col;
    }

    pub fn putpixel(&mut self, x: usize, y: usize, col: px8::Color) {
        return self.putpixel_(x, y, col);
    }

    pub fn getpixel(&mut self, x: usize, y: usize) -> u8 {
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return 0;
        }

        return self.back_buffer[x + y * SCREEN_WIDTH] as u8;
    }

    pub fn pget(&mut self, x: u32, y: u32) -> u8 {
        let col = self.getpixel(x as usize, y as usize);
        return col;
    }


    pub fn pset(&mut self, x: u32, y: u32, col: px8::Color) {
        let mut col = col;
        if col == px8::Color::UNKNOWN {
            col = self.color;
        }

        self.putpixel_(x as usize, y as usize, col);
    }

    pub fn sget(&mut self, x: u32, y: u32) -> u8 {
        let idx_sprite = (x/8) + 16 * (y/8);
        let sprite = &self.sprites[idx_sprite as usize];
        return *sprite.data.get(((x%8) + (y % 8) * 8) as usize).unwrap();
    }

    pub fn sset(&mut self, x: u32, y: u32, col: px8::Color) {
        let mut col = col;
        if col == px8::Color::UNKNOWN {
            col = self.color;
        }

        let idx_sprite = (x/8) + 16 * (y/8);
        let ref mut sprite = self.sprites[idx_sprite as usize];
        sprite.set_data(((x%8) + (y % 8) * 8) as usize, px8::Color::to_u8(col));
    }

    pub fn cls(&mut self) {
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                self.putpixel(x, y, px8::Color::Black);
            }
        }
    }

    pub fn print(&mut self, string: String, x: i32, y: i32, col: px8::Color) {
        let mut x = x as u32;
        let y = y as u32;

        let mut col = col;
        if col == px8::Color::UNKNOWN {
            col = self.color;
        }

        for k in 0..string.len() {
            let value = string.as_bytes()[k] as usize;
            let data;

            if value >= 32 && value <= 126 {
                data = GLYPH[value - 32];
            } else { /* Unknown char, replace by a space */
                data = [0x0000, 0x0000];
            }

            let mut idx = 1;
            for i in 0..32 {
               if (data[idx] & (0x1 << i)) != 0 {
                    self.pset(x, y + i % 8, col)
                }

                if i % 8 == 7 {
                    x = x + 1;
                }
                if i == 15 {
                    idx = 0;
                }
            }
        }
    }

    pub fn line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, col: px8::Color) {
        let x0: i32 = x0 as i32;
        let x1: i32 = x1 as i32;
        let y0: i32 = y0 as i32;
        let y1: i32 = y1 as i32;

        let mut col = col;
        if col == px8::Color::UNKNOWN {
            col = self.color;
        }

        let (mut x0, mut y0) = (x0, y0);
        let (x1, y1) = (x1, y1);

        let dx = (x1 - x0).abs();

        let sx = if x0 < x1 { 1 } else { -1 };
        let dy: i32 = -1 * (y1 - y0).abs();
        let sy: i32 = if y0 < y1 { 1 } else { -1 };
        let mut err: i32 = dx + dy; /* error value e_xy */

        loop {
            self.putpixel(x0 as usize, y0 as usize, col);
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

    pub fn hline(&mut self, x1: u32, x2: u32, y: u32, col: px8::Color) {
        self.line(x1, y, x2, y, col);
    }

    pub fn rect(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, col: px8::Color) {
        self.line(x0, y0, x0, y1, col);
        self.line(x0, y0, x1, y0, col);
        self.line(x0, y1, x1, y1, col);
        self.line(x1, y0, x1, y1, col);
    }

    pub fn rectfill(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, col: px8::Color) {
        self.line(x0, y0, x0, y1, col);
        self.line(x0, y0, x1, y0, col);
        self.line(x0, y1, x1, y1, col);
        self.line(x1, y0, x1, y1, col);


        for y in y0..y1 {
            self.line(x0, y, x1, y, col)
        }
    }

    pub fn square(&mut self, x0: u32, y0: u32, h: u32, col: px8::Color) {
        self.rect(x0, y0, x0+h, y0+h, col);
    }

    pub fn squarefill(&mut self, x0: u32, y0: u32, h: u32, col: px8::Color) {
        self.rectfill(x0, y0, x0+h, y0+h, col);
    }

    pub fn circ(&mut self, x: u32, y: u32, r: u32, col: px8::Color) {
        if r <= 0 {
            return;
        }

        let mut col = col;
        if col == px8::Color::UNKNOWN {
            col = self.color;
        }

        let x = x as i32;
        let y = y as i32;

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
                    self.putpixel(xmj as usize, ypi as usize, col);
                    self.putpixel(xpj as usize, ypi as usize, col);
                    self.putpixel(xmj as usize, ymi as usize, col);
                    self.putpixel(xpj as usize, ymi as usize, col);
                } else {
                    self.putpixel(xmj as usize, y as usize, col);
                    self.putpixel(xpj as usize, y as usize, col);
                }


                oi = i;
                xmk = x - k;
                xpk = x + k;
                if h > 0 {
                    yph = y + h;
                    ymh = y - h;
                    self.putpixel(xmk as usize, yph as usize, col);
                    self.putpixel(xpk as usize, yph as usize, col);
                    self.putpixel(xmk as usize, ymh as usize, col);
                    self.putpixel(xpk as usize, ymh as usize, col);
                } else {
                    self.putpixel(xmk as usize, y as usize, col);
                    self.putpixel(xpk as usize, y as usize, col);
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

    pub fn circfill(&mut self, x: u32, y: u32, r: u32, col: px8::Color) {
        if r <= 0 {
            return;
        }

        let mut col = col;
        if col == px8::Color::UNKNOWN {
            col = self.color;
        }

        let x = x as i32;
        let y = y as i32;

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
                    self.hline(xmj as u32, xpj as u32, (y + i) as u32, col);
                    self.hline(xmj as u32, xpj as u32, (y - i) as u32, col);
                } else {
                    self.hline(xmj as u32, xpj as u32, y as u32, col);
                }
                oi = i;
            }
            if (oh != h) && (oi != h) && (i != h) {
                xmk = x - k;
                xpk = x + k;
                if h > 0 {
                    self.hline(xmk as u32, xpk as u32, (y + h) as u32, col);
                    self.hline(xmk as u32, xpk as u32, (y - h) as u32, col);
                } else {
                    self.hline(xmk as u32, xpk as u32, y as u32, col);
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

    pub fn trigon(&mut self, x1: u32, y1: u32, x2: u32, y2: u32, x3: u32, y3: u32, col: px8::Color) {
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


    pub fn polygon(&mut self, vx: Vec<u32>, vy: Vec<u32>, col: px8::Color) {
        if vx.len() < 3 || vy.len() < 3 {
            return;
        }

        if vx.len() != vy.len() {
            return;
        }

        let mut idx = 0;

        while idx < vx.len() - 1 {
/*            self.line(*vx.get(idx).unwrap(),
                      *vy.get(idx).unwrap(),
                      *vx.get(idx+1).unwrap(),
                      *vy.get(idx+1).unwrap(),
                      col);*/

            self.line(vx[idx],
                      vy[idx],
                      vx[idx+1],
                      vy[idx+1],
                      col);


            idx += 1;
        }

        self.line(*vx.get(idx).unwrap(),
                  *vy.get(idx).unwrap(),
                  *vx.get(0).unwrap(),
                  *vy.get(0).unwrap(),
                  col);

    }

    pub fn spr(&mut self, n: u32, x: u32, y: u32, w: u32, h: u32, flip_x: bool, flip_y: bool) {
        let sprites_number = w * h;

        debug!("PRINT SPRITE = {:?} x:{:?} y:{:?} n:{:?} w:{:?} h:{:?} flip_x:{:?} flip_y:{:?}", sprites_number, x, y, n, w, h, flip_x, flip_y);

        let mut idx_w = 0;

        let mut orig_x = x;
        let mut orig_y = y;

        for i in 0..sprites_number {
            let mut sprite = self.sprites[(n+i) as usize].clone();
            if flip_x {
                sprite = sprite.flip_x();
            }
            if flip_y {
                sprite = sprite.flip_y();
            }

            let mut new_x = orig_x % 128;
            let mut new_y = orig_y;

            debug!("SPRITE = {:?} x:{:?} y:{:?} {:?}", (n+i) as usize, new_x, new_y, sprite);

            let mut index = 0;
            for c in &sprite.data {
                if self.transparency[*c as usize] == 0 {
                    self.putpixel_(new_x as usize, new_y as usize, px8::Color::from_u8(*c));
                }

                index = index + 1;

                if index != 0 && index % 8 == 0 {
                    new_y = new_y + 1;
                    new_x = orig_x % 128;
                } else {
                    new_x = new_x + 1;
                }
            }

            idx_w += 1;
            orig_x += 8;

            if idx_w == w {
                orig_y += 8;
                idx_w = 0;
                orig_x  = 0;
            }
        }
    }

    pub fn map(&mut self, cel_x: u32, cel_y: u32, sx: u32, sy: u32, cel_w: u32, cel_h: u32) {
        let mut idx_x = 0;
        let mut idx_y = 0;

        let mut cel_w = cel_w;
        if cel_w > 128 {
            cel_w = 128;
        }

        let mut cel_h = cel_h;
        if cel_h > 32 {
            cel_h = 32;
        }

        debug!("cel_x {:?} cel_y {:?} sx {:?} sy {:?} cel_w {:?} cel_h {:?}", cel_x, cel_y, sx, sy, cel_w, cel_h);

        while idx_y < cel_h {
            idx_x = 0;
            while idx_x < cel_w {
                let orig_x = sx + 8 * idx_x;

                let mut new_x = orig_x;
                let mut new_y = sy + 8 * idx_y;

                if new_x > 128 || new_y > 128 {
                    break
                }

                let mut map_x = (cel_x + idx_x) as i32;
                let mut map_y = (cel_y + idx_y) as i32;

                let idx_sprite = self.map[map_x as usize][map_y as usize];

                let sprite = self.sprites[idx_sprite as usize].clone();
                
                let mut index = 0;
                for c in &sprite.data {
                    if self.transparency[*c as usize] == 0 {
                        self.putpixel_(new_x as usize, new_y as usize, px8::Color::from_u8(*c));
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

    pub fn sspr(&mut self, sx: u32, sy: u32, sw: u32, sh: u32, dx: u32, dy: u32, dw: u32, dh: u32, flip_x: bool, flip_y: bool) {
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
                    if self.transparency[d as usize] == 0 {
                        self.putpixel_((i + dx) as usize, (j + dy) as usize, px8::Color::from_u8(d));
                    }
                }
            }
        }
    }

    pub fn pal(&mut self, c0: i32, c1: i32) {
        if c0 == -1 && c1 == -1 {
            self._reset_colors();
        } else {
            self.colors[c0 as usize] = px8::Color::from_u8(c1 as u8);
        }
    }

    pub fn palt(&mut self, c: u32, t: bool) {
        self.transparency[c as usize] = t as u8;
    }
}
