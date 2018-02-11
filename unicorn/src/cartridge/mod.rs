use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io;
use std::io::BufRead;
use std::io::Read;

use std::convert;
use std::fmt;
use std::io::Write;

use std::result::Result;
use std::collections::HashMap;
use std::u32;
use std::str;

use serde_json;

use regex::Regex;

use unicorn;
use unicorn::RGB;

use gfx::Sprite;

/* CART FORMAT

RANDOM COMMENT
version XX
__python__ __javascript__ __lua__

__palette__

__gfx__ 400x240 -> 1500

XXXXXXXX
XXXXXXXX
XXXXXXXX
XXXXXXXX    => X: 0..1024
XXXXXXXX
XXXXXXXX
XXXXXXXX
XXXXXXXX


__gff__

__map__ 400*60 -> 24.000

__sfx__

__music__

*/


fn read_u8(v: &mut Vec<u8>) -> usize {
    let u: Vec<_> = v.drain(0..2).collect();

    ((u[1] as usize) << 4) | u[0] as usize
}

pub struct CartridgeJavascript {
    pub data: String,
}

impl CartridgeJavascript {
    pub fn new(lines: &[String]) -> CartridgeJavascript {
        let mut data = "".to_string();

        for line in lines {
            data = data + line;
            data.push('\n');
        }

        CartridgeJavascript { data: data }
    }
}


pub struct CartridgePython {
    pub data: String,
}

impl CartridgePython {
    pub fn new(lines: &[String]) -> CartridgePython {
        let mut data = "".to_string();

        for line in lines {
            data = data + line;
            data.push('\n');
        }

        CartridgePython { data: data }
    }
}

pub struct CartridgeLua {
    pub data: String,
}

impl CartridgeLua {
    pub fn new(lines: &[String]) -> CartridgeLua {
        let mut data = "".to_string();

        for line in lines {
            data = data + line;
            data.push('\n');
        }

        CartridgeLua { data: data }
    }
}


pub struct CartridgePalette {
    pub colors: HashMap<u32, RGB>
}

impl CartridgePalette {
    pub fn empty() -> CartridgePalette {
        CartridgePalette { colors: HashMap::new() }
    }

    pub fn new(lines: &[String]) -> CartridgePalette {
        let mut colors = HashMap::new();

        let data = "".to_string();

        for line in lines {
            let split_line = line.split(" ");
            let vec: Vec<&str> = split_line.collect();

            if vec.len() == 4 {
                let color = vec[0].parse::<u32>().unwrap();
                let r = vec[1].parse::<u8>().unwrap();
                let g = vec[2].parse::<u8>().unwrap();
                let b = vec[3].parse::<u8>().unwrap();

                colors.insert(color, RGB::new(r, g, b));
            }
        }

        CartridgePalette { colors: colors }
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for (color, rgb) in &self.colors {
            data.push_str(&format!("{:?} {:?} {:?} {:?}\n", color, rgb.r, rgb.g, rgb.b));
        }

        data
    }

    pub fn set_colors(&mut self, colors: HashMap<u32, RGB>) {
        self.colors.clear();
        self.colors.extend(colors);
    }

}

pub struct CartridgeCode {
    pub lines: Vec<String>,
    pub data: Vec<u8>,
    pub version: u8,
    pub code_type: String,
    pub filename: String,
}

impl CartridgeCode {
    pub fn empty() -> CartridgeCode {
        info!("[CARTRIDGE] CartridgeCode");

        CartridgeCode {
            lines: Vec::new(),
            data: Vec::new(),
            version: 0,
            code_type: "".to_string(),
            filename: "".to_string(),
        }
    }

    pub fn new(code_type: String, lines: &[String]) -> CartridgeCode {
        info!("[CARTRIDGE] CartridgeCode");

        CartridgeCode {
            lines: lines.to_vec(),
            data: Vec::new(),
            version: 0,
            code_type: code_type,
            filename: "".to_string(),
        }
    }

    pub fn set_filename(&mut self, filename: &str) {
        self.filename = filename.to_string();
    }

    pub fn get_name(&mut self) -> String {
        self.code_type.clone()
    }

    pub fn reload(&mut self) {
        let f1 = File::open(self.filename.as_str());
        match f1 {
            Ok(f1) => {
                let buf_reader = BufReader::new(f1);

                let mut code_section = Vec::new();

                for line in buf_reader.lines() {
                    let l = line.unwrap();
                    code_section.push(l);
                }

                self.lines = code_section;
            }
            Err(e) => {
                warn!("[CARTRIDGE] Error to reload the file {:?} -> {:?}", self.filename, e);
            }
        }

    }

    pub fn get_data(&mut self) -> String {
        if self.code_type == "lua" {
            let cart = CartridgeLua::new(&self.lines);
            return cart.data;
        }

        if self.code_type == "python" {
            let cart = CartridgePython::new(&self.lines);
            return cart.data;
        }

        if self.code_type == "javascript" {
            let cart = CartridgeJavascript::new(&self.lines);
            return cart.data;
        }

        "".to_string()
    }

    pub fn set_data(&mut self, lines: Vec<String>) {
        self.lines.clear();
        for mut line in lines {
            if line.len() > 0 {
                let len = line.len();
                line.truncate(len - 1);
            }
            self.lines.push(line);
        }
    }
}

pub struct CartridgeGFX {
    pub sprites: Vec<Sprite>,
}

impl fmt::Debug for CartridgeGFX {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comma_separated = String::new();
        write!(f,
               "CartridgeGFX {{ sprites: {} {} }}",
               self.sprites.len(),
               comma_separated)
    }
}


impl CartridgeGFX {
    pub fn empty() -> CartridgeGFX {
        // Fill with no sprites
        CartridgeGFX { sprites:  Vec::new() }
    }

    pub fn new(lines: &[String]) -> CartridgeGFX {
        info!("[CARTRIDGE][CartridgeGFX]");

        let mut sprites: Vec<Sprite> = Vec::new();

        if !lines.is_empty() {
            let mut nbsprites = 0;
            let mut v = Vec::new();

            for line in lines {
                if line.len() > 1200 {
                    continue;
                }

                let mut i = 0;
                while i < 1200 {
                    let value = u32::from_str_radix(&line[i..i + 3], 16).unwrap();
                    //info!("VAL {:?} {:?}", v, z);

                    v.push(value);
                    
                    i += 3;
                    nbsprites += 1;
                }
            }

            info!("[CARTRIDGE][CartridgeGFX] {:?}", v.len());

            let mut g_off = 0;

            // Fill all sprites
            for idx in 0..1500 {
                let mut data: [u32; 8 * 8] = [0; 8 * 8];

                let mut idx_vec = 0;

                if idx > 0 {
                    if idx % 50 == 0 {
                        g_off = idx * 8 * 8;
                    } else {
                        g_off += 8;
                    }
                }

                for y in 0..8 {
                    for x in 0..8 {
                        let offset = g_off + y * 400 + x;

                        data[idx_vec] = v[offset];
                        idx_vec += 1;
                    }
                }

                sprites.push(Sprite::new(data));
            }

            info!("[CARTRIDGE][CartridgeGFX] {:?}", sprites.len());
        }

        CartridgeGFX { sprites: sprites }
    }

    pub fn set_sprites(&mut self, sprites: Vec<Sprite>) {
        self.sprites = sprites;
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        let mut idx_sprites = 0;
        let mut line;

        for y in 0..240 {
            line = y % 8;

            if y > 0 && (y % 8) == 0 {
                idx_sprites += 50;
            }

            for idx in idx_sprites..idx_sprites + 50 {
                let mut gfx_sprites = self.sprites[idx].clone();

                data.push_str(&gfx_sprites.get_line(line));
            }

            data.push('\n');
        }

        data
    }
}

pub struct CartridgeGFF {
    pub flags: Vec<u8>,
}

impl CartridgeGFF {
    pub fn empty() -> CartridgeGFF {
        CartridgeGFF { flags: Vec::new() }
    }

    pub fn new(lines: &[String]) -> CartridgeGFF {
        info!("[CARTRIDGE] CartridgeGFF");

        let mut v = Vec::new();

        for line in lines {
            for c in line.as_bytes() {
                v.push((*c as char).to_digit(16).unwrap() as u8);
            }
        }

        let mut v_order = Vec::new();
        let mut idx = 0;
        while idx < v.len() {
            v_order.push(v[idx + 1]);
            v_order.push(v[idx]);

            idx += 2;
        }

        CartridgeGFF::new_from_bytes(&v_order)
    }

    pub fn new_from_bytes(v: &[u8]) -> CartridgeGFF {
        let mut flags: Vec<u8> = Vec::new();

        let mut v_copy = v.to_vec();

        let len_v = v_copy.len();
        let mut idx: usize = 0;

        while idx < len_v {
            let flag = read_u8(&mut v_copy);
            flags.push(flag as u8);
            idx += 2;
        }

        CartridgeGFF { flags: flags.clone() }
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for (i, flag) in self.flags.iter().enumerate() {
            data.push_str(&format!("{:x}{:x}", (flag & 0xf0) >> 4, flag & 0x0f));

            if (i + 1) % 128 == 0 {
                data.push('\n');
            }
        }

        data
    }

    pub fn set_flags(&mut self, sprites: Vec<Sprite>) {
        if self.flags.len() != sprites.len() {
            warn!("Wrong number of flags {:?}, {:?}", self.flags.len(), sprites.len());
            return;
        }

        let mut idx = 0;
        for s in &sprites {
            if idx <= self.flags.len() {
                self.flags[idx] = s.flags;
            } else {
                self.flags.push(s.flags);
            }

            idx += 1;
        }
    }
}

pub struct CartridgeMusic {}

impl CartridgeMusic {
    pub fn new(_lines: &[String]) -> CartridgeMusic {
        info!("[CARTRIDGE] CartridgeMusic");
        CartridgeMusic {}
    }

    pub fn new_from_bytes(_v: &[u8]) -> CartridgeMusic {
        CartridgeMusic {}
    }

    pub fn empty() -> CartridgeMusic {
        CartridgeMusic {}
    }
}

pub struct CartridgeMap {
    pub map: Vec<u32>,
}

impl CartridgeMap {
    pub fn empty() -> CartridgeMap {
        CartridgeMap { map: Vec::new() }
    }

    pub fn new(lines: &[String]) -> CartridgeMap {
        info!("[CARTRIDGE] CartridgeMap");

        let mut map = Vec::new();
        let mut y = 0;

        for line in lines {
            let mut i = 0;

            while i < unicorn::MAP_WIDTH*3 {
                let idx_sprite = u32::from_str_radix(&line[i..i + 3], 16).unwrap();
                //info!("VAL {:?} {:?}", v, z);

                map.push(idx_sprite);
                
                i += 3;
            }

            y += 1;

            if y == 60 {
                break;
            }
        }

        CartridgeMap { map: map }
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for y in 0..unicorn::MAP_HEIGHT {
            for x in 0..unicorn::MAP_WIDTH {
                let idx_sprite = *self.map.get(x * unicorn::MAP_WIDTH + y).unwrap_or(&0);
                data.push_str(&format!("{:03x}", idx_sprite));
            }
            data.push('\n');
        }

        data
    }

    pub fn set_map(&mut self, map: Vec<u32>) {
        self.map = map;
    }
}

pub enum CartridgeFormat {
    UnicornSplittedFormat = 0,
    UnicornFormat = 1,
}

pub struct Cartridge {
    pub filename: String,
    pub data_filename: String,
    pub header: String,
    pub version: String,
    pub gfx: CartridgeGFX,
    pub map: CartridgeMap,
    pub gff: CartridgeGFF,
    pub code: CartridgeCode,
    pub palette: CartridgePalette,
    pub music: CartridgeMusic,
    pub format: CartridgeFormat,
}

pub static SECTION_DELIM_RE: &'static str = r"^__(\w+)__$";

#[derive(Debug)]
pub enum Error {
    Err(String),
    IOError(io::Error),
}

impl convert::From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IOError(e)
    }
}

fn read_from_uniformat<R: io::BufRead>(filename: &str, buf: &mut R) -> Result<Cartridge, Error> {
    let mut header = String::new();
    try!(buf.read_line(&mut header));

    let mut version = String::new();
    try!(buf.read_line(&mut version));

    let re_delim_section = Regex::new(SECTION_DELIM_RE).unwrap();

    let mut sections: HashMap<String, Vec<(String)>> = HashMap::new();

    let mut section_name = "".to_string();

    let mut new_section;

    for line in buf.lines() {
        let l = line.unwrap();
        if re_delim_section.is_match(l.as_str()) {
            debug!("NEW SECTION {:?}", l);
            section_name = l.clone();

            let vec_section = Vec::new();
            sections.insert(section_name.clone(), vec_section);
            new_section = false;
        } else {
            new_section = true;
        }

        if new_section {
            match sections.get_mut(&section_name) {
                Some(vec_section2) => vec_section2.push(l),
                _ => debug!("Impossible to find section {:?}", section_name),
            }
        }
    }

    for (section_name, section) in &sections {
        debug!("{}: \"{}\"", section_name, section.len());
    }

    let cartridge_gfx;
    let mut cartridge_code;
    let cartridge_palette;
    let cartridge_map;
    let cartridge_gff;
    let cartridge_music;


    if sections.contains_key("__lua__") {
        cartridge_code = CartridgeCode::new("lua".to_string(),
                                            sections.get_mut("__lua__").unwrap());
    } else if sections.contains_key("__python__") {
        cartridge_code = CartridgeCode::new("python".to_string(),
                                            sections.get_mut("__python__").unwrap());
    } else if sections.contains_key("__javascript__") {
        cartridge_code = CartridgeCode::new("javascript".to_string(),
                                            sections.get_mut("__javascript__").unwrap());
    } else {
        return Err(Error::Err("NO CODE DATA".to_string()));
    }

    match sections.get_mut("__palette__") {
        Some(vec_section) => cartridge_palette = CartridgePalette::new(vec_section),
        _ => cartridge_palette = CartridgePalette::empty(),
    }

    match sections.get_mut("__gfx__") {
        Some(vec_section) => cartridge_gfx = CartridgeGFX::new(vec_section),
        _ => cartridge_gfx = CartridgeGFX::empty(),
    }

    match sections.get_mut("__map__") {
        Some(vec_section) => cartridge_map = CartridgeMap::new(vec_section),
        _ => cartridge_map = CartridgeMap::empty(),
    }

    match sections.get_mut("__gff__") {
        Some(vec_section) => cartridge_gff = CartridgeGFF::new(vec_section),
        _ => cartridge_gff = CartridgeGFF::empty(),
    }

    match sections.get_mut("__music__") {
        Some(vec_section) => cartridge_music = CartridgeMusic::new(vec_section),
        _ => cartridge_music = CartridgeMusic::empty(),
    }


    cartridge_code.set_filename(filename);

    Ok(Cartridge {
           filename: filename.to_string(),
           data_filename: "".to_string(),
           header: header.clone(),
           version: version.clone(),
           gfx: cartridge_gfx,
           code: cartridge_code,
           palette: cartridge_palette,
           map: cartridge_map,
           gff: cartridge_gff,
           music: cartridge_music,
           format: CartridgeFormat::UnicornFormat,
       })
}

#[derive(Serialize, Deserialize)]
struct UnicornSplittedFormat {
    code: String,
    data: String,
}


impl Cartridge {
    pub fn empty() -> Cartridge {
        Cartridge {
            filename: "".to_string(),
            data_filename: "".to_string(),
            header: "".to_string(),
            version: "".to_string(),
            gfx: CartridgeGFX::empty(),
            map: CartridgeMap::empty(),
            gff: CartridgeGFF::empty(),
            code: CartridgeCode::empty(),
            palette: CartridgePalette::empty(),
            music: CartridgeMusic::empty(),
            format: CartridgeFormat::UnicornFormat,
        }
    }

    pub fn from_uni_raw(filename: &str, data: Vec<u8>) -> Result<Cartridge, Error> {
        let mut buf_reader = Cursor::new(data);
        let cartridge = try!(read_from_uniformat(filename, &mut buf_reader));
        Ok(cartridge)
    }

    pub fn from_unicorn_file(filename: &str) -> Result<Cartridge, Error> {
        let f = try!(File::open(filename));
        let mut buf_reader = BufReader::new(f);
        let cartridge = try!(read_from_uniformat(filename, &mut buf_reader));
        Ok(cartridge)
    }

    pub fn from_unicorn_splitted_file(filename: &str) -> Result<Cartridge, Error> {
        let mut f = try!(File::open(filename));

        let mut data = String::new();
        f.read_to_string(&mut data).unwrap();

        let json: UnicornSplittedFormat = serde_json::from_str(&data).unwrap();

        let code_file = json.code.as_str();

        let f1 = try!(File::open(code_file));
        let buf_reader = BufReader::new(f1);

        let mut code_section = Vec::new();

        for line in buf_reader.lines() {
            let l = line.unwrap();
            code_section.push(l);
        }

        let data_file = json.data.as_str();
        let f2 = try!(File::open(data_file));
        let buf_reader = BufReader::new(f2);

        let re_delim_section = Regex::new(SECTION_DELIM_RE).unwrap();

        let mut sections: HashMap<String, Vec<(String)>> = HashMap::new();

        let mut section_name = "".to_string();

        let mut new_section;

        for line in buf_reader.lines() {
            let l = line.unwrap();
            if re_delim_section.is_match(l.as_str()) {
                debug!("NEW SECTION {:?}", l);
                section_name = l.clone();

                let vec_section = Vec::new();
                sections.insert(section_name.clone(), vec_section);
                new_section = false;
            } else {
                new_section = true;
            }

            if new_section {
                match sections.get_mut(&section_name) {
                    Some(vec_section2) => vec_section2.push(l),
                    _ => debug!("Impossible to find section {:?}", section_name),
                }
            }
        }

        for (section_name, section) in &sections {
            debug!("{}: \"{}\"", section_name, section.len());
        }

        let cartridge_gfx;
        let cartridge_gff;
        let mut cartridge_code;
        let cartridge_palette;
        let cartridge_map;
        let cartridge_music;

        if code_file.contains(".py") {
            cartridge_code = CartridgeCode::new("python".to_string(), &code_section);
        } else if code_file.contains(".js") {
            cartridge_code = CartridgeCode::new("javascript".to_string(), &code_section);
        } else if code_file.contains(".lua") {
            cartridge_code = CartridgeCode::new("lua".to_string(), &code_section);
        } else {
            panic!("Unknown file to load the code {:?}", code_file);
        }

        cartridge_code.set_filename(code_file);

        match sections.get_mut("__palette__") {
            Some(vec_section) => cartridge_palette = CartridgePalette::new(vec_section),
            _ => cartridge_palette = CartridgePalette::empty(),
        }

        match sections.get_mut("__gfx__") {
            Some(vec_section) => cartridge_gfx = CartridgeGFX::new(vec_section),
            _ => cartridge_gfx = CartridgeGFX::empty(),
        }

        match sections.get_mut("__map__") {
            Some(vec_section) => cartridge_map = CartridgeMap::new(vec_section),
            _ => cartridge_map = CartridgeMap::empty(),
        }

        match sections.get_mut("__gff__") {
            Some(vec_section) => cartridge_gff = CartridgeGFF::new(vec_section),
            _ => cartridge_gff = CartridgeGFF::empty(),
        }

        match sections.get_mut("__music__") {
            Some(vec_section) => cartridge_music = CartridgeMusic::new(vec_section),
            _ => cartridge_music = CartridgeMusic::empty(),
        }


        Ok(Cartridge {
               filename: filename.to_string(),
               data_filename: data_file.to_string(),
               header: "".to_string(),
               version: "".to_string(),
               gfx: cartridge_gfx,
               code: cartridge_code,
               palette: cartridge_palette,
               map: cartridge_map,
               gff: cartridge_gff,
               music: cartridge_music,
               format: CartridgeFormat::UnicornSplittedFormat,
           })
    }

    pub fn from_dunicorn_file(filename: &str) -> Result<Cartridge, Error> {
        let code_section = Vec::new();

        let f2 = try!(File::open(filename));
        let buf_reader = BufReader::new(f2);

        let re_delim_section = Regex::new(SECTION_DELIM_RE).unwrap();

        let mut sections: HashMap<String, Vec<(String)>> = HashMap::new();

        let mut section_name = "".to_string();

        let mut new_section;

        for line in buf_reader.lines() {
            let l = line.unwrap();
            if re_delim_section.is_match(l.as_str()) {
                debug!("NEW SECTION {:?}", l);
                section_name = l.clone();

                let vec_section = Vec::new();
                sections.insert(section_name.clone(), vec_section);
                new_section = false;
            } else {
                new_section = true;
            }

            if new_section {
                match sections.get_mut(&section_name) {
                    Some(vec_section2) => vec_section2.push(l),
                    _ => debug!("Impossible to find section {:?}", section_name),
                }
            }
        }

        for (section_name, section) in &sections {
            debug!("{}: \"{}\"", section_name, section.len());
        }

        let cartridge_gfx;
        let cartridge_gff;
        let cartridge_palette;
        let mut cartridge_code;
        let cartridge_map;
        let cartridge_music;

        cartridge_code = CartridgeCode::new("javascript".to_string(), &code_section);
        cartridge_code.set_filename("empty.js");

        match sections.get_mut("__palette__") {
            Some(vec_section) => cartridge_palette = CartridgePalette::new(vec_section),
            _ => cartridge_palette = CartridgePalette::empty(),
        }


        match sections.get_mut("__gfx__") {
            Some(vec_section) => cartridge_gfx = CartridgeGFX::new(vec_section),
            _ => cartridge_gfx = CartridgeGFX::empty(),
        }

        match sections.get_mut("__map__") {
            Some(vec_section) => cartridge_map = CartridgeMap::new(vec_section),
            _ => cartridge_map = CartridgeMap::empty(),
        }

        match sections.get_mut("__gff__") {
            Some(vec_section) => cartridge_gff = CartridgeGFF::new(vec_section),
            _ => cartridge_gff = CartridgeGFF::empty(),
        }

        match sections.get_mut("__music__") {
            Some(vec_section) => cartridge_music = CartridgeMusic::new(vec_section),
            _ => cartridge_music = CartridgeMusic::empty(),
        }


        Ok(Cartridge {
               filename: "empty.py".to_string(),
               data_filename: filename.to_string(),
               header: "".to_string(),
               version: "".to_string(),
               gfx: cartridge_gfx,
               code: cartridge_code,
               palette: cartridge_palette,
               map: cartridge_map,
               gff: cartridge_gff,
               music: cartridge_music,
               format: CartridgeFormat::UnicornSplittedFormat,
           })
    }

    pub fn save_in_unicorn(&mut self, filename: &str, version: &str) {
        info!("Save the modified cartridge in Unicorn format {:?}", filename);

        let mut f = File::create(filename).unwrap();

        f.write_all(b"Saved by unicorn\n").unwrap();
        f.write_all(format!("Version {:?}\n", version).as_bytes())
            .unwrap();

        f.write_all(format!("__{:}__\n", self.code.code_type).as_bytes())
            .unwrap();
        f.write_all(self.code.get_data().clone().as_bytes())
            .unwrap();

        f.write_all(b"__palette__\n").unwrap();
        f.write_all(self.palette.get_data().clone().as_bytes()).unwrap();

        f.write_all(b"__gfx__\n").unwrap();
        f.write_all(self.gfx.get_data().clone().as_bytes()).unwrap();

        f.write_all(b"__gff__\n").unwrap();
        f.write_all(self.gff.get_data().clone().as_bytes()).unwrap();

        f.write_all(b"__map__\n").unwrap();
        f.write_all(self.map.get_data().clone().as_bytes()).unwrap();

        f.write_all(b"__sfx__\n").unwrap();

        f.write_all(b"__music__\n").unwrap();
    }

    pub fn save_in_unicorn_splitted(&mut self) {
        info!("Save the date of the Unicorn Splitted file in {:?}", self.data_filename);

        match self.format {
            CartridgeFormat::UnicornFormat => {
                let mut f = File::create(self.data_filename.clone()).unwrap();

                f.write_all(b"__gfx__\n").unwrap();
                f.write_all(self.gfx.get_data().clone().as_bytes()).unwrap();

                f.write_all(b"__gff__\n").unwrap();
                f.write_all(self.gff.get_data().clone().as_bytes()).unwrap();

                f.write_all(b"__map__\n").unwrap();
                f.write_all(self.map.get_data().clone().as_bytes()).unwrap();

                f.write_all(b"__sfx__\n").unwrap();

                f.write_all(b"__music__\n").unwrap();
            }
            _ => (),
        }
    }


    pub fn dump(&mut self, filename: &str) {
        info!("Dump the code in {:?}", filename);

        let mut f = File::create(filename).unwrap();
        f.write_all(self.code.get_data().clone().as_bytes())
            .unwrap();
    }
}

impl fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Cartridge {{ filename: {}, gfx: {:?} }}",
               self.filename,
               self.gfx)
    }
}
