pub mod edit;
pub mod info;
pub mod cartdata;
pub mod emscripten;
pub mod noise;
pub mod math;
pub mod utils;

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use std::fmt;
use std::cmp::{max, PartialOrd};
use std::io::prelude::*;
use std::time::Duration;

#[cfg(feature = "image")]
use image;

#[cfg(feature = "image")]
use gif;

#[cfg(feature = "image")]
use std::path::Path;
#[cfg(feature = "image")]
use std::fs::File;


use plugins::lua_plugin::plugin::LuaPlugin;
use plugins::python_plugin::plugin::PythonPlugin;
use plugins::javascript_plugin::plugin::JavascriptPlugin;

use config::Players;
use self::noise::Noise;
use gfx;
use cartridge::{Cartridge, CartridgeFormat};

include!(concat!(env!("OUT_DIR"), "/parameters.rs"));

#[inline]
pub fn clamp<T: PartialOrd>(val: T, min: T, max: T) -> T {
    if val > min {
        if val < max {
            val
        }
        else {
            max
        }
    }
    else {
        min
    }
}

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

#[derive(Debug,PartialEq)]
pub enum UnicornState {
    RUN,
    PAUSE,
    EDITOR,
    INTERACTIVE,
}

pub enum Code {
    UNKNOWN = 0,
    LUA = 1,
    PYTHON = 2,
    RUST = 3,
    JAVASCRIPT = 4,
}


#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn draw_logo(screen: &mut gfx::Screen) {
    let width = screen.width;
    let height = screen.height;

    screen
        .print(format!("Powered by Unicorn {:?}.{:?}.{:?}", VERSION, MAJOR_VERSION, MINOR_VERSION).to_string(),
        0,
        (height-16) as i32,
        7);
}

pub fn array_to_vec(arr: &[u8]) -> Vec<u8> {
     arr.iter().cloned().collect()
}


pub struct Menu {
    idx: u32,
}

impl Menu {
    pub fn new() -> Menu {
        Menu {
            idx: 0,
        }
    }

    pub fn update(&mut self, cartridges: &mut Vec<UnicornCartridge>, players: Arc<Mutex<Players>>) -> bool {
        if players.lock().unwrap().btnp(0, 2) {
            self.idx = clamp(self.idx - 1, 0, (cartridges.len() as u32) - 1);
        } else if players.lock().unwrap().btnp(0, 3) {
            self.idx = clamp(self.idx + 1, 0, (cartridges.len() as u32) - 1);
        }

        true
    }

    pub fn get_current_idx(&mut self) -> u32 {
        self.idx
    }

    pub fn draw(&mut self, cartridges: &mut Vec<UnicornCartridge>, screen: &mut gfx::Screen) {
        screen.cls(-1);

        let cartridges_info = format!("{:?}", cartridges.len());
        screen.print(cartridges_info, 0, 0, 7);

        let offset_x = 2;
        let offset_y = 10;

        if cartridges.len() > 0 {
            let mut idx = 0;
            let min_x = max(self.idx as i32-5, 0) as u32;
            let max_x = max(self.idx as i32+5, 10) as u32;
            
            let mut current_idx = 0;
            for cartridge in cartridges.iter_mut() {
                if idx >= min_x && idx <= max_x {
                    let filename = cartridge.filename.clone();

                    let data_to_print = format!("{:<width$}", filename, width = 10);
                    let mut color = 6;
                    if self.idx == idx {
                        color = 7;
                    }
                    screen.print(data_to_print, offset_x, offset_y + current_idx * 8, color);

                    current_idx += 1;
                }
                idx += 1;
            }


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
        items.push("Menu".to_string());
        items.push("Exit".to_string());

        PauseMenu {
            idx: 0,
            selected_idx: -1,
            items: items.clone(),
        }
    }

    pub fn reset(&mut self) {
        info!("[Unicorn][PauseMenu] Reset");

        self.selected_idx = -1;
        self.idx = 0;
    }

    pub fn stop(&mut self) -> bool {
        // Continue is clicked
        self.selected_idx == 0
    }

    pub fn quit(&mut self) -> bool {
        self.selected_idx == self.items.len() as i32 - 2
    }

    pub fn update(&mut self, players: Arc<Mutex<Players>>) -> bool {
        if players.lock().unwrap().btnp(0, 4) {
            self.selected_idx = self.idx as i32;
            if self.selected_idx == (self.items.len() as i32) - 1 {
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
            screen.cls(-1);
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

    pub fn switch_to_palette(&mut self, name: &str) {
        let values = &self.palettes[name];

        for (idx, rgb_value) in values.iter().enumerate() {
            PALETTE
                .lock()
                .unwrap()
                ._set_color(idx as u32, rgb_value.r, rgb_value.g, rgb_value.b);
        }

        self.name = name.to_string();
    }

    pub fn set_color(&mut self, color: u32, r: u8, g: u8, b: u8) {
        PALETTE.lock().unwrap().set_color(color, r, g, b);
    }

    pub fn set_colors(&mut self, colors: HashMap<u32, RGB>) {
        PALETTE.lock().unwrap().set_colors(colors);
    }

    pub fn get_color(&mut self, color: u32) -> u32 {
        PALETTE.lock().unwrap().get_color(color)
    }

    pub fn get_colors(&mut self) -> HashMap<u32, RGB> {
        let mut colors = HashMap::new();

        for (key, value) in PALETTE.lock().unwrap().colors.clone() {
            if key >= 16 {
                colors.insert(key, value);
            }
        }

        colors
    }

    pub fn reset(&mut self) {
        PALETTE.lock().unwrap().reset();
    }

    pub fn get_name(&mut self) -> String {
        self.name.clone()
    }
}

pub struct UnicornConfig {
    pub show_info_overlay: bool,
    pub show_mouse: bool,
}

impl UnicornConfig {
    pub fn new() -> UnicornConfig {
        UnicornConfig {
            show_info_overlay: false,
            show_mouse: false,
        }
    }

    pub fn toggle_info_overlay(&mut self) {
        self.show_info_overlay = !self.show_info_overlay;
    }

    pub fn toggle_mouse(&mut self, value: bool) {
        self.show_mouse = value;
    }
}

pub struct UnicornCartridge {
    pub filename: String,
    pub full_filename: String,
    pub loaded: bool,
    pub font_name: String,
    pub cartridge: Cartridge,
    pub lua_plugin: LuaPlugin,
    pub python_plugin: PythonPlugin,
    pub rust_plugin: Vec<Box<dyn RustPlugin>>,
    pub javascript_plugin: JavascriptPlugin,
}


impl fmt::Debug for UnicornCartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Unicorn Cartridge {{ cart: {:?} }}",
               self.cartridge)
    }
}


impl UnicornCartridge {
    pub fn new(cartridge: Cartridge, filename: String) -> UnicornCartridge {
        UnicornCartridge {
            filename: filename.clone(),
            full_filename: cartridge.filename.clone(),
            loaded: true,
            font_name: "pico-8".to_string(),
            cartridge: cartridge,
            lua_plugin: LuaPlugin::new(),
            python_plugin: PythonPlugin::new(),
            javascript_plugin: JavascriptPlugin::new(),
            rust_plugin: Vec::new(),
        }
    }

    pub fn empty(filename: String, full_filename: String) -> UnicornCartridge {
        UnicornCartridge {
            filename: filename.clone(),
            full_filename: full_filename.clone(),
            loaded: false,
            font_name: "pico-8".to_string(),
            cartridge: Cartridge::empty(),
            lua_plugin: LuaPlugin::new(),
            python_plugin: PythonPlugin::new(),
            rust_plugin: Vec::new(),
            javascript_plugin: JavascriptPlugin::new(),
        }
    }

    pub fn get_code_type(&mut self) -> Code {
        match self.cartridge.code.get_name().as_ref() {
            "lua" => Code::LUA,
            "python" => Code::PYTHON,
            "javascript" => Code::JAVASCRIPT,
            _ => Code::UNKNOWN,
        }
    }

    pub fn get_code(&mut self) -> String {
        self.cartridge.code.get_data().clone()
    }

    pub fn set_code(&mut self, data: Vec<String>) {
        self.cartridge.code.set_data(data);
    }

    pub fn get_palettes(&mut self) -> HashMap<u32, RGB> {
        self.cartridge.palette.colors.clone()
    }
}

pub struct Unicorn {
    pub screen: Arc<Mutex<gfx::Screen>>,
    pub info: Arc<Mutex<info::Info>>,
    pub palettes: Arc<Mutex<Palettes>>,
    pub players: Arc<Mutex<Players>>,
    pub configuration: Arc<Mutex<UnicornConfig>>,
    pub noise: Arc<Mutex<Noise>>,
    pub cartridges: Vec<UnicornCartridge>,
    pub editor: edit::edit::Editor,
    pub editing: bool,
    pub menu: Menu,
    pub current_cartridge: usize,
    pub current_code_type: Code,
    pub interactive: bool,
    pub state: UnicornState,
    pub pause_menu: PauseMenu,
    pub fps: f64,
    pub record: Record,
    pub update_return: bool,
    pub mouse_spr: Vec<u8>,
    pub version: u32,
    pub major_version: u32,
    pub minor_version: u32,
}

impl Unicorn {
    pub fn new() -> Unicorn {
        info!("[Unicorn] Creating new Unicorn");

        let screen = Arc::new(Mutex::new(gfx::Screen::new(MAP_WIDTH, MAP_HEIGHT)));

        Unicorn {
            screen: screen.clone(),
            info: Arc::new(Mutex::new(info::Info::new())),
            palettes: Arc::new(Mutex::new(Palettes::new())),
            players: Arc::new(Mutex::new(Players::new())),
            configuration: Arc::new(Mutex::new(UnicornConfig::new())),
            noise: Arc::new(Mutex::new(Noise::new())),
            cartridges: Vec::new(),
            editor: edit::edit::Editor::new(screen.clone()),
            editing: false,
            current_cartridge: 0,
            current_code_type: Code::UNKNOWN,
            interactive: false,
            state: UnicornState::RUN,
            pause_menu: PauseMenu::new(),
            menu: Menu::new(),
            fps: 0.0,
            record: Record::new(),
            update_return: true,
            mouse_spr: Unicorn::mouse_sprite(),
            version: VERSION,
            major_version: MAJOR_VERSION,
            minor_version: MINOR_VERSION,
        }
    }

    pub fn setup(&mut self) {
        info!("[Unicorn] Setup");

        self.palettes.lock().unwrap().init();

        self.reset();
    }

    pub fn stop(&mut self) {
    }

    pub fn toggle_debug(&mut self) {
        self.configuration.lock().unwrap().toggle_info_overlay();
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
        info!("[Unicorn] Reset");

        self.configuration.lock().unwrap().toggle_mouse(false);

        self.palettes.lock().unwrap().reset();
        self.palettes.lock().unwrap().switch_to_palette("pico-8");

        self.screen.lock().unwrap().init();

        self.update_return = true;
    }

    pub fn init_interactive(&mut self) {
        self.interactive = true;
    }

    pub fn debug_draw(&mut self) {
        let show_info_overlay = self.configuration.lock().unwrap().show_info_overlay;
        if show_info_overlay {
            let screen = &mut self.screen.lock().unwrap();
            let mouse_x = self.players.lock().unwrap().mouse_coordinate(0);
            let mouse_y = self.players.lock().unwrap().mouse_coordinate(1);

            let width = screen.width as i32;
            
            screen.rectfill(0, 0, width, 8, 0);

            screen.force_print(format!("{:.0}FPS {:.2?} {:.2?} {:?} {:?}",
                                       self.fps,
                                       mouse_x,
                                       mouse_y,
                                       &self.palettes.lock().unwrap().name,
                                       self.state)
                                       .to_string(),
                               0,
                               0,
                               7);
        }
    }

    pub fn update_time(&mut self, dt: Duration) {
        self.info.lock().unwrap().update(dt);

        self.players
            .lock()
            .unwrap()
            .update(self.info.lock().unwrap().elapsed_time);
    }

    pub fn init(&mut self) {
        match self.state {
            UnicornState::RUN => {
                self.call_init();
            }
            _ => {}
        }
    }

    pub fn update(&mut self) -> bool {
        match self.state {
            UnicornState::PAUSE => {
                if self.pause_menu.stop() {
                    self.state = UnicornState::RUN;
                }

                if self.pause_menu.quit() {
                    self.state = UnicornState::INTERACTIVE;
                }

                return self.pause_menu.update(self.players.clone());
            }
            UnicornState::RUN => {
                if self.is_end() {
                    return false;
                }

                self.call_update();
            }
            UnicornState::INTERACTIVE => {
                let return_value = self.menu.update(&mut self.cartridges, self.players.clone());
                if self.players.lock().unwrap().btnp(0, 4) {
                    let filename = self.cartridges[self.menu.get_current_idx() as usize].filename.clone();
                    let full_filename = self.cartridges[self.menu.get_current_idx() as usize].full_filename.clone();
                    self.load_cartridge(filename.as_str(), full_filename.as_str(), false);
                }

                return return_value;
            }
            UnicornState::EDITOR => {
                #[cfg(feature = "editor")]
                {
                    let cartridge = self.cartridges.get_mut(self.current_cartridge).unwrap();
                    return self.editor.update(cartridge, &mut self.screen.lock().unwrap(), self.players.clone());
                }
            }
        }
        true
    }

    pub fn draw(&mut self) {
        match self.state {
            UnicornState::PAUSE => {
                self.pause_menu.draw(&mut self.screen.lock().unwrap());
            }
            UnicornState::RUN => {
                self.call_draw();
            }
            UnicornState::INTERACTIVE => {
                self.menu.draw(&mut self.cartridges, &mut self.screen.lock().unwrap());
            }
            UnicornState::EDITOR => {
                #[cfg(feature = "editor")]
                {
                    self.editor
                        .draw(self.players.clone(), self.palettes.clone(), &mut self.screen.lock().unwrap());
                }
            }
        }

        if self.state != UnicornState::PAUSE && self.configuration.lock().unwrap().show_mouse {
            let mouse_x = self.players.lock().unwrap().mouse_coordinate(0);
            let mouse_y = self.players.lock().unwrap().mouse_coordinate(1);

            for y in 0..8 {
                for x in 0..8 {
                    let pixel = self.mouse_spr[x + y * 8];
                    if pixel != 0 {
                        self.screen
                            .lock()
                            .unwrap()
                            .putpixel_direct(mouse_x + x as i32, mouse_y + y as i32, pixel as u32);
                    }
                }
            }
        }

        if self.is_recording() {
            self.record();
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
        info!("[Unicorn] Start to record the frame");

        self.record.recording = true;
        self.record.images.clear();
        self.record.filename = filename.to_string();
    }

    pub fn record(&mut self) {
        info!("[Unicorn] Recording the frame {:?}", self.record.images.len());

        if self.record.nb % 4 == 0 {
            let mut buffer: Vec<u8> = Vec::new();
            let screen = &mut self.screen.lock().unwrap();

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

    #[cfg(not(feature = "image"))]
    pub fn stop_record(&mut self) {
    }

    #[cfg(feature = "image")]
    pub fn stop_record(&mut self) {
        info!("[Unicorn] Stop to record the frame {:?}",
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
            info!("[Unicorn] Generate frame {:?} {:?}/{:?}",
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

            info!("[Unicorn] Creating ImageBuffer {:?}", buffer.len());

            let image =
                image::ImageBuffer::from_raw(screen.height as u32, screen.width as u32, buffer)
                    .unwrap();

            info!("[Unicorn] Rotating image");
            let image = image::DynamicImage::ImageRgb8(image)
                .rotate270()
                .flipv();

            info!("[Unicorn] Creating gif Frame");
            let mut frame = gif::Frame::from_rgb(screen.width as u16,
                                                 screen.height as u16,
                                                 &image.raw_pixels());

            frame.delay = 1;
            encoder.write_frame(&frame).unwrap();
        }

        info!("[Unicorn] GIF created in {:?}", self.record.filename);
    }

    #[cfg(not(feature = "image"))]
    pub fn screenshot(&mut self, _filename: &str) {
    }

    #[cfg(feature = "image")]
    pub fn screenshot(&mut self, filename: &str) {
        let screen = &mut self.screen.lock().unwrap();

        info!("[Unicorn] Taking screenshot {:?}x{:?} in {:?}", screen.width, screen.height, filename);

        let mut buffer: Vec<u8> = vec![0; (screen.width*screen.height) * 3];

        let mut idx = 0;
        for x in 0..screen.width {
            for y in 0..screen.height {
                let value = screen.pget(x as u32, y as u32);
                let rgb_value = PALETTE.lock().unwrap().get_rgb(value);

                buffer[idx] = rgb_value.r;
                buffer[idx + 1] = rgb_value.g;
                buffer[idx + 2] = rgb_value.b;
                
                idx += 3;
            }
        }

        let image = image::ImageBuffer::from_raw(screen.height as u32, screen.width as u32, buffer)
            .unwrap();
        let image = image::DynamicImage::ImageRgb8(image)
            .rotate270()
            .flipv();

        let mut output = File::create(&Path::new(filename)).unwrap();
        image.save(&mut output, image::ImageFormat::PNG).unwrap();
    }

    pub fn save_current_cartridge(&mut self) {
        if !self.editing {
            return;
        }

        let screen = &self.screen.lock().unwrap();

        let cartridge = &mut self.cartridges[self.current_cartridge].cartridge;

        let output_filename = &cartridge.filename.clone();
        info!("[Unicorn][SAVE] Saving the current cartridge in {:?}",
              output_filename);

        info!("[Unicorn][SAVE] Set the new sprites");
        cartridge.gfx.set_sprites(screen.sprites.clone());
        info!("[Unicorn][SAVE] Set the new map");
        cartridge.map.set_map(screen.map.clone());
        info!("[Unicorn][SAVE] Set the new flags");
        cartridge.gff.set_flags(screen.sprites.clone());
        info!("[Unicorn][SAVE] Set the new palette");
        cartridge.palette.set_colors(self.palettes.lock().unwrap().get_colors());

        match cartridge.format {
            CartridgeFormat::UnicornFormat => {
                cartridge.save_in_unicorn(output_filename,
                                          format!("{:?}.{:?}.{:?}",
                                                self.version,
                                                self.major_version,
                                                self.minor_version)
                                                .as_str());
            }
            CartridgeFormat::UnicornSplittedFormat => {
                cartridge.save_in_unicorn_splitted();
            }
            _ => {}
        }
    }

    pub fn save_state() {

    }

    pub fn restore_state() {

    }

    pub fn switch_pause(&mut self) {
        info!("[Unicorn] Switch pause");

        let screen = &mut self.screen.lock().unwrap();

        match self.state {
            UnicornState::PAUSE => {
                if self.editing {
                    self.state = UnicornState::EDITOR;
                } else {
                    self.state = UnicornState::RUN;
                }
                /* Restore previous state */
                screen.restore();
                screen.font(&self.cartridges[self.current_cartridge].font_name.clone());
            }
            UnicornState::RUN => {
                /* Save state */
                screen.save();
                self.cartridges[self.current_cartridge].font_name = screen.get_font();

                screen.font("pico-8");

                self.pause_menu.reset();
                self.state = UnicornState::PAUSE;
            }
            UnicornState::INTERACTIVE => {
                self.pause_menu.reset();
                self.state = UnicornState::PAUSE;
                screen.save();
            }
            UnicornState::EDITOR => {
                self.pause_menu.reset();
                self.state = UnicornState::PAUSE;
                screen.save();
            }
        }
        info!("[Unicorn] End Switch pause");
    }

    #[allow(dead_code)]
    pub fn register<F: RustPlugin + 'static>(&mut self, callback: F) {
        info!("[Unicorn] Register new cartridge");

        let mut unicorn_cartridge = UnicornCartridge::empty("RustPlugin".to_string(), "RustPlugin".to_string());
        unicorn_cartridge.rust_plugin.push(Box::new(callback));
        self.add_cartridge(unicorn_cartridge);
    }

    pub fn _setup_screen(&mut self) {
        let cartridge = &self.cartridges[self.current_cartridge];

        info!("[Unicorn] Setup screen {:?}", cartridge);

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
            .set_map(cartridge.cartridge.map.map.clone());

        self.palettes.lock().unwrap().set_colors(cartridge.cartridge.palette.colors.clone());
    }

    pub fn _load_cartridge(&mut self,
                           cartridge: &mut UnicornCartridge,
                           editor: bool)
                           -> bool {
        info!("[Unicorn] Loading cartridge {:?}", cartridge);

        let data = cartridge.get_code();

        let mut ret: bool = false;

        match cartridge.get_code_type() {
            Code::LUA => {
                info!("[Unicorn] Loading LUA Plugin");

                cartridge
                    .lua_plugin
                    .load(self.players.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.noise.clone());

                ret = cartridge.lua_plugin.load_code(data.clone());
            }
            Code::JAVASCRIPT => {
                info!("[Unicorn] Loading JAVASCRIPT Plugin");

                cartridge
                    .javascript_plugin
                    .load(self.players.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.noise.clone());

                ret = cartridge.javascript_plugin.load_code(data.clone());
            }
            Code::PYTHON => {
                info!("[Unicorn] Loading PYTHON Plugin");

                cartridge
                    .python_plugin
                    .load(self.palettes.clone(),
                          self.players.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.noise.clone(),
                          self.configuration.clone());

                ret = cartridge.python_plugin.load_code(data.clone());
            }
            _ => (),
        }

        info!("[Unicorn] LOAD CARTRIDGE {:?}", ret);

        self.editing = editor;

        #[cfg(feature = "editor")]
        {
            if editor {
                self.editor
                    .init(self.configuration.clone(),
                          self.palettes.clone(),
                          &mut self.screen.lock().unwrap(),
                          cartridge.cartridge.filename.clone(),
                          data.clone());
                self.state = UnicornState::EDITOR;
                return true;
            }
        }

        ret
    }

    pub fn load_cartridge(&mut self, filename: &str, full_filename: &str, editor: bool) -> bool {
        info!("[Unicorn] Load cartridge from {:?}", filename);

        if filename == "DemoUnicorn" {
            self.current_cartridge = 0;
            self._setup_screen();
            self.cartridges[0].loaded = true;
            return true;
        }

        let cartridge;
        if filename.contains(".uni") {
            match Cartridge::from_unicorn_file(full_filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the unicorn cartridge {:?}", e),
            }
        }  else if filename.contains(".uc") {
            match Cartridge::from_unicorn_splitted_file(full_filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the unicorn splitted cartridge {:?}", e),
            }
        } else if filename.contains(".duc") {
            match Cartridge::from_dunicorn_file(full_filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the dUnicorn cartridge {:?}", e),
            }
        } else if filename.contains(".png") {
            match Cartridge::from_png_file(full_filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the Pico8 PNG cartridge {:?}", e),
            }
        } else if filename.contains(".p8") {
            match Cartridge::from_p8_file(full_filename) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the Pico8 P8 cartridge {:?}", e),
            }
        } else {
            panic!("[Unicorn] Unknown file format !");
        }

        let mut unicorn_cartridge = UnicornCartridge::new(cartridge, filename.to_string());
        let ret = self._load_cartridge(&mut unicorn_cartridge, editor);
        if ret {
            if self.state != UnicornState::EDITOR {
                self.state = UnicornState::RUN;
            }
            unicorn_cartridge.loaded = true;

            self.add_cartridge(unicorn_cartridge);
            self._setup_screen();


            self.init();
        }

        ret
    }

    pub fn add_cartridge(&mut self, mut new_cartridge: UnicornCartridge) {
        info!("[Unicorn] ADD cartridge {:?}", new_cartridge.filename);

        let mut exists = false;
        let mut idx = 0;
        for cartridge in self.cartridges.iter() {
            if cartridge.filename == new_cartridge.filename {
                exists = true;
                break;
            }
            idx += 1;
        }

        if !exists {
            self.current_cartridge = self.cartridges.len();
            self.current_code_type = new_cartridge.get_code_type();
            self.cartridges.push(new_cartridge);
        } else {
            self.current_cartridge = idx;
            self.current_code_type = new_cartridge.get_code_type();
            self.cartridges[idx as usize] = new_cartridge;
        }
    }

    pub fn load_cartridge_raw(&mut self,
                              filename: &str,
                              data: Vec<u8>,
                              editor: bool)
                              -> bool {
        let cartridge;

        if filename.contains(".uni") {
            match Cartridge::from_uni_raw(filename, data) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("Impossible to load the uni cartridge {:?}", e),
            }
        } else {
            panic!("[Unicorn] Unknown file");
        }

        let mut unicorn_cartridge = UnicornCartridge::new(cartridge, filename.to_string());
        let ret = self._load_cartridge(&mut unicorn_cartridge, editor);
        if ret {
            self.add_cartridge(unicorn_cartridge);
            self._setup_screen();
            if !editor {
                self.init();
            }
        }

        ret
    }

    pub fn switch_code(&mut self) {
        info!("[Unicorn] Switch code");

        #[cfg(feature = "editor")]
        {
            let idx = self.current_cartridge;

            if self.editing {
                info!("[Unicorn] Switch editor to run");

                self.cartridges[idx].set_code(self.editor.get_code());

                // Reload the code for the Unicorn format
               /* match self.cartridges[idx].cartridge.format {
                    CartridgeFormat::UnicornSplittedFormat => {
                        info!("[Unicorn] Reloading code section for the cartridge from the file");
                        self.cartridges[idx].cartridge.code.reload();
                    }
                    CartridgeFormat::UnicornFormat => {
                        info!("[Unicorn] Reloading code section for the cartridge from the buffer");
                        self.cartridges[idx].set_code(self.editor.get_code());
                    }
                    _ => (),
                }*/

                let data = self.cartridges[idx].get_code();
                let code_type = self.cartridges[idx].get_code_type();

                match code_type {
                    Code::LUA => {
                        self.cartridges[idx].lua_plugin.load_code(data);
                    }
                    Code::JAVASCRIPT => {
                        self.cartridges[idx].javascript_plugin.load_code(data);
                    }
                    Code::PYTHON => {
                        self.cartridges[idx].python_plugin.load_code(data);
                    }
                    _ => (),
                }

                self.editing = false;
                self.state = UnicornState::RUN;
                self.reset();
            } else {
                info!("[Unicorn] Switch run to editor");
                info!("[Unicorn] Back to {:?}/{:?}", self.current_cartridge, self.cartridges.len());
                let filename = self.cartridges[self.current_cartridge].filename.clone();
                let full_filename = self.cartridges[self.current_cartridge].full_filename.clone();

                if self.cartridges[self.current_cartridge].loaded == false {
                    self.load_cartridge(filename.as_str(), full_filename.as_str(), false);
                }
                let code = self.cartridges[self.current_cartridge].get_code();

                self.editor
                    .init(self.configuration.clone(),
                          self.palettes.clone(),
                          &mut self.screen.lock().unwrap(),
                          filename,
                          code);
                self.editing = true;
                self.state = UnicornState::EDITOR;
            }

            self.init();
        }
    }

    pub fn call_init(&mut self) {
        info!("[Unicorn] CALL INIT");

        self.reset();

        match self.current_code_type {
            Code::LUA => match self.cartridges[self.current_cartridge].lua_plugin.init() {
                _ => (),
            }
            Code::JAVASCRIPT => match self.cartridges[self.current_cartridge].javascript_plugin.init() {
                _ => (),
            }
            Code::PYTHON => match self.cartridges[self.current_cartridge].python_plugin.init() {
                _ => (),
            }
            Code::RUST => {
                for callback in &mut self.cartridges[self.current_cartridge].rust_plugin {
                    callback.init(&mut self.screen.lock().unwrap());
                }
            }
            _ => error!("[Unicorn] Impossible to match a plugin"),
        }
    }

    pub fn call_draw(&mut self) {
        match self.current_code_type {
            Code::LUA => {
                match self.cartridges[self.current_cartridge].lua_plugin.draw() {
                    Ok(()) => (),
                    Err(err) => error!("[Unicorn] [call_draw / lua]: {}", err),
                }
            }
            Code::JAVASCRIPT => {
                match self.cartridges[self.current_cartridge].javascript_plugin.draw() {
                    Ok(()) => (),
                    Err(err) => error!("[Unicorn] [call_draw / javascript]: {}", err),
                }
            }
            Code::PYTHON => {
                match self.cartridges[self.current_cartridge].python_plugin.draw() {
                    Ok(()) => (),
                    Err(err) => error!("[Unicorn] [call_draw / python]: {}", err),
                }
            }/*
            Code::RUST => {
                for callback in &mut self.cartridges[self.current_cartridge].rust_plugin {
                    callback.draw(&mut self.screen.lock().unwrap(),
                                  &mut self.info.lock().unwrap());
                }
            }*/
            _ => (),
        }
    }

    pub fn call_update(&mut self) {
        match self.current_code_type {
            Code::LUA => {
                match self.cartridges[self.current_cartridge].lua_plugin.update() {
                    Ok(()) => (),
                    Err(err) => error!("[Unicorn] [call_update / lua]: {}", err),
                }
            }
            
            Code::JAVASCRIPT => {
                match self.cartridges[self.current_cartridge].javascript_plugin.update() {
                    Ok(()) => (),
                    Err(err) => error!("[Unicorn] [call_update / javascript]: {}", err),
                }
            }
            Code::PYTHON => {
                match self.cartridges[self.current_cartridge].python_plugin.update() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_update / python]: {}", err),
                    }
            }/*
            Code::RUST => {
                self.update_return = true;

                for callback in &mut self.cartridges[self.current_cartridge].rust_plugin {
                    callback.update(&mut self.players.lock().unwrap());
                }
            }*/
            _ => (),
        }
    }
}
