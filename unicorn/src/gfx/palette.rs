use std::collections::HashMap;
use std::io::Cursor;
use std::io::prelude::*;
use log::{info};

#[derive(Debug)]
pub struct Palettes {
    pub palette_idx: u32,
    pub palettes: HashMap<String, Vec<RGB>>,
    pub palettes_list: Vec<String>,
    pub name: String,
}

impl Palettes {
    pub fn new() -> Palettes {
        Palettes {
            palette_idx: 0,
            palettes: HashMap::new(),
            palettes_list: Vec::new(),
            name: "".to_string(),
        }
    }

    pub fn reset(&mut self) {
        // load palettes statically for wasm
        self.load("a64".to_string(),
                  include_str!("./palettes/a64.gpl").to_string());
        self.load("apple-ii".to_string(),
                  include_str!("./palettes/apple-ii.gpl").to_string());
        self.load("arne-paldac".to_string(),
                  include_str!("./palettes/arne-paldac.gpl").to_string());
        self.load("arne16".to_string(),
                  include_str!("./palettes/arne16.gpl").to_string());
        self.load("arne32".to_string(),
                  include_str!("./palettes/arne32.gpl").to_string());
        self.load("atari2600-ntsc".to_string(),
                  include_str!("./palettes/atari2600-ntsc.gpl").to_string());
        self.load("atari2600-pal".to_string(),
                  include_str!("./palettes/atari2600-pal.gpl").to_string());
        self.load("cg-arne".to_string(),
                  include_str!("./palettes/cg-arne.gpl").to_string());
        self.load("cga".to_string(),
                  include_str!("./palettes/cga.gpl").to_string());
        self.load("commodore-plus4".to_string(),
                  include_str!("./palettes/commodore-plus4.gpl").to_string());
        self.load("commodore-vic20".to_string(),
                  include_str!("./palettes/commodore-vic20.gpl").to_string());
        self.load("commodore64".to_string(),
                  include_str!("./palettes/commodore64.gpl").to_string());
        self.load("copper-tech".to_string(),
                  include_str!("./palettes/copper-tech.gpl").to_string());
        self.load("cpc-boy".to_string(),
                  include_str!("./palettes/cpc-boy.gpl").to_string());
        self.load("db16".to_string(),
                  include_str!("./palettes/db16.gpl").to_string());
        self.load("db32".to_string(),
                  include_str!("./palettes/db32.gpl").to_string());
        self.load("edg16".to_string(),
                  include_str!("./palettes/edg16.gpl").to_string());
        self.load("edg32".to_string(),
                  include_str!("./palettes/edg32.gpl").to_string());
        self.load("eroge-copper".to_string(),
                  include_str!("./palettes/eroge-copper.gpl").to_string());
        self.load("gameboy-color-type1".to_string(),
                  include_str!("./palettes/gameboy-color-type1.gpl").to_string());
        self.load("gameboy".to_string(),
                  include_str!("./palettes/gameboy.gpl").to_string());
        self.load("google-ui".to_string(),
                  include_str!("./palettes/google-ui.gpl").to_string());
        self.load("jmp".to_string(),
                  include_str!("./palettes/jmp.gpl").to_string());
        self.load("mail24".to_string(),
                  include_str!("./palettes/mail24.gpl").to_string());
        self.load("master-system".to_string(),
                  include_str!("./palettes/master-system.gpl").to_string());
        self.load("monokai".to_string(),
                  include_str!("./palettes/monokai.gpl").to_string());
        self.load("nes-ntsc".to_string(),
                  include_str!("./palettes/nes-ntsc.gpl").to_string());
        self.load("nes".to_string(),
                  include_str!("./palettes/nes.gpl").to_string());
        self.load("pico-8".to_string(),
                  include_str!("./palettes/pico-8.gpl").to_string());
        self.load("psygnork".to_string(),
                  include_str!("./palettes/psygnork.gpl").to_string());
        self.load("smile-basic".to_string(),
                  include_str!("./palettes/smile-basic.gpl").to_string());
        self.load("solarized".to_string(),
                  include_str!("./palettes/solarized.gpl").to_string());
        self.load("teletext".to_string(),
                  include_str!("./palettes/teletext.gpl").to_string());
        self.load("vga-13h".to_string(),
                  include_str!("./palettes/vga-13h.gpl").to_string());
        self.load("web-safe-colors".to_string(),
                  include_str!("./palettes/web-safe-colors.gpl").to_string());
        self.load("win16".to_string(),
                  include_str!("./palettes/win16.gpl").to_string());
        self.load("x11".to_string(),
                  include_str!("./palettes/x11.gpl").to_string());
        self.load("zx-spectrum".to_string(),
                  include_str!("./palettes/zx-spectrum.gpl").to_string());
    }

    pub fn load(&mut self, name: String, data: String) {
        let buf_reader = Cursor::new(data);

        let mut values = Vec::new();

        for line in buf_reader.lines() {
            let line = line.unwrap();
            let l = line.trim_start().to_string();

            if l.is_empty() {
                continue;
            }

            if l.starts_with('#') {
                continue;
            }

            let l_b = l.as_bytes();

            if !(l_b[0] as char).is_digit(10) {
                continue;
            }

            let mut iter = l.split_whitespace();

            let r = iter.next().unwrap().parse::<u8>().unwrap();
            let g = iter.next().unwrap().parse::<u8>().unwrap();
            let b = iter.next().unwrap().parse::<u8>().unwrap();

            values.push(RGB::new(r, g, b));
        }

        self.palettes.insert(name.clone(), values);
        self.palettes_list.push(name.clone());
    }

    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }
}


#[derive(Clone, Debug)]
pub struct Palette {
    pub colors: HashMap<u32, RGB>,
    rcolors: HashMap<u32, u32>,
    cached_colors: [u32; 16],
}

impl Palette {
    pub fn new() -> Palette {
        Palette {
            colors: HashMap::new(),
            rcolors: HashMap::new(),
            cached_colors: [0; 16],
        }
    }

    pub fn get_rgb(&mut self, value: u32) -> RGB {
        if value < 16 {
            let v = self.cached_colors[value as usize];

            let r = ((v & 0xff0000) >> 16) as u8;
            let g = ((v & 0x00ff00) >> 8) as u8;
            let b = (v & 0x0000ff) as u8;

            return RGB::new(r, g, b);
        }

        match self.colors.get(&value) {
            Some(rgb_value) => RGB::new(rgb_value.r, rgb_value.g, rgb_value.b),
            _ => RGB::new(0, 0, 0),
        }
    }

    pub fn reset(&mut self) {
        self.colors.clear();
    }

    pub fn _set_color(&mut self, color: u32, r: u8, g: u8, b: u8) {
        let u32_color = (r as u32) << 16 | (g as u32) << 8 | (b as u32);

        self.colors.insert(color, RGB::new(r, g, b));
        self.rcolors.insert(u32_color, color);

        if color < 16 {
            self.cached_colors[color as usize] = u32_color;
        }
    }

    pub fn set_color(&mut self, color: u32, r: u8, g: u8, b: u8) {
        if color >= 16 {
            self._set_color(color, r, g, b);
        }
    }

    pub fn set_colors(&mut self, colors: HashMap<u32, RGB>) {
        for (color, values) in colors {
            self._set_color(color, values.r, values.g, values.b);
        }
    }

    pub fn add_color(&mut self, r: u8, g: u8, b: u8) -> u8 {
        let value = self.rcolors.len() as u8;

        let v = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        match self.rcolors.get(&v) {
            Some(color) => return *color as u8,
            _ => (),
        }

        info!("[GFX][Palette] add color {:?} r:{:?} g:{:?}: b:{:?}", value, r, g, b);

        self.set_color(value as u32, r, g, b);

        value
    }

    pub fn get_color(&mut self, color: u32) -> u32 {
        match self.colors.get(&color) {
            Some(rgb_value) => {
                (rgb_value.r as u32) << 16 | (rgb_value.g as u32) << 8 | (rgb_value.b as u32)
            }
            _ => 0,
        }
    }

    pub fn get_color_rgb(&mut self, r: u8, g: u8, b: u8) -> u32 {
        let u32_color = (r as u32) << 16 | (g as u32) << 8 | (b as u32);

        match self.rcolors.get(&u32_color) {
            Some(value) => {
                *value
            }
            _ => 0,
        }
    }
}


#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r: r, g: g, b: b, a: 0xFF }
    }

    pub fn new_hexa(v: u32) -> RGB {
        RGB {
            r: ((v & 0xff0000) >> 16) as u8,
            g: ((v & 0x00ff00) >> 8) as u8,
            b: (v & 0x0000ff) as u8,
            a: 0xFF,
        }
    }

    pub fn into_pixel_data(&self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl From<[u8; 4]> for RGB {
    fn from(color: [u8; 4]) -> Self {
        Self {
            r: color[0],
            g: color[1],
            b: color[2],
            a: color[3],
        }
    }
}