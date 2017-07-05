pub mod editor;
pub mod info;
pub mod cartdata;
pub mod emscripten;
pub mod noise;
pub mod math;
pub mod packet;
pub mod wfc;

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use time;

use nalgebra::clamp;

use image;

use gif;
use gif::SetParameter;

use std::io::prelude::*;

use std::path::{Path, PathBuf};
use std::fs::File;
use glob::glob;

use plugins::lua_plugin::plugin::LuaPlugin;
use plugins::python_plugin::plugin::PythonPlugin;

use config::Players;
use self::noise::Noise;
use gfx;
use cartridge::{Cartridge, CartridgeFormat};
use sound::sound::{Sound, SoundInternal};

include!(concat!(env!("OUT_DIR"), "/parameters.rs"));

pub struct Palette {
    colors: HashMap<u32, RGB>,
    rcolors: HashMap<u32, u32>,
    cached_colors: [u32; 16],
}

impl Palette {
    pub fn new() -> Palette {
        Palette {
            colors: HashMap::new(),
            rcolors: HashMap::new(),
            cached_colors: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
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

    pub fn set_color(&mut self, color: u32, r: u8, g: u8, b: u8) {
        let u32_color = (r as u32) << 16 | (g as u32) << 8 | (b as u32);

        self.colors.insert(color, RGB::new(r, g, b));
        self.rcolors.insert(u32_color, color);
        if color < 16 {
            self.cached_colors[color as usize] = u32_color;
        }
    }

    pub fn get_color(&mut self, color: u32) -> u32 {
        match self.colors.get(&color) {
            Some(rgb_value) => {
                (rgb_value.r as u32) << 16 | (rgb_value.g as u32) << 8 | (rgb_value.b as u32)
            }
            _ => 0,
        }
    }
}

lazy_static! {
    pub static ref PALETTE: Mutex<Palette> = {
        Mutex::new(Palette::new())
    };
}

#[derive(Clone)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    pub fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB { r: r, g: g, b: b }
    }

    pub fn new_hexa(v: u32) -> RGB {
        RGB {
            r: ((v & 0xff0000) >> 16) as u8,
            g: ((v & 0x00ff00) >> 8) as u8,
            b: (v & 0x0000ff) as u8,
        }
    }
}

pub trait RustPlugin {
    fn init(&mut self, screen: &mut gfx::Screen) -> f64;
    fn update(&mut self, players: &mut Players) -> f64;
    fn draw(&mut self, screen: &mut gfx::Screen, info: &mut info::Info) -> f64;
}

#[derive(PartialEq)]
pub enum PX8Mode {
    PX8,
    PICO8,
}

pub enum PX8State {
    RUN,
    PAUSE,
    INTERACTIVE,
}

pub enum Code {
    UNKNOWN = 0,
    LUA = 1,
    PYTHON = 2,
    RUST = 3,
}


#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn draw_logo(screen: &mut gfx::Screen) {
    let logo = vec![
        0, 0, 0, 0, 0, 0, 0, 0,
        8, 0, 0, 0, 0, 0, 0, 8,
        0, 8, 8, 8, 8, 8, 8, 0,
        8, 8, 8, 9, 8, 8, 9, 8,
        0, 8, 8, 8, 8, 8, 8, 0,
        8, 0, 0, 0, 0, 0, 0, 8,
        0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0 ];

    screen
        .print("Powered by PX8".to_string(), 64, 112, 7);

    let idx_x = 114;
    let idx_y = 120;

    let mut x = 0;
    let mut y = 0;

    for c in logo {
        if x > 0 && x % 8 == 0 {
            x = 0;
            y += 1;
        }

        if c != 0 {
            screen.pset(idx_x + x, idx_y + y, c);
        }
        x += 1;
    }
}

pub struct Menu {
    idx: u32,
    cartridges: Vec<PathBuf>,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            idx: 0,
            cartridges: Vec::new(),
        }
    }

    pub fn reset(&mut self) {
        info!("[PX8][Menu] Reset");

        self.cartridges.clear();

        // PX8 Files
        for entry in glob("**/*.px8").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => self.cartridges.push(path),
                Err(e) => println!("{:?}", e),
            }
        }

        // P8 Files
        for entry in glob("**/*.p8").expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => self.cartridges.push(path),
                Err(e) => println!("{:?}", e),
            }
        }
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) -> bool {
        if players.lock().unwrap().btnp(0, 0) {
            self.idx = clamp(self.idx - 1, 0, (self.cartridges.len() as u32) - 1);
        } else if players.lock().unwrap().btnp(0, 1) {
            self.idx = clamp(self.idx + 1, 0, (self.cartridges.len() as u32) - 1);
        }

        true
    }

    pub fn selected_cartridge(&mut self) -> String {
        self.cartridges[self.idx as usize]
            .as_path()
            .to_str()
            .unwrap()
            .to_string()
    }

    pub fn draw(&mut self, screen: &mut gfx::Screen) {
        screen.cls();

        if self.cartridges.len() > 0 {
            let mut filename = self.cartridges[self.idx as usize]
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            filename.truncate(10);


            let data_to_print = format!("< {:<width$} >", filename, width = 10);
            screen.print(data_to_print, 30, 20, 7);

            let extension = self.cartridges[self.idx as usize]
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let extension_to_print = format!("CARTRIDGE {:}", extension);
            screen.print(extension_to_print, 30, 28, 7);

            let metadata = self.cartridges[self.idx as usize].metadata().unwrap();
            let metadata_to_print = format!("{:?} bytes", metadata.len());
            screen.print(metadata_to_print, 30, 36, 7);
            draw_logo(screen);
        }
    }
}

pub struct PauseMenu {
    idx: u32,
    selected_idx: i32,
    items: Vec<String>,
}

impl PauseMenu {
    pub fn new() -> PauseMenu {
        let mut items = Vec::new();

        items.push("Continue".to_string());
        items.push("Config".to_string());
        items.push("Quit".to_string());

        PauseMenu {
            idx: 0,
            selected_idx: -1,
            items: items.clone(),
        }
    }

    pub fn reset(&mut self) {
        info!("[PX8][PauseMenu] Reset");

        self.selected_idx = -1;
        self.idx = 0;
    }

    pub fn stop(&mut self) -> bool {
        // Continue is clicked
        self.selected_idx == 0
    }

    pub fn quit(&mut self) -> bool {
        self.selected_idx == self.items.len() as i32 - 1
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) -> bool {
        if players.lock().unwrap().btnp(0, 4) {
            self.selected_idx = self.idx as i32;
            if self.selected_idx == self.items.len() as i32 {
                return false;
            }
        } else {
            if players.lock().unwrap().btnp(0, 2) {
                self.idx = clamp(self.idx - 1, 0, (self.items.len() as u32) - 1);
            }

            if players.lock().unwrap().btnp(0, 3) {
                self.idx = clamp(self.idx + 1, 0, (self.items.len() as u32) - 1);
            }
        }

        true
    }

    pub fn draw(&mut self, screen: &mut gfx::Screen) {
        if self.selected_idx == -1 {
            let idx_x = (screen.width / 2 - 20) as i32;
            let idx_y = (screen.height / 2 - 10) as i32;

            screen.rectfill(idx_x,
                            idx_y - 5,
                            idx_x + 40,
                            idx_y + 10 * self.items.len() as i32,
                            11);


            screen.rect(idx_x - 1,
                        idx_y - 6,
                        idx_x + 41,
                        idx_y + 1 + 10 * self.items.len() as i32,
                        0);

            screen.print(">".to_string(), idx_x, idx_y + (self.idx as i32) * 10, 3);

            draw_logo(screen);

            for (pos, item) in self.items.iter().enumerate() {
                screen.print(item.to_string(), idx_x + 5, idx_y + (pos as i32) * 10, 7);
            }

        }

        if self.selected_idx == 1 {
            screen.cls();
        }
    }
}

pub struct Record {
    pub recording: bool,
    pub images: Vec<u8>,
    pub filename: String,
    pub nb: i32,
}

impl Record {
    pub fn new() -> Record {
        let images = Vec::new();

        Record {
            recording: false,
            images: images,
            filename: "".to_string(),
            nb: 0,
        }
    }
}

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

    pub fn init(&mut self) {
        // load palettes statically for emscripten
        self.load("a64".to_string(),
                  include_str!("../../sys/assets/palettes/a64.gpl").to_string());
        self.load("apple-ii".to_string(),
                  include_str!("../../sys/assets/palettes/apple-ii.gpl").to_string());
        self.load("arne-paldac".to_string(),
                  include_str!("../../sys/assets/palettes/arne-paldac.gpl").to_string());
        self.load("arne16".to_string(),
                  include_str!("../../sys/assets/palettes/arne16.gpl").to_string());
        self.load("arne32".to_string(),
                  include_str!("../../sys/assets/palettes/arne32.gpl").to_string());
        self.load("atari2600-ntsc".to_string(),
                  include_str!("../../sys/assets/palettes/atari2600-ntsc.gpl").to_string());
        self.load("atari2600-pal".to_string(),
                  include_str!("../../sys/assets/palettes/atari2600-pal.gpl").to_string());
        self.load("cg-arne".to_string(),
                  include_str!("../../sys/assets/palettes/cg-arne.gpl").to_string());
        self.load("cga".to_string(),
                  include_str!("../../sys/assets/palettes/cga.gpl").to_string());
        self.load("commodore-plus4".to_string(),
                  include_str!("../../sys/assets/palettes/commodore-plus4.gpl").to_string());
        self.load("commodore-vic20".to_string(),
                  include_str!("../../sys/assets/palettes/commodore-vic20.gpl").to_string());
        self.load("commodore64".to_string(),
                  include_str!("../../sys/assets/palettes/commodore64.gpl").to_string());
        self.load("copper-tech".to_string(),
                  include_str!("../../sys/assets/palettes/copper-tech.gpl").to_string());
        self.load("cpc-boy".to_string(),
                  include_str!("../../sys/assets/palettes/cpc-boy.gpl").to_string());
        self.load("db16".to_string(),
                  include_str!("../../sys/assets/palettes/db16.gpl").to_string());
        self.load("db32".to_string(),
                  include_str!("../../sys/assets/palettes/db32.gpl").to_string());
        self.load("edg16".to_string(),
                  include_str!("../../sys/assets/palettes/edg16.gpl").to_string());
        self.load("edg32".to_string(),
                  include_str!("../../sys/assets/palettes/edg32.gpl").to_string());
        self.load("eroge-copper".to_string(),
                  include_str!("../../sys/assets/palettes/eroge-copper.gpl").to_string());
        self.load("gameboy-color-type1".to_string(),
                  include_str!("../../sys/assets/palettes/gameboy-color-type1.gpl").to_string());
        self.load("gameboy".to_string(),
                  include_str!("../../sys/assets/palettes/gameboy.gpl").to_string());
        self.load("google-ui".to_string(),
                  include_str!("../../sys/assets/palettes/google-ui.gpl").to_string());
        self.load("jmp".to_string(),
                  include_str!("../../sys/assets/palettes/jmp.gpl").to_string());
        self.load("mail24".to_string(),
                  include_str!("../../sys/assets/palettes/mail24.gpl").to_string());
        self.load("master-system".to_string(),
                  include_str!("../../sys/assets/palettes/master-system.gpl").to_string());
        self.load("monokai".to_string(),
                  include_str!("../../sys/assets/palettes/monokai.gpl").to_string());
        self.load("nes-ntsc".to_string(),
                  include_str!("../../sys/assets/palettes/nes-ntsc.gpl").to_string());
        self.load("nes".to_string(),
                  include_str!("../../sys/assets/palettes/nes.gpl").to_string());
        self.load("pico-8".to_string(),
                  include_str!("../../sys/assets/palettes/pico-8.gpl").to_string());
        self.load("psygnork".to_string(),
                  include_str!("../../sys/assets/palettes/psygnork.gpl").to_string());
        self.load("smile-basic".to_string(),
                  include_str!("../../sys/assets/palettes/smile-basic.gpl").to_string());
        self.load("solarized".to_string(),
                  include_str!("../../sys/assets/palettes/solarized.gpl").to_string());
        self.load("teletext".to_string(),
                  include_str!("../../sys/assets/palettes/teletext.gpl").to_string());
        self.load("vga-13h".to_string(),
                  include_str!("../../sys/assets/palettes/vga-13h.gpl").to_string());
        self.load("web-safe-colors".to_string(),
                  include_str!("../../sys/assets/palettes/web-safe-colors.gpl").to_string());
        self.load("win16".to_string(),
                  include_str!("../../sys/assets/palettes/win16.gpl").to_string());
        self.load("x11".to_string(),
                  include_str!("../../sys/assets/palettes/x11.gpl").to_string());
        self.load("zx-spectrum".to_string(),
                  include_str!("../../sys/assets/palettes/zx-spectrum.gpl").to_string());
    }

    pub fn load(&mut self, name: String, data: String) {
        let buf_reader = Cursor::new(data);

        let mut values = Vec::new();

        for line in buf_reader.lines() {
            let line = line.unwrap();
            let l = line.trim_left().to_string();

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

    pub fn next(&mut self) {
        self.palette_idx = (self.palette_idx + 1) % self.palettes_list.len() as u32;
        let palette_name = self.palettes_list[self.palette_idx as usize].clone();
        self.switch_to(&palette_name);
    }

    pub fn switch_to(&mut self, name: &str) {
        let values = &self.palettes[name];

        for (idx, rgb_value) in values.iter().enumerate() {
            PALETTE
                .lock()
                .unwrap()
                .set_color(idx as u32, rgb_value.r, rgb_value.g, rgb_value.b);
        }

        self.name = name.to_string();
    }

    pub fn set_color(&mut self, color: u32, r: u8, g: u8, b: u8) {
        PALETTE.lock().unwrap().set_color(color, r, g, b);
    }

    pub fn get_color(&mut self, color: u32) -> u32 {
        PALETTE.lock().unwrap().get_color(color)
    }

    pub fn reset(&mut self) {
        PALETTE.lock().unwrap().reset();
    }
}

pub struct PX8Config {
    pub show_info_overlay: bool,
    pub show_mouse: bool,
}

impl PX8Config {
    pub fn new() -> PX8Config {
        PX8Config {
            show_info_overlay: false,
            show_mouse: false,
        }
    }

    pub fn toggle_info_overlay(&mut self) {
        self.show_info_overlay = !self.show_info_overlay;
    }

    pub fn toggle_mouse(&mut self) {
        self.show_mouse = !self.show_mouse;
    }

    pub fn show_mouse(&mut self, value: bool) {
        self.show_mouse = value;
    }
}

pub struct PX8Cartridge {
    pub cartridge: Cartridge,
    pub lua_plugin: LuaPlugin,
    pub python_plugin: PythonPlugin,
    pub rust_plugin: Vec<Box<RustPlugin>>,
    pub edit: bool,
}

impl PX8Cartridge {
    pub fn new(cartridge: Cartridge) -> PX8Cartridge {
        PX8Cartridge {
            cartridge: cartridge,
            lua_plugin: LuaPlugin::new(),
            python_plugin: PythonPlugin::new(),
            rust_plugin: Vec::new(),
            edit: false,
        }
    }

    pub fn empty() -> PX8Cartridge {
        PX8Cartridge {
            cartridge: Cartridge::empty(),
            lua_plugin: LuaPlugin::new(),
            python_plugin: PythonPlugin::new(),
            rust_plugin: Vec::new(),
            edit: false,
        }
    }

    pub fn set_code_type(&mut self, code: Code) {
        match code {
            Code::PYTHON => self.cartridge.code.set_name("python".to_string()),
            Code::LUA => self.cartridge.code.set_name("lua".to_string()),
            _ => (),
        }

    }

    pub fn get_code_type(&mut self) -> Code {
        match self.cartridge.code.get_name().as_ref() {
            "lua" => Code::LUA,
            "python" => Code::PYTHON,
            _ => Code::UNKNOWN,
        }
    }

    pub fn get_code(&mut self) -> String {
        self.cartridge.code.get_data().clone()
    }
}

pub struct PX8 {
    pub screen: Arc<Mutex<gfx::Screen>>,
    pub info: Arc<Mutex<info::Info>>,
    pub sound: Arc<Mutex<Sound>>,
    pub sound_internal: SoundInternal,
    pub palettes: Arc<Mutex<Palettes>>,
    pub players: Arc<Mutex<Players>>,
    pub configuration: Arc<Mutex<PX8Config>>,
    pub noise: Arc<Mutex<Noise>>,
    pub cartridges: Vec<PX8Cartridge>,
    pub editor: editor::Editor,
    pub menu: Menu,
    pub current_cartridge: usize,
    pub current_code_type: Code,
    pub state: PX8State,
    pub pause_menu: PauseMenu,
    pub fps: f64,
    pub draw_time: f64,
    pub init_time: f64,
    pub update_time: f64,
    pub record: Record,
    pub draw_return: bool,
    pub update_return: bool,
    pub mouse_spr: Vec<u8>,
}

impl PX8 {
    pub fn new() -> PX8 {
        info!("[PX8] Creating new PX8");

        let sound_internal = SoundInternal::new();
        let csend = sound_internal.csend.clone();

        PX8 {
            screen: Arc::new(Mutex::new(gfx::Screen::new(128, 128))),
            sound_internal: sound_internal,
            sound: Arc::new(Mutex::new(Sound::new(csend))),
            info: Arc::new(Mutex::new(info::Info::new())),
            palettes: Arc::new(Mutex::new(Palettes::new())),
            players: Arc::new(Mutex::new(Players::new())),
            configuration: Arc::new(Mutex::new(PX8Config::new())),
            noise: Arc::new(Mutex::new(Noise::new())),
            cartridges: Vec::new(),
            editor: editor::Editor::new(),
            current_cartridge: 0,
            current_code_type: Code::UNKNOWN,
            state: PX8State::RUN,
            pause_menu: PauseMenu::new(),
            menu: Menu::new(),
            fps: 0.0,
            draw_time: 0.0,
            init_time: 0.0,
            update_time: 0.0,
            record: Record::new(),
            draw_return: true,
            update_return: true,
            mouse_spr: PX8::mouse_sprite(),
        }
    }

    pub fn setup(&mut self) {
        info!("[PX8] Setup");
        self.sound_internal.init();
        self.reset();
    }

    pub fn update_sound(&mut self) {
        self.sound_internal.update(self.sound.clone());
    }

    pub fn stop(&mut self) {
        self.sound_internal.stop();
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn mouse_sprite() -> Vec<u8> {
        vec![0, 1, 0, 0, 0, 0, 0, 0,
             1, 7, 1, 0, 0, 0, 0, 0,
             1, 7, 7, 1, 0, 0, 0, 0,
             1, 7, 7, 7, 1, 0, 0, 0,
             1, 7, 7, 7, 7, 1, 0, 0,
             1, 7, 7, 1, 1, 0, 0, 0,
             0, 1, 1, 7, 1, 0, 0, 0,
             0, 0, 0, 0, 0, 0, 0, 0]
    }

    pub fn reset(&mut self) {
        info!("[PX8] Reset");

        self.palettes.lock().unwrap().init();
        self.palettes.lock().unwrap().switch_to("pico-8");

        self.screen.lock().unwrap().init();
        self.update_return = true;
        self.draw_return = true;
    }

    pub fn init_interactive(&mut self) {
        self.menu.reset();
        self.state = PX8State::INTERACTIVE;
    }

    pub fn next_palette(&mut self) {
        self.palettes.lock().unwrap().next();
    }

    pub fn debug_draw(&mut self) {
        if self.configuration.lock().unwrap().show_info_overlay {
            let mut screen = &mut self.screen.lock().unwrap();

            let width = screen.width as i32;
            screen.rectfill(0, 0, width, 8, 0);

            screen.force_print(format!("{:.0}FPS {:.2} {:.2} {:?}",
                                       self.fps,
                                       self.draw_time,
                                       self.update_time,
                                       &self.palettes.lock().unwrap().name)
                                       .to_string(),
                               0,
                               0,
                               7);
        }
    }

    pub fn init(&mut self) {
        match self.state {
            PX8State::RUN => {
                self.init_time = self.call_init() * 1000.0;
            }
            _ => {}
        }
    }

    pub fn update(&mut self) -> bool {
        match self.state {
            PX8State::PAUSE => {
                if self.pause_menu.stop() {
                    self.state = PX8State::RUN;
                }

                if self.pause_menu.quit() {
                    self.menu.reset();
                    self.state = PX8State::INTERACTIVE;
                    self.sound_internal.stop();
                }

                return self.pause_menu.update(self.players.clone());
            }
            PX8State::RUN => {
                if self.is_end() {
                    return false;
                }

                self.update_time = self.call_update() * 1000.0;
            }
            PX8State::INTERACTIVE => {
                let return_value = self.menu.update(self.players.clone());
                if self.players.lock().unwrap().btnp(0, 4) {
                    let filename = self.menu.selected_cartridge().clone();
                    self.load_cartridge(filename.as_str(), false, PX8Mode::PX8);
                }

                return return_value;
            }
        }
        true
    }

    pub fn draw(&mut self) {
        match self.state {
            PX8State::PAUSE => {
                self.pause_menu.draw(&mut self.screen.lock().unwrap());
            }
            PX8State::RUN => {
                self.draw_time = self.call_draw() * 1000.0;

                if self.configuration.lock().unwrap().show_mouse {
                    let mouse_x = self.players.lock().unwrap().mouse_coordinate(0);
                    let mouse_y = self.players.lock().unwrap().mouse_coordinate(1);

                    for y in 0..8 {
                        for x in 0..8 {
                            let pixel = self.mouse_spr[x + y * 8];
                            if pixel != 0 {
                                self.screen
                                    .lock()
                                    .unwrap()
                                    .putpixel_direct(mouse_x + x as i32,
                                                     mouse_y + y as i32,
                                                     pixel as u32);
                            }
                        }
                    }
                }

                if self.is_recording() {
                    self.record();
                }
            }
            PX8State::INTERACTIVE => {
                self.menu.draw(&mut self.screen.lock().unwrap());
            }
        }

        self.debug_draw();
    }

    pub fn is_end(&self) -> bool {
        !self.update_return
    }

    pub fn is_recording(&self) -> bool {
        self.record.recording
    }

    pub fn start_record(&mut self, filename: &str) {
        info!("[PX8] Start to record the frame");

        self.record.recording = true;
        self.record.images.clear();
        self.record.filename = filename.to_string();
    }

    pub fn record(&mut self) {
        info!("[PX8] Recording the frame {:?}", self.record.images.len());

        if self.record.nb % 4 == 0 {
            let mut buffer: Vec<u8> = Vec::new();
            let mut screen = &mut self.screen.lock().unwrap();

            for x in 0..screen.width {
                for y in 0..screen.height {
                    let value = screen.pget(x as u32, y as u32);
                    let rgb_value = PALETTE.lock().unwrap().get_rgb(value);

                    buffer.push(rgb_value.r);
                    buffer.push(rgb_value.g);
                    buffer.push(rgb_value.b);
                }
            }
            self.record.images.append(&mut buffer);
        }

        self.record.nb += 1;
    }

    pub fn stop_record(&mut self, scale: usize) {
        info!("[PX8] Stop to record the frame {:?}",
              self.record.images.len());

        let screen = &self.screen.lock().unwrap();

        self.record.recording = false;

        let mut filedata = File::create(self.record.filename.clone()).unwrap();

        let mut encoder = gif::Encoder::new(&mut filedata,
                                            screen.width as u16,
                                            screen.height as u16,
                                            &[])
                .unwrap();

        encoder.set(gif::Repeat::Infinite).unwrap();

        let mut idx = 0;
        for i in 0..self.record.images.len() / (screen.width * screen.height * 3) {
            info!("[PX8] Generate frame {:?} {:?}/{:?}",
                  i,
                  self.record.images.len(),
                  idx);

            let mut buffer: Vec<u8> = Vec::new();

            for _ in 0..screen.width {
                for _ in 0..screen.height {
                    buffer.push(self.record.images[idx]);
                    buffer.push(self.record.images[idx + 1]);
                    buffer.push(self.record.images[idx + 2]);
                    idx += 3;
                }
            }

            info!("[PX8] Creating ImageBuffer {:?}", buffer.len());

            let image =
                image::ImageBuffer::from_raw(screen.width as u32, screen.height as u32, buffer)
                    .unwrap();

            info!("[PX8] Rotating image");
            let image = image::DynamicImage::ImageRgb8(image)
                .rotate90()
                .resize((screen.width * scale) as u32,
                        (screen.height * scale) as u32,
                        image::FilterType::Nearest)
                .fliph();

            info!("[PX8] Creating gif Frame");
            let mut frame = gif::Frame::from_rgb((screen.width * scale) as u16,
                                                 (screen.height * scale) as u16,
                                                 &image.raw_pixels());

            frame.delay = 1;
            encoder.write_frame(&frame).unwrap();
        }

        info!("[PX8] GIF created in {:?}", self.record.filename);
    }

    pub fn screenshot(&mut self, filename: &str) {
        info!("[PX8] Taking screenshot in {:?}", filename);

        let mut screen = &mut self.screen.lock().unwrap();

        let mut buffer: Vec<u8> = Vec::new();

        for x in 0..screen.width {
            for y in 0..screen.height {
                let value = screen.pget(x as u32, y as u32);
                let rgb_value = PALETTE.lock().unwrap().get_rgb(value);

                buffer.push(rgb_value.r);
                buffer.push(rgb_value.g);
                buffer.push(rgb_value.b);
            }
        }

        let image = image::ImageBuffer::from_raw(screen.width as u32, screen.height as u32, buffer)
            .unwrap();
        let image = image::DynamicImage::ImageRgb8(image)
            .rotate270()
            .resize((screen.width * 4) as u32,
                    (screen.height * 4) as u32,
                    image::FilterType::Nearest)
            .flipv();

        let mut output = File::create(&Path::new(filename)).unwrap();
        image.save(&mut output, image::ImageFormat::PNG).unwrap();
    }

    pub fn save_current_cartridge(&mut self) {
        let screen = &self.screen.lock().unwrap();

        let cartridge = &mut self.cartridges[self.current_cartridge].cartridge;

        let output_filename = &cartridge.filename.clone();
        info!("[PX8] Saving the current cartridge in {:?}",
              output_filename);

        cartridge.gfx.set_sprites(screen.sprites.clone());
        cartridge.map.set_map(screen.map);

        match cartridge.format {
            CartridgeFormat::P8Format => {
                cartridge.save_in_p8(output_filename);
            }
            CartridgeFormat::PngFormat => {
                cartridge.save_in_p8(output_filename);
            }
            CartridgeFormat::Px8Format => {
                cartridge.save_in_dpx8();
            }
        }
    }

    pub fn switch_pause(&mut self) {
        info!("[PX8] Switch pause");

        let mut screen = &mut self.screen.lock().unwrap();

        match self.state {
            PX8State::PAUSE => {
                self.state = PX8State::RUN;
                screen.restore();
                self.sound_internal.resume();
            }
            PX8State::RUN => {
                self.pause_menu.reset();
                self.state = PX8State::PAUSE;
                screen.save();
                self.sound_internal.pause();
            }
            PX8State::INTERACTIVE => {
                self.pause_menu.reset();
                self.state = PX8State::PAUSE;
                screen.save();
            }
        }
        info!("[PX8] End Switch pause");
    }

    #[allow(dead_code)]
    pub fn register<F: RustPlugin + 'static>(&mut self, callback: F) {
        info!("[PX8] Register new cartridge");

        let mut px8_cartridge = PX8Cartridge::empty();
        px8_cartridge.rust_plugin.push(Box::new(callback));
        self.add_cartridge(px8_cartridge);
    }

    pub fn _load_cartridge(&mut self, cartridge: &mut PX8Cartridge, editor: bool) -> bool {
        info!("[PX8] Loading cartridge");

        let data;
        if editor {
            self.editor.init();
            data = self.editor.data.clone();
            cartridge.edit = true;
        } else {
            data = cartridge.get_code();
        }

        let mut ret: bool = false;

        if editor {
            cartridge.set_code_type(Code::PYTHON);
        }

        match cartridge.get_code_type() {
            Code::LUA => {
                info!("[PX8] Loading LUA Plugin");

                cartridge
                    .lua_plugin
                    .load(self.players.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.noise.clone(),
                          self.sound.clone());

                ret = cartridge.lua_plugin.load_code(data);
            }
            Code::PYTHON => {
                info!("[PX8] Loading PYTHON Plugin");

                cartridge
                    .python_plugin
                    .load(self.palettes.clone(),
                          self.players.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.sound.clone(),
                          self.noise.clone(),
                          self.configuration.clone());

                ret = cartridge.python_plugin.load_code(data);
            }
            _ => (),
        }

        if ret {
            self.screen
                .lock()
                .unwrap()
                .set_sprites(cartridge.cartridge.gfx.sprites.clone());

            self.screen
                .lock()
                .unwrap()
                .set_sprites_flags(cartridge.cartridge.gff.flags.clone());

            self.screen
                .lock()
                .unwrap()
                .set_map(cartridge.cartridge.map.map);

            self.state = PX8State::RUN;
        }

        ret
    }

    pub fn load_cartridge(&mut self, filename: &str, editor: bool, mode: PX8Mode) -> bool {
        let mut cartridge;

        if filename.contains(".png") {
            match Cartridge::from_png_file(filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[PX8] Impossible to load the png cartridge {:?}", e),
            }
        } else if filename.contains(".p8") {
            match Cartridge::from_p8_file(filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[PX8] Impossible to load the p8 cartridge {:?}", e),
            }
        } else if filename.contains(".py") {
            match Cartridge::from_p8_file(filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[PX8] Impossible to load the p8 cartridge {:?}", e),
            }
        } else if filename.contains(".px8") {
            match Cartridge::from_px8_file(filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[PX8] Impossible to load the px8 cartridge {:?}", e),
            }
        } else {
            panic!("[PX8] Unknown file format !");
        }

        cartridge.set_mode(mode == PX8Mode::PICO8);
        let mut px8_cartridge = PX8Cartridge::new(cartridge);
        let ret = self._load_cartridge(&mut px8_cartridge, editor);
        if ret {
            self.add_cartridge(px8_cartridge);
            self.init();
        }

        ret
    }

    pub fn add_cartridge(&mut self, mut cartridge: PX8Cartridge) {
        info!("[PX8] Add cartridge");

        self.current_cartridge = self.cartridges.len();
        self.current_code_type = cartridge.get_code_type();
        self.cartridges.push(cartridge);
    }

    #[allow(dead_code)]
    pub fn load_cartridge_raw(&mut self,
                              filename: &str,
                              data: Vec<u8>,
                              editor: bool,
                              mode: PX8Mode)
                              -> bool {
        let mut cartridge;

        if filename.contains(".png") {
            match Cartridge::from_png_raw(filename, data) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("Impossible to load the png cartridge {:?}", e),
            }
        } else if filename.contains(".p8") {
            match Cartridge::from_p8_raw(filename, data) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("Impossible to load the p8 cartridge {:?}", e),
            }
        } else if filename.contains(".py") {
            match Cartridge::from_p8_raw(filename, data) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("Impossible to load the p8 cartridge {:?}", e),
            }
        } else {
            panic!("[PX8] Unknown file");
        }

        cartridge.set_mode(mode == PX8Mode::PICO8);
        let mut px8_cartridge = PX8Cartridge::new(cartridge);
        let ret = self._load_cartridge(&mut px8_cartridge, editor);
        if ret {
            self.add_cartridge(px8_cartridge);
            self.init();
        }

        ret
    }

    pub fn switch_code(&mut self) {
        let idx = self.current_cartridge;

        let data;
        let code_type;

        if self.cartridges[idx].edit {
            // Reload the code for the px8 format
            match self.cartridges[idx].cartridge.format {
                CartridgeFormat::Px8Format => {
                    info!("[PX8] Reloading code section for the cartridge");
                    self.cartridges[idx].cartridge.code.reload();
                }
                _ => (),
            }

            data = self.cartridges[idx].get_code();
            self.cartridges[idx].edit = false;
            code_type = self.cartridges[idx].get_code_type();
        } else {
            data = self.editor.data.clone();
            self.cartridges[idx].edit = true;
            code_type = Code::PYTHON;
        }

        match code_type {
            Code::LUA => {
                self.cartridges[idx].lua_plugin.load_code(data);
            }
            Code::PYTHON => {
                self.cartridges[idx].python_plugin.load_code(data);
            }
            _ => (),
        }

        self.reset();
    }

    pub fn call_init(&mut self) -> f64 {
        info!("[PX8] CALL INIT");

        self.reset();

        let current_time = time::now();

        match self.current_code_type {
            Code::LUA => self.cartridges[self.current_cartridge].lua_plugin.init(),
            Code::PYTHON => self.cartridges[self.current_cartridge].python_plugin.init(),
            Code::RUST => {
                self.draw_return = true;
                for callback in &mut self.cartridges[self.current_cartridge].rust_plugin {
                    callback.init(&mut self.screen.lock().unwrap());
                }
            }
            _ => panic!("[PX8] Impossible to match a plugin"),
        }

        let diff_time = time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
                          (diff_time.num_seconds() * 1000000000) as f64;

        // Elapsed time
        diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0
    }

    pub fn call_draw(&mut self) -> f64 {
        let current_time = time::now();

        match self.current_code_type {
            Code::LUA => {
                self.draw_return = self.cartridges[self.current_cartridge].lua_plugin.draw()
            }
            Code::PYTHON => {
                self.draw_return = self.cartridges[self.current_cartridge].python_plugin.draw()
            }
            Code::RUST => {
                self.draw_return = true;

                for callback in &mut self.cartridges[self.current_cartridge].rust_plugin {
                    callback.draw(&mut self.screen.lock().unwrap(),
                                  &mut self.info.lock().unwrap());
                }
            }
            _ => (),
        }

        let diff_time = time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
                          (diff_time.num_seconds() * 1000000000) as f64;

        // Elapsed time
        diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0
    }

    pub fn call_update(&mut self) -> f64 {
        let current_time = time::now();

        match self.current_code_type {
            Code::LUA => {
                self.update_return = self.cartridges[self.current_cartridge].lua_plugin.update()
            }
            Code::PYTHON => {
                self.update_return = self.cartridges[self.current_cartridge]
                    .python_plugin
                    .update()
            }
            Code::RUST => {
                self.update_return = true;

                for callback in &mut self.cartridges[self.current_cartridge].rust_plugin {
                    callback.update(&mut self.players.lock().unwrap());
                }
            }
            _ => (),
        }

        let diff_time = time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) -
                          (diff_time.num_seconds() * 1000000000) as f64;

        // Elapsed time
        diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0
    }
}
