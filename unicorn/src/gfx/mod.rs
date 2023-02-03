pub mod fonts;
pub mod camera;
pub mod clip;
pub mod palette;
pub mod sprite;
pub mod framerate;

use log::{debug, error, info};

use std::collections::HashMap;
use std::cmp;
use std::f64::consts::PI;

use gfx::palette::RGB;


#[derive(Debug)]
pub struct Screen {
    pub width: usize,
    pub height: usize,

    pub map_width: usize,
    pub map_height: usize,

    pub pixel_buffer: Box<[u8]>,

    pub palettes: palette::Palettes,
    pub palette: palette::Palette,
    pub current_palette_name: String,

    pub sprites: Vec<sprite::Sprite>,
    pub dyn_sprites: Vec<sprite::DynamicSprite>,

    pub map: Vec<u32>,

    pub transparency_map: [u8; 256],
    pub fillp_pat: u32,
    pub fillp_transparent: bool,

    pub color: u32,
    pub color_map: [u8; 0xFFF],

    pub camera: camera::Camera,
    pub cliprect: clip::ClipRect,
    
    pub font: &'static fonts::Font,
}

impl Screen {
    pub fn new(width: usize, height: usize, map_width: usize, map_height: usize) -> Screen {
        info!("[GFX] [Screen] Creating Screen [width:{:?} height:{:?}, map_width:{:?}, map_height:{:?}]", width, height, map_width, map_height);
        
        let pixel_buffer = (0..(width * height)*4)
            .map(|_| 0)
            .collect::<Vec<u8>>()
            .into_boxed_slice();


        Screen {
            width: width,
            height: height,

            map_width: map_width,
            map_height: map_height,

            pixel_buffer: pixel_buffer,
            palettes: palette::Palettes::new(),
            palette: palette::Palette::new(),
            current_palette_name: "".to_string(),

            sprites: Vec::new(),
            dyn_sprites: Vec::new(),
            
            map: Vec::new(),
            transparency_map: [0xFF; 256],
            fillp_pat: 0,
            fillp_transparent: false,
            color_map: [0; 0xFFF],
            
            color: 0,
            camera: camera::Camera::new(),
            
            cliprect: clip::ClipRect::new(),
            font: &fonts::pico8::FONT,
        }
    }

    pub fn resize_buffer(&mut self, width: usize, height: usize) {
        info!("[GFX] [Screen] Resizing buffer {:?}x{:?}", width, height);

        if width != self.map_width || height != self.map_height {
            self.pixel_buffer = (0..(width * height)*4)
                .map(|_| 0)
                .collect::<Vec<u8>>()
                .into_boxed_slice();
            self.width = width;
            self.height = height;
        }
    }

    pub fn reset(&mut self) {
        self._reset_colors();
        self._reset_transparency();
        self._reset_fillp();
        self._reset_cliprect();
        self._reset_palettes();
        self._reset_palette();
        self._reset_sprites();
        self._reset_map();

        self.color = 0;
    }

    pub fn mode_width(&mut self) -> usize {
        self.width
    }


    pub fn mode_height(&mut self) -> usize {
        self.height
    }
    
    pub fn _reset_sprites(&mut self) {
        debug!("[GFX] [Screen] Reset sprites");
        self.sprites = Vec::new();
        self.dyn_sprites = Vec::new();
    }

    pub fn _reset_map(&mut self) {
        debug!("[GFX] [Screen] Reset map");
        self.map = Vec::new();
    }


    pub fn _reset_palettes(&mut self) {
        debug!("[GFX] [Screen] Reset palettes");
        self.palettes.reset();
    }

    pub fn _reset_palette(&mut self) {
        debug!("[GFX] [Screen] Reset palette");

        self.switch_palette("pico-8".to_string());
    }


    pub fn _reset_fillp(&mut self) {
        debug!("[GFX] [Screen] Reset fillp");

        self.fillp_pat = 0;
        self.fillp_transparent = false;
    }

    pub fn _reset_transparency(&mut self) {
        debug!("[GFX] [Screen] Reset transparency");

        self.transparency_map = [0xFF; 256];
        self.transparency_map[0] = 0;
    }

    pub fn _reset_colors(&mut self) {
        debug!("[GFX] [Screen] Reset colors");

        for i in 0..0xFFF {
            self.color_map[i] = i as u8;
        }
    }

    pub fn _reset_cliprect(&mut self) {
        debug!("[GFX] [Screen] Reset cliprect");

        self.cliprect = clip::ClipRect {
            left: 0,
            top: 0,
            right: self.width as i32,
            bottom: self.height as i32,
        };
    }

    pub fn switch_palette(&mut self, name: String) {
        info!("[GFX] [Screen] Switch palette to {:?}", name);

        self.current_palette_name = name.clone();

        let values = &self.palettes.palettes[&name];

        for (idx, rgb_value) in values.iter().enumerate() {
            self.palette._set_color(idx as u32, rgb_value.r, rgb_value.g, rgb_value.b);
        }
    }

    pub fn set_palette_colors(&mut self, colors: HashMap<u32, RGB>) {
        self.palette.set_colors(colors);
    }

    pub fn set_palette_color(&mut self, color: u32, r: u8, g: u8, b: u8) {
        self.palette.set_color(color, r, g, b);
    }

    pub fn save(&mut self) {
        info!("[GFX] [Screen] SAVE SCREEN");
       // self.saved_frame_buffer.copy_from_slice(&self.frame_buffer);
    }

    pub fn restore(&mut self) {
        info!("[GFX] [Screen] Restore SCREEN");
      //  self.frame_buffer.copy_from_slice(&self.saved_frame_buffer);
    }

    pub fn get_palette_rgb(&mut self, value: u32) -> palette::RGB {
        return self.palette.get_rgb(value);
    }

    #[inline]
    pub fn _find_color(&mut self, col: i32) -> u32 {
        if col == -1 { self.color } else { col as u32 }
    }

    pub fn camera(&mut self, x: i32, y: i32) {
        self.camera.x = x;
        self.camera.y = y;
    }

    pub fn set_sprites(&mut self, sprites: Vec<sprite::Sprite>) {
        info!("Set Sprites {:?}", sprites.len());
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
        ((x as usize) + ((y as usize) * self.width))*4
    }

    #[inline]
    pub fn putpixel_direct(&mut self, x: i32, y: i32, col: u32) {
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32 {
            return;
        }

        let offset = self.pixel_offset(x, y);

        let rgb = self.palette.get_rgb(col as u32);
        self.pixel_buffer[offset] = rgb.r;
        self.pixel_buffer[offset + 1] = rgb.g;
        self.pixel_buffer[offset + 2] = rgb.b;
        self.pixel_buffer[offset + 3] = self.transparency_map[col as usize];     
    }

    #[inline]
    pub fn putpixel_(&mut self, x: i32, y: i32, col: u32, fillp_flag: bool) {
        //debug!("[SCREEN] [Screen] [Putpixel_] x:{:?} y:{:?} col:{:?}", x, y, col);
        
        if self.is_transparent(col as u32) {
            return;
        }

        // Make camera adjustment
        let x = x - self.camera.x;
        let y = y - self.camera.y;

        // Clip
        if !self.cliprect.contains(x, y) {
            return;
        }

        if fillp_flag == false || self.fillp_pat == 0 {
            if col < self.color_map.len() as u32 {
                let draw_col = self.color_map[col as usize];

                let offset = self.pixel_offset(x, y);

                let rgb = self.palette.get_rgb(draw_col as u32);
                self.pixel_buffer[offset] = rgb.r;
                self.pixel_buffer[offset + 1] = rgb.g;
                self.pixel_buffer[offset + 2] = rgb.b;
                self.pixel_buffer[offset + 3] = self.transparency_map[draw_col as usize];   
            }
        } else {
            let value = (self.fillp_pat >> (15 - (x & 3) - 4 * (y & 3))) & 0x1;
            let draw_col;

            if value == 0 {
                draw_col = self.color_map[(col & 0xF) as usize];
            } else {
                draw_col = self.color_map[((col & 0xF0) >> 4 )as usize];
            }

            let offset = self.pixel_offset(x, y);

            if value == 0 || (value == 1 && !self.fillp_transparent) {
                let rgb = self.palette.get_rgb(draw_col as u32);
                self.pixel_buffer[offset] = rgb.r;
                self.pixel_buffer[offset + 1] = rgb.g;
                self.pixel_buffer[offset + 2] = rgb.b;
                self.pixel_buffer[offset + 3] = self.transparency_map[draw_col as usize];     
            }
        }
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
    pub fn putpixel(&mut self, x: i32, y: i32, col: u32, fillp_flag: bool) {
        self.putpixel_(x, y, col, fillp_flag);
    }

    #[inline]
    pub fn getpixel(&mut self, x: usize, y: usize) -> u32 {
        let x = (x as i32 - self.camera.x) as usize;
        let y = (y as i32 - self.camera.y) as usize;

        if x >= self.width || y >= self.height {
            return 0;
        }

        let offset = self.pixel_offset(x as i32, y as i32);
        
        let r = self.pixel_buffer[offset];
        let g = self.pixel_buffer[offset + 1];
        let b = self.pixel_buffer[offset + 2];
        
        return self.palette.get_color_rgb(r, g, b)
    }

    pub fn pget(&mut self, x: u32, y: u32) -> u32 {
        self.getpixel(x as usize, y as usize) as u32
    }

    pub fn pset(&mut self, x: i32, y: i32, col: i32) {
        let color = self._find_color(col);
        self.putpixel_(x, y, color, true);
    }

    pub fn sget(&mut self, x: i32, y: i32) -> u8 {
        //info!("SGET x:{:?} y:{:?}", x, y);

        if x < 0 || y < 0 {
            return 0;
        }

        if x as usize >= self.mode_width() || y as usize >= self.mode_height() {
            return 0;
        }

        let idx_sprite = ((x / 8) + 16 * (y / 8)) as u32;
        //info!("SGET IDX {:?}/{:?}", idx_sprite, self.sprites.len());

        let sprite = &self.sprites[idx_sprite as usize];
        sprite.data[((x % 8) + (y % 8) * 8) as usize] as u8
    }

    pub fn sprite_set(&mut self, idx_sprite: i32, x: u32, y: u32, col: i32) {
        let col = self._find_color(col);

        let sprite = &mut self.sprites[idx_sprite as usize];
        sprite.set_data(((x % 8) + (y % 8) * 8) as usize, col as u8);
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

    pub fn cls(&mut self, value: i8) {
        let mut nvalue = value as u8;
        if value == -1 {
            nvalue = 0;
        }
        let rgb = self.palette.get_rgb(nvalue as u32);
        self.pixel_buffer
        .chunks_exact_mut(4)
        .for_each(|pixel| pixel.copy_from_slice(&rgb.into_pixel_data()));

        self._reset_cliprect();
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
        let initial_x = x;
        let mut x = x;
        let mut y = y + self.font.top_bearing;

        for c in string.as_bytes() {
            if *c == 0x0A { // \n
                x = initial_x;
                y = y + self.font.advance_width;
            } else if *c == 0x0D { // \r
                x = initial_x;
            } else {
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
                                self.putpixel(x + dx, y + (i as i32), col as u32, false);
                            }
                        }
                        row <<= 1;
                        dx += 1;
                    }
                }
                x += self.font.advance_width;
            }
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
            self.putpixel(x0, y0, color, true);
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
            self.putpixel(x, y, col as u32, true);
        }
    }

    pub fn rect(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, col: i32) {
        let x_min = cmp::min(x0, x1);
        let x_max = cmp::max(x0, x1);
        let y_min = cmp::min(y0, y1);
        let y_max = cmp::max(y0, y1);

        for x in x_min..(x_max + 1) {
            self.putpixel(x, y_min, col as u32, true);
            self.putpixel(x, y_max, col as u32, true);
        }
        for y in (y_min + 1)..y_max {
            self.putpixel(x0, y, col as u32, true);
            self.putpixel(x1, y, col as u32, true);
        }
    }

    pub fn rectfill(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, col: i32) {
        debug!("[GFX][rectfill] x0:{:?} y0:{:?} x1:{:?} y1:{:?} col:{:?}", x0, y0, x1, y1, col);
        
        let x_min = cmp::min(x0, x1);
        let x_max = cmp::max(x0, x1);
        let y_min = cmp::min(y0, y1);
        let y_max = cmp::max(y0, y1);

        for y in y_min..(y_max + 1) {
            for x in x_min..(x_max + 1) {
                self.putpixel(x, y, col as u32, true);
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
            .intersect(&clip::ClipRect {
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

                        self.putpixel(xmh, ypk, col, true);
                        self.putpixel(xph, ypk, col, true);
                        self.putpixel(xmh, ymk, col, true);
                        self.putpixel(xph, ymk, col, true);
                    } else {
                        self.putpixel(xmh, y, col, true);
                        self.putpixel(xph, y, col, true);
                    }

                    ok = k;
                    xpi = x + i;
                    xmi = x - i;
                    if j > 0 {
                        ypj = y + j;
                        ymj = y - j;
                        self.putpixel(xmi, ypj, col, true);
                        self.putpixel(xpi, ypj, col, true);
                        self.putpixel(xmi, ymj, col, true);
                        self.putpixel(xpi, ymj, col, true);
                    } else {
                        self.putpixel(xmi, y, col, true);
                        self.putpixel(xpi, y, col, true);
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
                        self.putpixel(xmj, ypi, col, true);
                        self.putpixel(xpj, ypi, col, true);
                        self.putpixel(xmj, ymi, col, true);
                        self.putpixel(xpj, ymi, col, true);
                    } else {
                        self.putpixel(xmj, y, col, true);
                        self.putpixel(xpj, y, col, true);
                    }


                    oi = i;
                    xmk = x - k;
                    xpk = x + k;
                    if h > 0 {
                        yph = y + h;
                        ymh = y - h;
                        self.putpixel(xmk, yph, col, true);
                        self.putpixel(xpk, yph, col, true);
                        self.putpixel(xmk, ymh, col, true);
                        self.putpixel(xpk, ymh, col, true);
                    } else {
                        self.putpixel(xmk, y, col, true);
                        self.putpixel(xpk, y, col, true);
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
                        self.hline(xmj, xpj, y + i, col);
                        self.hline(xmj, xpj, y - i, col);
                    } else {
                        self.hline(xmj, xpj, y, col);
                    }
                    oi = i;
                }
                if (oh != h) && (oi != h) && (i != h) {
                    xmk = x - k;
                    xpk = x + k;
                    if h > 0 {
                        self.hline(xmk, xpk, y + h, col);
                        self.hline(xmk, xpk, y - h, col);
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

    pub fn spr_reg(&mut self, n: i64, data: Vec<u8>, width: u32, height: u32) -> i64 {

        let mut idx = 0;
        let mut v: Vec<u8> = Vec::new();

        while idx < data.len() {
            let r = *data.get(idx).unwrap();
            let g = *data.get(idx + 1).unwrap();
            let b = *data.get(idx + 2).unwrap();

            v.push(self.palette.add_color(r, g, b));

            idx += 3;
        }

        //info!("DYNAMIC SPRITE {:?} {:?} width:{:?} height:{:?}", v, v.len(), width, height);
        
        let dyn_sprite = sprite::DynamicSprite::new(v, width, height);
        if n == -1 {
            self.dyn_sprites.push(dyn_sprite);
            return (self.dyn_sprites.len() - 1) as i64;
        }
        if n >= self.dyn_sprites.len() as i64 {
            return -1;
        }
        self.dyn_sprites[n as usize] = dyn_sprite;
        return n;
    }

    pub fn spr(&mut self, n: u32,
               x: i32, y: i32, 
               w: i32, h: i32,
               flip_x: bool, flip_y: bool,
               angle: f64, zoom: f64,
               dynamic: bool) {
        debug!("[SCREEN] [Screen] [SPR] n:{:?} x:{:?} y:{:?} w:{:?} h:{:?} flip_x:{:?} flip_y:{:?} angle:{:?} zoom:{:?} dynamic:{:?}",
               n,
               x,
               y,
               w,
               h,
               flip_x,
               flip_y,
               angle, zoom,
               dynamic);

        if w < -1 || h < -1 {
            return;
        }

        if dynamic {
            let sprite = self.dyn_sprites[n as usize].clone();
            if w != sprite.width as i32 || h != sprite.height as i32 {
                let mut w2 = w as u32;
                let mut h2 = h as u32;

                if w == -1 {
                    w2 = sprite.width as u32;
                }

                if h == -1 {
                    h2 = sprite.height as u32;
                }

                let mut ret = Vec::with_capacity((w2 * h2) as usize);

                let x_ratio: u32 = (sprite.width << 16) / w2;
                let y_ratio: u32 = (sprite.height << 16) / h2;

                let mut x2: u32;
                let mut y2: u32;

                for i in 0..h2 {
                    for j in 0..w2 {
                        x2 = (j * x_ratio) >> 16;
                        y2 = (i * y_ratio) >> 16;
                        let idx = (y2 * sprite.width + x2) as usize;
                        ret.insert((i * w2 + j) as usize, sprite.data[idx]);
                    }
                }

                self._sprite_rotazoom(
                    ret.clone(),
                    w2,
                    h2,
                    x,
                    y,
                    angle,
                    zoom,
                    flip_x, flip_y);

            } else {
                //info!("DYNAMIC SPR {:?} {:?} {:?}", sprite.data.clone(), sprite.width, sprite.height);
/* 
                self._sprite_quick(
                    sprite.data.clone(),
                    sprite.width,
                    sprite.height,
                    x,
                    y,
                    flip_x,
                    flip_y);*/
                    
                self._sprite_rotazoom(
                    sprite.data.clone(),
                    sprite.width,
                    sprite.height,
                    x,
                    y,
                    angle,
                    zoom,
                    flip_x, flip_y);
                }

        } else {
            let mut orig_x = x;
            let mut orig_y = y;

            if flip_x {
                orig_x = (w * 8 - 8) + x;
            }

            if flip_y {
                orig_y = (h * 8 - 8) + y;
            }

            let sprites_len = self.sprites.len();
            for i in 0..h {
                for j in 0..w {
                    let sprite_offset = ((j + n as i32) + i * 16) as usize;
                    if sprite_offset >= sprites_len {
                        break;
                    }

                    let sprite = self.sprites[sprite_offset].clone();
                    debug!("[SCREEN] [Screen] [SPR] Access to sprite {:?} {:?}", sprite_offset, sprite);

                    self._sprite_rotazoom(
                        sprite.data.clone().to_vec(),
                        8, 8,
                        orig_x, orig_y, angle, zoom,
                        flip_x, flip_y);

                    if flip_x {
                        orig_x -= 8;
                    } else {
                        orig_x += 8;
                    }
                }

                if flip_y {
                    orig_y -= 8;
                } else {
                    orig_y += 8;
                }

                if flip_x {
                    orig_x = (w * 8 - 8) + x;
                } else {    
                    orig_x = x;
                }
            }
        }
    }
 
    pub fn mapdraw(&mut self,
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
        if cel_w > self.map_width as u32 {
            cel_w = self.map_width as u32;
        }

        let mut cel_h = cel_h;
        if cel_h > self.map_height as u32 {
            cel_h = self.map_height as u32;
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

                let idx_sprite: u32 = *self.map.get(map_x as usize  + map_y as usize * self.map_width).unwrap_or(&0);

                //info!("GET SPRITE {:?}, {:?} {:?} {:?} {:?}", idx_sprite, map_x, map_y, new_x, new_y);

                // not the correct layer
                if idx_sprite != 0 {
                    let mut sprite = self.sprites[idx_sprite as usize].clone();

                    if layer == 0 || sprite.is_bit_flags_set(layer) {
                        let mut index = 0;

                        for (_, c) in sprite.data.iter_mut().enumerate() {
                            self.putpixel_(new_x, new_y, *c as u32, false);

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
        //info!("MGET x {:?} y {:?}", x, y);

        if x < 0 || y < 0 {
            return 0;
        }

        if x as usize > self.map_width || y as usize >= self.map_height {
            return 0;
        }

        *self.map.get(x as usize + self.map_width * y as usize).unwrap_or(&0)
    }

    pub fn mset(&mut self, x: i32, y: i32, v: u32) {
        //info!("MSET x {:?} y {:?} v {:?}", x, y, v);

        if x < 0 || y < 0 {
            return;
        }

        if x as usize > self.map_width || y as usize >= self.map_height {
            return;
        }

        self.map[x as usize + self.map_width * y as usize] = v;
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
                v.push(self.sget(x as i32, y as i32));
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
                    self.putpixel_(i as i32 + dx, j as i32 + dy, d as u32, false);
                }
                idx += 1;
            }
        }
    }

    #[inline]
    pub fn _sprite_rotazoom(&mut self, v: Vec<u8>, 
                            sw: u32,
                            sh: u32,
                            destx: i32,
                            desty: i32,
                            angle: f64,
                            zoom: f64,
                            flip_x: bool,
                            flip_y: bool) -> (i32, i32) {
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

        let xd = (sw as i32 - dw) << 15;
        let yd = (sh as i32 - dh) << 15;
        let ax = ((dstwidthhalf as i32) << 16) - (icos * dstwidthhalf as i32);
        let ay = ((dstheighthalf as i32) << 16) - (isin * dstwidthhalf as i32);

        //debug!("NEXT {:?} {:?} {:?} {:?}", xd, yd, ax, ay);

        let centerx = destx;// + (sw as i32 / 2);
        let centery = desty;// + (sh as i32 / 2);

        let destx = centerx;
        let desty = centery;
        
    //   let mut destx = destx - dw / 2;
    //   let mut desty = desty - dh / 2;

    // debug!("DEST {:?} {:?}", destx, desty);

        for y in 0..dh {
            let mut dy = dstheighthalf as i32 - y;
            let mut sdx = (ax + (isin * dy)) + xd;
            let mut sdy = (ay - (icos * dy)) + yd;
            
        //  debug!("DY {:?} SDX {:?} SDY {:?}", dy, sdx, sdy);

            for x in 0..dw {
                let mut dx = sdx >> 16;
                dy = sdy >> 16;

                if flip_x {
                    dx = (sw as i32 - 1) - dx;
                }
                if flip_y {
                    dy = (sh as i32 - 1) - dy;
                }

//                debug!("DX {:?} DY {:?}", dx, dy);
                if (dx >= 0) && (dy >= 0) && (dx < sw as i32) && (dy < sh as i32) {
                    let d = v[(dy * sw as i32 + dx) as usize];
                    if d != 0 {
                        self.putpixel_(x as i32 + destx, y as i32 + desty, d as u32, false);
                    }
                }

                sdx += icos;
                sdy += isin;
            }
        }

        (dw, dh)     
    }

    #[inline]
    pub fn _sprite_quick(&mut self, v: Vec<u8>, 
                            sw: u32,
                            sh: u32,
                            destx: i32,
                            desty: i32,
                            flip_x: bool,
                            flip_y: bool) -> (i32, i32) {
        let dw = sw as i32;
        let dh = sh as i32;

        for y in 0..dh {
            for x in 0..dw {
                let idx = (y*dh+x) as usize;

                let d = v[idx as usize];
                if d != 0 {
                    self.putpixel_(x as i32 + destx, y as i32 + desty, d as u32, false);
                }
            }
        }

        (dw, dh)
    }

    pub fn sspr_rotazoom(&mut self,
                         _idx_sprite: i32,
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
                v.push(self.sget(x as i32, y as i32));
            }
        }

        return self._sprite_rotazoom(v, sw, sh, destx, desty, angle, zoom, flip_x, flip_y)
    }

    #[inline]
    pub fn is_transparent(&self, value: u32) -> bool {
        if value <= 255 {
            self.transparency_map[value as usize] == 0
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

    pub fn palt(&mut self, c: i32, t: u8) {
        if c == -1 {
            self._reset_transparency();
        } else if (c >= 0) && (c <= 255) {
            self.transparency_map[c as usize] = t;
        }
    }

    pub fn fillp(&mut self, pat: u32, transparent: bool) {
        // info!("[Screen][GFX] Fillp {:x} {:?} {:#034b}", pat, pat, pat);
        
        self.fillp_pat = pat;
        self.fillp_transparent = transparent;
    }

    /* Pico8 Memory Emulation */
    pub fn peek(&mut self, addr: u32) -> u8 {
        if addr <= 0x0fff { // 	Sprite sheet (0-127)*
            let r = self.pixel_buffer[addr as usize];
            let g = self.pixel_buffer[addr as usize + 1];
            let b = self.pixel_buffer[addr as usize+ 2];
            
            return self.palette.get_color_rgb(r, g, b) as u8
        } else if addr >= 0x1000 && addr <= 0x1fff { // Sprite sheet (128-255)* / Map (rows 32-63) (shared)

        } else if addr >= 0x2000 && addr <= 0x2fff { // Map (rows 0-31)
        } else if addr >= 0x3000 && addr <= 0x30ff { // Sprite flags
        }

        0
    }

    /* Pico8 Memory Emulation */
    pub fn poke(&mut self, addr: u32, values: Vec<u8>) {
        if addr <= 0x0fff { // 	Sprite sheet (0-127)*

        } else if addr >= 0x1000 && addr <= 0x1fff { // Sprite sheet (128-255)* / Map (rows 32-63) (shared)

        } else if addr >= 0x2000 && addr <= 0x2fff { // Map (rows 0-31)
        } else if addr >= 0x3000 && addr <= 0x30ff { // Sprite flags
        }
    }

    /* Pico8 Memory Emulation */
    pub fn memcpy(&mut self, dest_addr: u32, source_addr: u32, len: u32) {/*
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
        }*/
    }

    /* Pico8 Memory Emulation */
    pub fn memset(&mut self, _dest_addr: u32, _val: u32, _len: u32) {

        
    }
}
