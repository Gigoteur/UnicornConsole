mod fonts;

use std::fmt;

use unicorn;
use std::cmp;
use std::ptr;
use num_traits::pow;
use std::f64;
use std::f64::consts::PI;

// Fixed pitch font definition
#[allow(dead_code)]
pub struct Font {
    // Width of glyph in pixels
    glyph_width: i32,
    // Height of glyph in pixels
    glyph_height: i32,
    // Number of x pixels before glyph
    left_bearing: i32,
    // Number of y pixels before glyph
    top_bearing: i32,
    // Horizontal distance to next character
    advance_width: i32,
    // Vertical distance between lines
    line_height: i32,
    // Glyph bitmap data - one byte per row, first bit in MSB
    glyph_data: &'static [u8],
    name: &'static str,
}

#[derive(Copy)]
pub struct Sprite {
    pub data: [u8; 64],
    pub flags: u8,
}

impl Clone for Sprite {
    fn clone(&self) -> Sprite {
        *self
    }
}

impl Sprite {
    pub fn new(d: [u8; 64]) -> Sprite {
        Sprite { data: d, flags: 0 }
    }

    pub fn is_flags_set(&self, value: u8) -> bool {
        (self.flags & pow(2, value as usize)) != 0
    }

    pub fn is_bit_flags_set(&self, value: u8) -> bool {
        (self.flags & value) != 0
    }

    pub fn get_flags(&self) -> u8 {
        self.flags
    }

    pub fn set_flag(&mut self, flag: u8, value: bool) {
        if value {
            self.flags |= pow(2, flag as usize);
        } else {
            self.flags &= !(1 << flag);
        }
    }

    pub fn set_flags(&mut self, flags: u8) {
        self.flags = flags;
    }

    pub fn set_data(&mut self, idx: usize, col: u8) {
        self.data[idx] = col;
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for (i, elem) in self.data.iter_mut().enumerate() {
            data.push_str(&format!("{:?}", elem));
        }

        data
    }

    pub fn get_line(&mut self, line: u32) -> String {
        let mut v = Vec::new();
        v.extend(self.data.iter().cloned());

        let mut data = String::new();

        let mut data_clone = v.clone();

        let data_line: Vec<_> = data_clone
            .drain((line * 8) as usize..(line * 8 + 8) as usize)
            .collect();

        for c in data_line.clone() {
            data.push_str(&format!("{:03x}", c));
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

#[cfg(test)]
mod tests {
    use super::Sprite;

    #[test]
    fn test_sprite_flags() {
        let mut s = Sprite::new([0; 64]);
        s.set_flag(0, true);
        assert_eq!(s.is_flags_set(0), true);

        s.set_flag(7, true);
        assert_eq!(s.is_flags_set(7), true);

        s.set_flag(7, false);
        assert_eq!(s.is_flags_set(7), false);
    }

    #[test]
    fn test_sprite_flags2() {
        let mut s = Sprite::new([0; 64]);
        s.set_flags(131);
        assert_eq!(s.is_flags_set(0), true);
        assert_eq!(s.is_flags_set(1), true);
        assert_eq!(s.is_flags_set(2), false);
        assert_eq!(s.is_flags_set(3), false);
        assert_eq!(s.is_flags_set(4), false);
        assert_eq!(s.is_flags_set(5), false);
        assert_eq!(s.is_flags_set(6), false);
        assert_eq!(s.is_flags_set(7), true);
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

// ClipRect rectangle is exclusive of right and bottom edges
pub struct ClipRect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl ClipRect {
    pub fn new() -> ClipRect {
        ClipRect {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    pub fn intersect(&mut self, other: &ClipRect) {
        self.left = cmp::max(self.left, other.left);
        self.top = cmp::max(self.top, other.top);
        self.right = cmp::min(self.right, other.right);
        self.bottom = cmp::min(self.bottom, other.bottom);
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        (x >= self.left) && (x < self.right) && (y >= self.top) && (y < self.bottom)
    }
}

pub struct Screen {
    pub width: usize,
    pub height: usize,
    pub aspect_ratio: f32,

    pub frame_buffer: Vec<u8>,
    pub saved_frame_buffer: Vec<u8>,
    pub sprites: Vec<Sprite>,

    pub map: Vec<u32>,

    pub transparency_map: [bool; 256],

    pub color: u32,
    pub color_map: [u8; 256],

    pub camera: Camera,
    pub cliprect: ClipRect,
    
    pub font: &'static Font,
}

unsafe impl Send for Screen {}
unsafe impl Sync for Screen {}

impl Screen {
    pub fn new(width: usize, height: usize) -> Screen {
        info!("Creating Screen. width:{} height:{}", width, height);
        Screen {
            width: width,
            height: height,
            frame_buffer: vec![0; width * height],
            saved_frame_buffer: vec![0; width * height],
            aspect_ratio: width as f32 / height as f32,
            sprites: Vec::new(),
            map: Vec::new(),
            transparency_map: [false; 256],
            color_map: [0; 256],
            color: 0,
            camera: Camera::new(),
            cliprect: ClipRect::new(),
            font: &fonts::pico8::FONT,
        }
    }

    pub fn init(&mut self) {
        self._reset_colors();
        self._reset_transparency();
        self._reset_cliprect();
        self.color = 0;
    }

    pub fn mode_width(&mut self) -> usize {
        self.width
    }


    pub fn mode_height(&mut self) -> usize {
        self.height
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

    pub fn _reset_cliprect(&mut self) {
        self.cliprect = ClipRect {
            left: 0,
            top: 0,
            right: self.width as i32,
            bottom: self.height as i32,
        };
    }

    pub fn save(&mut self) {
        info!("[GFX] SAVE SCREEN");
        self.saved_frame_buffer.copy_from_slice(&self.frame_buffer);
    }

    pub fn restore(&mut self) {
        info!("[GFX] Restore SCREEN");
        self.frame_buffer.copy_from_slice(&self.saved_frame_buffer);
    }

    #[inline]
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

    pub fn set_map(&mut self, map: Vec<u32>) {
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

    #[inline]
    pub fn pixel_offset(&self, x: i32, y: i32) -> usize {
        (x as usize) + ((y as usize) * self.width)
    }

    #[inline]
    pub fn putpixel_direct(&mut self, x: i32, y: i32, col: u32) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }

        let offset = self.pixel_offset(x, y);
        self.frame_buffer[offset] = col as u8;
    }

    #[inline]
    pub fn putpixel_(&mut self, x: i32, y: i32, col: u32) {
        // Make camera adjustment
        let x = x - self.camera.x;
        let y = y - self.camera.y;

        // Clip
        if !self.cliprect.contains(x, y) {
            return;
        }

        let draw_col = self.color_map[(col & 0xFF) as usize];

        let offset = self.pixel_offset(x, y);
        self.frame_buffer[offset] = draw_col;
    }

    #[inline]
    pub fn color(&mut self, col: i32) {
        if (col >= 0) && (col <= 255) {
            self.color = col as u32;
        }
    }

    pub fn font(&mut self, name: &str) {
        self.font = match name {
            "pico-8" => &fonts::pico8::FONT,
            "bbc" => &fonts::bbc::FONT,
            "cbmII" => &fonts::cbmii::FONT,
            "appleII" => &fonts::appleii::FONT,
            "trollmini" => &fonts::trollmini::FONT,
            _ => &fonts::pico8::FONT,
        }
    }

    pub fn get_font(&mut self) -> String {
        self.font.name.to_string().clone()
    }

    #[inline]
    pub fn putpixel(&mut self, x: i32, y: i32, col: u32) {
        self.putpixel_(x, y, col);
    }

    #[inline]
    pub fn getpixel(&mut self, x: usize, y: usize) -> u32 {
        let x = (x as i32 - self.camera.x) as usize;
        let y = (y as i32 - self.camera.y) as usize;

        if x >= self.width || y >= self.height {
            return 0;
        }

        self.frame_buffer[x + y * self.width] as u32
    }

    pub fn pget(&mut self, x: u32, y: u32) -> u32 {
        self.getpixel(x as usize, y as usize)
    }

    pub fn pset(&mut self, x: i32, y: i32, col: i32) {
        let color = self._find_color(col);
        self.putpixel_(x, y, color);
    }

    pub fn sget(&mut self, x: u32, y: u32) -> u32 {
        let idx_sprite = (x / 8) + 50 * (y / 8);
        let sprite = &self.sprites[idx_sprite as usize];
        sprite.data[((x % 8) + (y % 8) * 8) as usize] as u32
    }

    pub fn sset(&mut self, x: u32, y: u32, col: i32) {
        let col = self._find_color(col);

        let idx_sprite = (x / 8) + 50 * (y / 8);
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

    pub fn cls(&mut self, value: i8) {
        let mut nvalue = value as u8;
        if value == -1 {
            nvalue = 0;
        }
        // Maximum performance!
        unsafe {
            let fb_ptr = self.frame_buffer.as_mut_ptr();
            ptr::write_bytes(fb_ptr, nvalue, self.frame_buffer.len());
        }
    }

    pub fn force_print(&mut self, string: String, x: i32, y: i32, col: i32) {
        self._print(string, x, y, col, true);
    }


    pub fn print_char(&mut self, data: char, x: i32, y: i32, col: i32) {
        self._print(data.to_string(), x, y, col, false);
    }

    pub fn print(&mut self, string: String, x: i32, y: i32, col: i32) {
        self._print(string, x, y, col, false);
    }

    #[inline]
    pub fn _print(&mut self, string: String, x: i32, y: i32, col: i32, force: bool) {
        let mut x = x;
        let y = y + self.font.top_bearing;

        for c in string.as_bytes() {
            let glyph_index = if (*c < 32) || (*c > 126) { 0 } else { *c - 32 } as u32;

            let glyph_start = (glyph_index * (self.font.glyph_height as u32)) as usize;
            let glyph_end = glyph_start + (self.font.glyph_height as usize);

            let glyph_data = &self.font.glyph_data[glyph_start..glyph_end];

            for (i, glyph_row) in glyph_data.iter().enumerate() {
                let mut dx = self.font.left_bearing;
                let mut row = *glyph_row;
                while row != 0 {
                    if row & 0x80 != 0 {
                        if force {
                            self.putpixel_direct(x + dx, y + (i as i32), col as u32);
                        } else {
                            self.pset(x + dx, y + (i as i32), col);
                        }
                    }
                    row <<= 1;
                    dx += 1;
                }
            }

            x += self.font.advance_width;
        }
    }

    pub fn line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, col: i32) {
        // debug!("LINE {:?} {:?} {:?} {:?} {:?}", x0, y0, x1, y1, col);

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
        self._reset_cliprect();

        if x == -1 && y == -1 && w == -1 && h == -1 {
            return;
        }

        self.cliprect
            .intersect(&ClipRect {
                           left: x,
                           top: y,
                           right: x + w,
                           bottom: y + h,
                       });
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
        /* debug!("PRINT SPRITE = x:{:?} y:{:?} n:{:?} w:{:?} h:{:?} flip_x:{:?} flip_y:{:?}",
               x,
               y,
               n,
               w,
               h,
               flip_x,
               flip_y);*/

        let mut orig_x = x;
        let mut orig_y = y;

        let sprites_len = self.sprites.len();
        for i in 0..h {
            for j in 0..w {
                let sprite_offset = ((j + n) + i * 50) as usize;
                if sprite_offset >= sprites_len {
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

                /*                debug!("SPRITE = {:?} x:{:?} y:{:?} {:?}",
                       sprite_offset,
                       new_x,
                       new_y,
                       sprite);*/

                let mut index = 0;
                for (_, c) in sprite.data.iter_mut().enumerate() {
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
        if cel_w > unicorn::MAP_WIDTH as u32 {
            cel_w = unicorn::MAP_WIDTH as u32;
        }

        let mut cel_h = cel_h;
        if cel_h > unicorn::MAP_HEIGHT as u32 {
            cel_h = unicorn::MAP_HEIGHT as u32;
        }

        /*debug!("MAP cel_x {:?} cel_y {:?} sx {:?} sy {:?} cel_w {:?} cel_h {:?} layer {:?}",
               cel_x,
               cel_y,
               sx,
               sy,
               cel_w,
               cel_h,
               layer);*/

        while idx_y < cel_h as i32 {
            idx_x = 0;
            while idx_x < cel_w as i32 {
                let orig_x = sx + 8 * idx_x;

                let mut new_x = orig_x;
                let mut new_y = sy + 8 * idx_y;

                let map_x = cel_x as i32 + idx_x;
                let map_y = cel_y as i32 + idx_y;

                //debug!("MAP X {:?} MAP Y {:?}", map_x, map_y);

                let idx_sprite: u32 = *self.map.get(((map_x as usize) % unicorn::MAP_WIDTH) * unicorn::MAP_WIDTH + (map_y as usize) % unicorn::MAP_HEIGHT).unwrap_or(&0);
                //self.map[(map_x as usize) % unicorn::MAP_WIDTH][(map_y as usize) % unicorn::MAP_HEIGHT];

                // Skip the sprite 0
                if idx_sprite != 0 {
                    let mut sprite = self.sprites[idx_sprite as usize].clone();
                    //debug!("GET SPRITE {:?}, {:?} {:?}", idx_sprite, map_x, map_y);

                    // not the correct layer
                    if layer == 0 || sprite.is_bit_flags_set(layer) {
                        let mut index = 0;

                        for (_, c) in sprite.data.iter_mut().enumerate() {
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
        //debug!("MGET x {:?} y {:?}", x, y);

        if x < 0 || y < 0 {
            return 0;
        }

        if x as usize > unicorn::MAP_WIDTH || y as usize >= unicorn::MAP_HEIGHT {
            return 0;
        }

        0
        //self.map[x as usize][y as usize]
    }

    pub fn mset(&mut self, x: i32, y: i32, v: u32) {
        //info!("MSET x {:?} y {:?} v {:?}", x, y, v);

        if x < 0 || y < 0 {
            return;
        }

        if x as usize > unicorn::MAP_WIDTH || y as usize >= unicorn::MAP_HEIGHT {
            return;
        }

  //      self.map[x as usize][y as usize] = v;
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
        /*debug!("SSPR sx {:?} sy {:?} sw {:?} sh {:?} dx {:?} dy {:?} dw {:?} dh {:?} flip_x {:?} flip_y {:?}",
               sx,
               sy,
               sw,
               sh,
               dx,
               dy,
               dw,
               dh,
               flip_x,
               flip_y);*/

        let mut v = Vec::new();

        for y in sy..sy + sh {
            for x in sx..sx + sw {
                v.push(self.sget(x, y));
            }
        }

        // debug!("SSPR V {:?} {:?}", v.len(), v);

        let mut x2;
        let mut y2;

        let w1 = sw;
        let w2 = dw;

        let h1 = sh;
        let h2 = dh;

        let x_ratio;
        let y_ratio;

        let mut ret = Vec::with_capacity((w2 * h2) as usize);

        x_ratio = (w1 << 16) / w2;
        y_ratio = (h1 << 16) / h2;

        /*debug!("SSPR H1 {:?} W1 {:?} H2 {:?} W2 {:?} X RATIO {:?} Y RATIO {:?} RET {:?} V {:?}",
               h1,
               w1,
               h2,
               w2,
               x_ratio,
               y_ratio,
               ret.capacity(),
               v.len());*/

        for i in 0..h2 {
            for j in 0..w2 {
                x2 = (j * x_ratio) >> 16;
                y2 = (i * y_ratio) >> 16;
                let idx = (y2 * w1 + x2) as usize;
                ret.insert((i * w2 + j) as usize, v[idx]);
            }
        }

        //debug!("SSPR OUTPUT RET {:?} {:?}", ret.len(), ret);

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
                let d = ret[idx];
                if d != 0 {
                    if !self.is_transparent(d as u32) {
                        self.putpixel_(i as i32 + dx, j as i32 + dy, d as u32);
                    }
                }
                idx += 1;
            }
        }
    }

    pub fn sspr2(&mut self,
                sx: u32,
                sy: u32,
                sw: u32,
                sh: u32,
                destx: i32,
                desty: i32,
                angle: f64,
                zoom: f64,
                flip_x: bool,
                flip_y: bool) -> (i32, i32) {
        
        let mut v = Vec::new();

        for y in sy..sy + sh {
            for x in sx..sx + sw {
                v.push(self.sget(x, y));
            }
        }

        // algorithm from SDL_gfx
        // no rotation ?
       // if angle.abs() > 0.001 {
            let radangle = angle * (PI / 180.0);
            let mut sanglezoom = radangle.sin();
            let mut canglezoom = radangle.cos();

            sanglezoom *= zoom;
            canglezoom *= zoom;

        // debug!("SSPR2 {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?} -> {:?} {:?}", sx, sy, sw, sh, destx, desty, angle, radangle, zoom, flip_x, flip_y, sanglezoom, canglezoom);

            let x = sw as f64 / 2.0;
            let y = sh as f64 / 2.0;

            // debug!("X Y {:?} {:?}", x, y);

            let cx = canglezoom * x;
            let cy = canglezoom * y;
            let sx = sanglezoom * x;
            let sy = sanglezoom * y;

            let dstwidthhalf: f64 = (cx + sy).abs().max((cx - sy).abs()).max((-cx + sy).abs()).max((-cx - sy).abs()).max(1.0);
            let dstheighthalf: f64 = (sx + cy).abs().max((sx - cy).abs()).max((-sx + cy).abs()).max((-sx - cy).abs()).max(1.0);

            //debug!("DST HALF {:?} {:?}", dstwidthhalf, dstheighthalf);

            let dw = (2.0 * dstwidthhalf) as i32;
            let dh = (2.0 * dstheighthalf) as i32;

            let zoominv = 65536.0 / (zoom * zoom);

            let mut sanglezoominv = sanglezoom;
            let mut canglezoominv = canglezoom;
            sanglezoominv *= zoominv;
            canglezoominv *= zoominv;

            let isin = sanglezoominv as i32;
            let icos = canglezoominv as i32;

            //debug!("DST {:?} {:?} {:?} {:?} {:?} {:?} {:?} {:?}", sw, sh, dw, dh, sanglezoominv, canglezoominv, isin, icos);

            let xd = ((sw as i32 - dw) << 15);
            let yd = ((sh as i32 - dh) << 15);
            let ax = ((dstwidthhalf as i32) << 16) - (icos * dstwidthhalf as i32);
            let ay = ((dstheighthalf as i32) << 16) - (isin * dstwidthhalf as i32);

            //debug!("NEXT {:?} {:?} {:?} {:?}", xd, yd, ax, ay);

            let centerx = destx;// + (sw as i32 / 2);
            let centery = desty;// + (sh as i32 / 2);

            let mut destx = centerx;
            let mut desty = centery;
            
        //   let mut destx = destx - dw / 2;
        //   let mut desty = desty - dh / 2;

        // debug!("DEST {:?} {:?}", destx, desty);

            for y in 0..dh {
                let mut dy = dstheighthalf as i32 - y;
                let mut sdx = (ax + (isin * dy)) + xd;
                let mut sdy = (ay - (icos * dy)) + yd;
                
            //  debug!("DY {:?} SDX {:?} SDY {:?}", dy, sdx, sdy);

                for x in 0..dw {
                    let mut dx = (sdx >> 16);
                    dy = (sdy >> 16);

                    if flip_x {
                        dx = (sw as i32 - 1) - dx;
                    }
                    if flip_y {
                        dy = (sh as i32 - 1) - dy;
                    }

    //                debug!("DX {:?} DY {:?}", dx, dy);
                    if ((dx >= 0) && (dy >= 0) && (dx < sw as i32) && (dy < sh as i32)) {
                        let d = v[(dy * sw as i32 + dx) as usize];
                        if d != 0 {
                            if !self.is_transparent(d as u32) {
                                self.putpixel_(x as i32 + destx, y as i32 + desty, d as u32);
                            }
                        }
                    }

                    sdx += icos;
                    sdy += isin;
                }
            }

            (dw, dh)
     /*   } else {
            let mut dw = sw as i32 * zoom;
            let mut dh = sh as i32 * zoom;


            

        }*/
    }

    #[inline]
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
        self.frame_buffer[addr as usize]
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

        let a = &self.frame_buffer[source_addr as usize..(source_addr + len * 2) as usize].to_vec();

        while idx < len * 2 {
            let value = a[idx as usize] as u32;

            self.frame_buffer[(dest_addr + idx) as usize] = value as u8;

            idx += 1;
        }
    }

    pub fn memset(&mut self, _dest_addr: u32, _val: u32, _len: u32) {}
}
