mod code;
mod gff;
mod gfx;
mod sfx;
mod map;
mod music;
mod palette;
mod utils;

use log::{debug, info};

use std::fs::File;
use std::io::BufReader;
use std::io::Cursor;
use std::io;
use std::io::BufRead;
use std::io::Read;

use std::convert;
use std::io::Write;

use std::result::Result;
use std::collections::HashMap;
use std::u32;
use std::str;

use png;
use regex::Regex;

pub use self::code::CartridgeCode;
pub use self::gff::CartridgeGFF;
pub use self::gfx::CartridgeGFX;
pub use self::sfx::CartridgeSFX;
pub use self::palette::CartridgePalette;
pub use self::map::CartridgeMap;
pub use self::music::CartridgeMusic;

/* [CART FORMAT]

RANDOM COMMENT
version XX
{__python__} | {__rpython__} | {__lua__} | {__rhai__}

__palette__

__gfx__ 128x128 -> 16384

XXXXXXXX
XXXXXXXX
XXXXXXXX
XXXXXXXX    => X: 0..128
XXXXXXXX
XXXXXXXX
XXXXXXXX
XXXXXXXX


__gff__

__map__ 128*32

__sfx__

__music__

*/

#[derive(Debug)]
pub enum CartridgeFormat {
    UnicornFormat = 0,
    Pico8PNGFormat = 1,
    Pico8P8Format = 2,
}

#[derive(Debug)]
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
    pub sfx: CartridgeSFX,
    pub format: CartridgeFormat,
}

pub static SECTION_DELIM_RE: &'static str = r"^__(\w+)__$";
pub static SUB_SECTION_DELIM_RE: &'static str = r"^___(\w+)___$";

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

/* Unicorn format cardrgide */
fn read_from_uniformat<R: io::BufRead>(filename: &str, buf: &mut R) -> Result<Cartridge, Error> {
    debug!("[CARTRIDGE] read_from_uniformat");

    let mut header = String::new();
    buf.read_line(&mut header)?;

    let mut version = String::new();
    buf.read_line(&mut version)?;

    let re_delim_section = Regex::new(SECTION_DELIM_RE).unwrap();

    let mut sections: HashMap<String, Vec<String>> = HashMap::new();

    let mut section_name = "".to_string();

    let mut new_section;

    for line in buf.lines() {
        let l = line.unwrap();
        if re_delim_section.is_match(l.as_str()) {
            debug!("[CARTRIDGE] [Cartridge] NEW SECTION {:?}", l);
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
                _ => debug!("[CARTRIDGE] [Cartridge] Impossible to find section {:?}", section_name),
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
    let cartridge_sfx;

    if sections.contains_key("__lua__") {
        cartridge_code = CartridgeCode::new("lua".to_string(),
                                            sections.get_mut("__lua__").unwrap());
    } else if sections.contains_key("__python__") {
        cartridge_code = CartridgeCode::new("python".to_string(),
                                            sections.get_mut("__python__").unwrap());
    } else if sections.contains_key("__rhai__") {
        cartridge_code = CartridgeCode::new("rhai".to_string(),
                                            sections.get_mut("__rhai__").unwrap());
    } else if sections.contains_key("__code__") {
        if sections.contains_key("___rhai___") {
            
        }
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
        Some(vec_section) => cartridge_map = CartridgeMap::new(vec_section, 128, 128),
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

    match sections.get_mut("__sfx__") {
        Some(vec_section) => cartridge_sfx = CartridgeSFX::new(vec_section),
        _ => cartridge_sfx = CartridgeSFX::empty(),
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
           sfx: cartridge_sfx,
           format: CartridgeFormat::UnicornFormat,
       })
}

pub fn from_dunicorn_file_raw<R: io::BufRead>(buf_reader: &mut R) -> Result<Cartridge, Error> {
    let code_section = Vec::new();

    let re_delim_section = Regex::new(SECTION_DELIM_RE).unwrap();

    let mut sections: HashMap<String, Vec<String>> = HashMap::new();

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
    let cartridge_sfx;

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
        Some(vec_section) => cartridge_map = CartridgeMap::new(vec_section, 128, 32),
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

    match sections.get_mut("__sfx__") {
        Some(vec_section) => cartridge_sfx = CartridgeSFX::new(vec_section),
        _ => cartridge_sfx = CartridgeSFX::empty(),
    }

    Ok(Cartridge {
           filename: "empty".to_string(),
           data_filename: "empty.duc".to_string(),
           header: "".to_string(),
           version: "".to_string(),
           gfx: cartridge_gfx,
           code: cartridge_code,
           palette: cartridge_palette,
           map: cartridge_map,
           gff: cartridge_gff,
           music: cartridge_music,
           sfx: cartridge_sfx,
           format: CartridgeFormat::UnicornFormat,
       })
}

/* Pico8 PNG format cartridge */
fn read_from_pngformat<R: io::BufRead>(filename: &str, buf: &mut R) -> Result<Cartridge, Error> {
    info!("[CARTRIDGE] [read_from_pngformat] Starting to parse the Pico8 PNG");

    let decoder = png::Decoder::new(buf);
    let mut reader = decoder.read_info().unwrap();
    info!("{:?}", reader.output_buffer_size());

    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();

    let mut picodata = Vec::new();

    let mut row = 0;
    while row < buf.len() {
        for col_i in 0..info.width {
            let g_idx: u32 = row as u32;

            let mut r = buf[(g_idx + col_i * 4) as usize] as u8;
            let mut g = buf[(g_idx + col_i * 4 + 1) as usize] as u8;
            let mut b = buf[(g_idx + col_i * 4 + 2) as usize] as u8;
            let mut a = buf[(g_idx + col_i * 4 + 3) as usize] as u8;

            r &= 3;
            g &= 3;
            b &= 3;
            a &= 3;

            let v = b | (g << 2) | (r << 4) | (a << 6);
            let lo = v & 0x0f;
            let hi = v >> 4;

            picodata.push(lo);
            picodata.push(hi);
        }

        row += 640;
    }

    let mut gfx_data: Vec<String> = Vec::new();
    for i in 0..0x2000 * 2 {
        gfx_data.push(picodata[i].to_string());
    }

    let mut map_data = Vec::new();
    for i in 0x2000 * 2..0x3000 * 2 {
        map_data.push(picodata[i]);
    }

    let mut gff_data = Vec::new();
    for i in 0x3000 * 2..0x3100 * 2 {
        gff_data.push(picodata[i]);
    }

    let _version = picodata[0x8000 * 2];

    let mut code_data = Vec::new();
    for i in 0x4300 * 2..0x8000 * 2 {
        code_data.push(picodata[i]);
    }

    let mut music_data = Vec::new();
    for i in 0x3100 * 2..0x3200 * 2 {
        music_data.push(picodata[i]);
    }

    let cartridge_gfx = CartridgeGFX::new(&gfx_data);
    //let cartridge_code = CartridgeCode::new("lua".to_string(), &mut code_data, version);
    //let cartridge_map = CartridgeMap::new(&map_data);
    //let cartridge_gff = CartridgeGFF::new(&gff_data);
    //let cartridge_music = CartridgeMusic::new(&music_data);

    Ok(Cartridge {
           filename: filename.to_string(),
           data_filename: "".to_string(),
           header: "".to_string(),
           version: "".to_string(),
           gfx: cartridge_gfx,
           code: CartridgeCode::empty(),
           map: CartridgeMap::empty(),
           gff: CartridgeGFF::empty(),
           palette: CartridgePalette::empty(),
           music: CartridgeMusic::empty(),
           sfx: CartridgeSFX::empty(),
           format: CartridgeFormat::Pico8PNGFormat,
       })
}

/* Pico8 P8 format cartridge */
fn read_from_p8format<R: io::BufRead>(filename: &str, buf: &mut R) -> Result<Cartridge, Error> {
    info!("[CARTRIDGE] [read_from_p8format] Starting to parse the Pico8 P8");

    let mut header = String::new();
    buf.read_line(&mut header)?;

    let mut version = String::new();
    buf.read_line(&mut version)?;

    let re_delim_section = Regex::new(SECTION_DELIM_RE).unwrap();

    let mut sections: HashMap<String, Vec<String>> = HashMap::new();

    let mut section_name = "".to_string();

    let mut new_section;

    for line in buf.lines() {
        let l = line.unwrap();
        if re_delim_section.is_match(l.as_str()) {
            debug!("[CARTRIDGE] [Cartridge] NEW SECTION {:?}", l);
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
    let cartridge_code;
    let cartridge_map;
    let cartridge_gff;
    let cartridge_music;
    let cartridge_sfx;


    if sections.contains_key("__lua__") {
        cartridge_code = CartridgeCode::new("lua".to_string(),
                                            sections.get_mut("__lua__").unwrap());
    } else {
        return Err(Error::Err("NO CODE DATA".to_string()));
    }

    match sections.get_mut("__gfx__") {
        Some(vec_section) => cartridge_gfx = CartridgeGFX::new(vec_section),
        _ => cartridge_gfx = CartridgeGFX::empty(),
    }

    match sections.get_mut("__map__") {
        Some(vec_section) => cartridge_map = CartridgeMap::new(vec_section, 128, 128),
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

    match sections.get_mut("__sfx__") {
        Some(vec_section) => cartridge_sfx = CartridgeSFX::new(vec_section),
        _ => cartridge_sfx = CartridgeSFX::empty(),
    }


    Ok(Cartridge {
           filename: filename.to_string(),
           data_filename: "".to_string(),
           header: header.clone(),
           version: version.clone(),
           gfx: cartridge_gfx,
           code: cartridge_code,
           map: cartridge_map,
           gff: cartridge_gff,
           palette: CartridgePalette::empty(),
           music: cartridge_music,
           sfx: cartridge_sfx,
           format: CartridgeFormat::Pico8P8Format,
       })
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
            sfx: CartridgeSFX::empty(),
            format: CartridgeFormat::UnicornFormat,
        }
    }

    pub fn from_uni_raw(filename: &str, data: Vec<u8>) -> Result<Cartridge, Error> {
        let mut buf_reader = Cursor::new(data);
        let cartridge = read_from_uniformat(filename, &mut buf_reader)?;
        Ok(cartridge)
    }

    pub fn from_unicorn_file(filename: &str) -> Result<Cartridge, Error> {
        let f = File::open(filename)?;
        let mut buf_reader = BufReader::new(f);
        let cartridge = read_from_uniformat(filename, &mut buf_reader)?;
        Ok(cartridge)
    }

    pub fn from_dunicorn_file(filename: &str) -> Result<Cartridge, Error> {
        let mut f = File::open(filename)?;

        let mut data = String::new();
        f.read_to_string(&mut data).unwrap();

        let mut buf_reader = Cursor::new(data);

        from_dunicorn_file_raw(&mut buf_reader)
    }

    pub fn from_dunicorn_string(data: Vec<u8>) -> Result<Cartridge, Error> {
        let mut buf_reader = Cursor::new(data);

        from_dunicorn_file_raw(&mut buf_reader)
    }

    pub fn from_png_file(filename: &str) -> Result<Cartridge, Error> {
        let mut f = File::open(filename)?;

        let mut data = Vec::new();
        f.read_to_end(&mut data).unwrap();

        let mut buf_reader = Cursor::new(data);

        read_from_pngformat(filename, &mut buf_reader)
    }

    pub fn from_p8_file(filename: &str) -> Result<Cartridge, Error> {
        let mut f = File::open(filename)?;

        let mut data = String::new();
        f.read_to_string(&mut data).unwrap();

        let mut buf_reader = Cursor::new(data);

        read_from_p8format(filename, &mut buf_reader)
    }

    pub fn save_in_unicorn(&mut self, filename: &str, version: &str) {
        info!("[CARTRIDGE] [Cartridge] Save the modified cartridge in Unicorn format {:?}", filename);

        let mut f = File::create(filename).unwrap();

        f.write_all(b"Saved by Unicorn Console https://github.com/Gigoteur/UnicornConsole\n").unwrap();
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
}