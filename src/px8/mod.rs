pub mod info;
pub mod packet;
pub mod cartdata;

use std;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::Cursor;
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};
use time;

use chan;
use chan::{Receiver, Sender};

use image;

use gif;
use gif::SetParameter;

use std::io;
use std::io::prelude::*;

use std::error::Error;
use std::fmt;
use std::path::Path;
use std::fs::File;

use plugins::lua_plugin::plugin::LuaPlugin;
use plugins::python_plugin::plugin::PythonPlugin;

use config::Players;
use self::info::Info;
use gfx;
use gfx::{Screen, Sprite};
use cartridge::{Cartridge, CartridgeFormat};


pub const SCREEN_WIDTH: usize = 128;
pub const SCREEN_HEIGHT: usize = 128;

pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

pub type ScreenBuffer = [Color; SCREEN_PIXELS];

pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    DarkBlue = 1,
    DarkPurple = 2,
    DarkGreen = 3,
    Brown = 4,
    DarkGray = 5,
    LightGray = 6,
    White = 7,
    Red = 8,
    Orange = 9,
    Yellow = 10,
    Green = 11,
    Blue = 12,
    Indigo = 13,
    Pink = 14,
    Peach = 15,
    UNKNOWN = 16,
}

impl Color {
    #[inline]
    pub fn from_u8(value: u8) -> Color {
        use self::Color::*;
        match value {
            0 => Black,
            1 => DarkBlue,
            2 => DarkPurple,
            3 => DarkGreen,
            4 => Brown,
            5 => DarkGray,
            6 => LightGray,
            7 => White,
            8 => Red,
            9 => Orange,
            10 => Yellow,
            11 => Green,
            12 => Blue,
            13 => Indigo,
            14 => Pink,
            15 => Peach,
            _ => UNKNOWN,
        }
    }
    #[inline]
    pub fn to_rgb(value: Color) -> RGB {
        use self::Color::*;
        match value {
            Black => RGB{r:0, g:0, b:0}, // 0   0   0 black
            DarkBlue => RGB{r:32, g:51, b:123}, // 32  51 123 dark_blue
            DarkPurple => RGB{r:126, g:37, b:83}, // 126  37  83 dark_purple
            DarkGreen => RGB{r:0, g:144, b:61}, // 0 144  61 dark_green
            Brown => RGB{r:171, g:82, b:54}, // 171  82  54 brown
            DarkGray => RGB{r:52, g:54, b:53}, // 52  54  53 dark_gray
            LightGray => RGB{r:194, g:195, b:199}, // 194 195 199 light_gray
            White => RGB{r:255, g:241, b:232}, // 255 241 232 white
            Red => RGB{r:255, g:0, b:77}, // 255   0  77 red
            Orange => RGB{r:255, g:155, b:0}, // 255 155   0 orange
            Yellow => RGB{r:255, g:231, b:39}, // 255 231  39 yellow
            Green => RGB{r:0, g:226, b:50}, // 0 226  50 green
            Blue => RGB{r:41, g:173, b:255}, // 41 173 255 blue
            Indigo => RGB{r:132, g:112, b:169}, // 132 112 169 indigo
            Pink => RGB{r:255, g:119, b:168}, // 255 119 168 pink
            Peach => RGB{r:255, g:214, b:197}, // 255 214 197 peach
            _ => RGB{r:0, g:0, b:0},
        }
    }
    #[inline]
    pub fn to_u8(value: Color) -> u8 {
        use self::Color::*;
        match value {
            Black => 0, // 0   0   0 black
            DarkBlue => 1, // 32  51 123 dark_blue
            DarkPurple => 2, // 126  37  83 dark_purple
            DarkGreen => 3, // 0 144  61 dark_green
            Brown => 4, // 171  82  54 brown
            DarkGray => 5, // 52  54  53 dark_gray
            LightGray => 6, // 194 195 199 light_gray
            White => 7, // 255 241 232 white
            Red => 8, // 255   0  77 red
            Orange => 9, // 255 155   0 orange
            Yellow => 10, // 255 231  39 yellow
            Green => 11, // 0 226  50 green
            Blue => 12, // 41 173 255 blue
            Indigo => 13, // 132 112 169 indigo
            Pink => 14, // 255 119 168 pink
            Peach => 15, // 255 214 197 peach
            _ => 16,
        }
    }
}

pub const SCREEN_EMPTY: ScreenBuffer = [Color::Black; SCREEN_PIXELS];

pub enum PX8State {
    RUN,
    PAUSE,
}

pub struct GfxCursor {
    x: u8,
    y: u8,
}

pub struct Debug {
    last_time: f64,
    frames: u32,
    fps: i32,
}

impl Debug {
    pub fn new() -> Debug {
        Debug {
            last_time: time::precise_time_s(),
            frames: 0,
            fps: 0,
        }
    }

    pub fn update(&mut self) -> i32 {
        let now = time::precise_time_s();
        if now >= self.last_time + 1f64 {
            // info!("{} FPS", self.frames);
            let v = self.frames;

            self.frames = 0;
            self.last_time = now;
            return v as i32;
        } else {
            self.frames += 1;
        }

        return -1;
    }
}

pub enum Code {
    UNKNOWN = 0,
    LUA = 1,
    PYTHON = 2,
}

pub struct Menu {
    idx: u32,
    items: Vec<String>,
}

pub struct Record {
    pub recording: bool,
    pub images: Vec<u8>,
    pub filename: String,
    pub nb: i32,
}

impl Record {
    pub fn new() -> Record {
        let mut images = Vec::new();

        Record {
            recording: false,
            images: images,
            filename: "".to_string(),
            nb: 0,
        }
    }
}

pub struct Px8New {
    pub screen: Arc<Mutex<gfx::Screen>>,
    cursor: GfxCursor,
    pub cartridges: Vec<Cartridge>,
    pub current_cartridge: usize,
    pub lua_plugin: LuaPlugin,
    pub python_plugin: PythonPlugin,
    pub code_type: Code,
    pub state: PX8State,
    pub menu: Menu,
    pub show_info_overlay: bool,
    pub fps: f64,
    pub draw_time: f64,
    pub init_time: f64,
    pub update_time: f64,
    pub record: Record,
    pub draw_return: bool,
    pub update_return: bool,
}


impl Px8New {
    pub fn new() -> Px8New {
        Px8New {
            screen: Arc::new(Mutex::new(gfx::Screen::new())),
            cursor: GfxCursor { x: 0, y: 0 },
            cartridges: Vec::new(),
            current_cartridge: 0,
            lua_plugin: LuaPlugin::new(),
            python_plugin: PythonPlugin::new(),
            code_type: Code::UNKNOWN,
            state: PX8State::RUN,
            menu: Menu{ idx:0, items: Vec::new() },
            show_info_overlay: true,
            fps: 0.0,
            draw_time: 0.0,
            init_time: 0.0,
            update_time: 0.0,
            record: Record::new(),
            draw_return: true,
            update_return: true,
        }
    }

    pub fn init(&mut self) {
        self.screen.lock().unwrap().init();
        self.update_return = true;
        self.draw_return = true;
    }

    pub fn toggle_info_overlay(&mut self) {
        self.show_info_overlay = !self.show_info_overlay;
    }

    pub fn update(&mut self) {
        if self.show_info_overlay {
            self.screen.lock().unwrap().rectfill(0, 0, 108, 8, Color::Black);

            self.screen.lock().unwrap().print(format!("{:.0} FPS {:.2}ms {:.2}ms {:.2}ms",
                                                      self.fps,
                                                      self.init_time,
                                                      self.draw_time,
                                                      self.update_time).to_string(), 0, 0, Color::White);
        }
    }

    pub fn is_end(&mut self) -> bool {
        return !self.update_return;
    }

    pub fn is_recording(&mut self) -> bool {
        return self.record.recording;
    }

    pub fn start_record(&mut self, filename: String) {
        info!("Start to record the frame");

        self.record.recording = true;
        self.record.images.clear();
        self.record.filename = filename;
    }

    pub fn record(&mut self) {
        info!("Recording the frame");

        if self.record.nb % 4 == 0 {
            let mut buffer:Vec<u8> = Vec::new();

            let scale = 2;

            for x in 0..self::SCREEN_WIDTH {
                for y in 0..self::SCREEN_HEIGHT {
                    let value = self.screen.lock().unwrap().pget(x as u32, y as u32);
                    let rgb_value = Color::to_rgb(Color::from_u8(value));

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
        info!("Stop to record the frame {:?}", self.record.images.len());

        self.record.recording = false;

        let mut filedata = File::create(self.record.filename.clone()).unwrap();

        let mut encoder = gif::Encoder::new(&mut filedata, SCREEN_WIDTH as u16, SCREEN_HEIGHT as u16, &[]).unwrap();
        encoder.set(gif::Repeat::Infinite).unwrap();

        let mut idx = 0;
        for i in 0..self.record.images.len() / (SCREEN_WIDTH*SCREEN_HEIGHT*3) {
            info!("Generate frame {:?} {:?}/{:?}", i, self.record.images.len(), idx);

            let mut buffer:Vec<u8> = Vec::new();

            for x in 0..SCREEN_WIDTH {
                for y in 0..SCREEN_HEIGHT {
                    buffer.push(*self.record.images.get(idx).unwrap());
                    buffer.push(*self.record.images.get(idx+1).unwrap());
                    buffer.push(*self.record.images.get(idx+2).unwrap());
                    idx += 3;
                }
            }

            info!("Creating ImageBuffer {:?}", buffer.len());

            let image = image::ImageBuffer::from_raw(SCREEN_WIDTH as u32,
                                                     SCREEN_HEIGHT as u32,
                                                     buffer).unwrap();

            info!("Rotating image");
            let image = image::DynamicImage::ImageRgb8(image).rotate90().resize(
                (SCREEN_WIDTH * scale) as u32,
                (SCREEN_HEIGHT * scale) as u32,
                image::FilterType::Nearest).fliph();

            info!("Creating gif Frame");
            let mut frame = gif::Frame::from_rgb((SCREEN_WIDTH * scale) as u16,
                                             (SCREEN_HEIGHT * scale) as u16,
                                             &mut *image.raw_pixels());

            frame.delay = 1;
            encoder.write_frame(&frame).unwrap();
        }

        info!("GIF created");
    }

    pub fn screenshot(&mut self, filename: String) {
        info!("Taking screenshot");

        let mut buffer:Vec<u8> = Vec::new();

        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                let value = self.screen.lock().unwrap().pget(x as u32, y as u32);
                let rgb_value = Color::to_rgb(Color::from_u8(value));

                buffer.push(rgb_value.r);
                buffer.push(rgb_value.g);
                buffer.push(rgb_value.b);
            }
        }

        let image = image::ImageBuffer::from_raw(SCREEN_WIDTH as u32,
                                                 SCREEN_HEIGHT as u32,
                                                 buffer).unwrap();
        let image = image::DynamicImage::ImageRgb8(image).rotate90().resize(
            (SCREEN_WIDTH * 4) as u32,
            (SCREEN_WIDTH * 4) as u32,
            image::FilterType::Nearest).flipv();

        let mut output = File::create(&Path::new(&filename)).unwrap();
        image.save(&mut output, image::ImageFormat::PNG).unwrap();
    }

    pub fn save_current_cartridge(&mut self, suffix: String) {
        let ref mut cartridge = self.cartridges[self.current_cartridge];

        let output_filename = cartridge.filename.clone() + "-" + &suffix + ".p8";
        info!("Saving the current cartridge in {:?}", output_filename);

        cartridge.gfx.set_sprites(self.screen.lock().unwrap().sprites.clone());

        match cartridge.format {
            CartridgeFormat::P8Format => {
                cartridge.save_in_p8(output_filename);
            },
            CartridgeFormat::PngFormat => {
                cartridge.save_in_p8(output_filename);
            },
            CartridgeFormat::Px8Format => {
                cartridge.save_data();
            }
        }
    }

    pub fn pause(&mut self) {
        self.state = PX8State::PAUSE;

        self.screen.lock().unwrap().rectfill(40, 50, 90, 80, Color::Black);

        self.screen.lock().unwrap().pset(45, 59, Color::White);

        self.screen.lock().unwrap().print("Continue".to_string(), 50, 55, Color::White);
        self.screen.lock().unwrap().print("Quit".to_string(), 50, 65, Color::White);
    }

    pub fn update_pause(&mut self, enter:bool, up:bool, down: bool) {
        self.screen.lock().unwrap().rectfill(40, 50, 90, 80, Color::Black);

        if down {
            self.menu.idx = 1;
        }

        if up {
            self.menu.idx = 0;
        }


        if  self.menu.idx == 0 {
            self.screen.lock().unwrap().pset(45, 59, Color::White);
        }

        if  self.menu.idx == 1 {
            self.screen.lock().unwrap().pset(45, 59+10, Color::White);
        }

        self.screen.lock().unwrap().print("Continue".to_string(), 50, 55, Color::White);
        self.screen.lock().unwrap().print("Quit".to_string(), 50, 65, Color::White);
    }

    pub fn load_cartridge(&mut self,
                          filename: String,
                          tx_input: Sender<Vec<u8>>,
                          rx_output: Receiver<Vec<u8>>,
                          players: Arc<Mutex<Players>>,
                          info: Arc<Mutex<Info>>,
                          editor: bool) {

        let idx = self.cartridges.len();

        info!("IDX CARTRIDGE {:?}", idx);

        if filename.contains(".png") {
            match Cartridge::from_png_file(filename.clone()) {
                Ok(c) => self.cartridges.push(c),
                Err(e) => panic!("Impossible to load the png cartridge"),
            }
        } else if filename.contains(".p8") {
            match Cartridge::from_p8_file(filename.clone()) {
                Ok(c) => self.cartridges.push(c),
                Err(e) => panic!("Impossible to load the p8 cartridge"),
            }
        } else if filename.contains(".py") {
            match Cartridge::from_p8_file(filename.clone()) {
                Ok(c) => self.cartridges.push(c),
                Err(e) => panic!("Impossible to load the p8 cartridge"),
            }
        } else if filename.contains(".px8") {
            match Cartridge::from_px8_file(filename.clone()) {
                Ok(c) => self.cartridges.push(c),
                Err(e) => panic!("Impossible to load the px8 cartridge"),
            }
        } else {
            panic!("Unknown file");
        }


        self.current_cartridge = idx;

        self.screen.lock().unwrap().set_sprites(self.cartridges[idx].gfx.sprites.clone());
        self.screen.lock().unwrap().set_map(self.cartridges[idx].map.map);

        self.load_plugin(idx, tx_input, rx_output, players, info, editor);
    }

    pub fn _get_code_type(&mut self, idx: usize) -> Code {
        if self.cartridges[idx].code.get_name() == "lua" {
            return Code::LUA;
        }

        if self.cartridges[idx].code.get_name() == "python" {
            return Code::PYTHON;
        }

        return Code::UNKNOWN;
    }

    pub fn switch_code(&mut self) {

        let idx = self.current_cartridge;

        let mut data;

        if self.cartridges[idx].edit {

            // Reload the code for the px8 format
            match self.cartridges[idx].format {
                CartridgeFormat::Px8Format => {
                    info!("Reloading code section for the cartridge");
                    self.cartridges[idx].code.reload();
                }
                _ => ()
            }

            data = self.cartridges[idx].code.get_data().clone();
            self.cartridges[idx].edit = false;
            self.code_type = self._get_code_type(idx);
        } else {
            data = self.load_editor("./sys/editor/editor.py".to_string()).clone();
            self.cartridges[idx].edit = true;
            self.code_type = Code::PYTHON;
        }

        println!("CODE -> {:?}", data);

        match self.code_type {
            Code::LUA => {
                self.lua_plugin.load_code(data);
            },
            Code::PYTHON => {
                self.python_plugin.load_code(data);
            },
            Code::UNKNOWN => ()
        }
    }

    pub fn is_editing_current_cartridge(&mut self) -> bool {
        let idx = self.current_cartridge;
        return self.cartridges[idx].edit;
    }

    pub fn load_plugin(&mut self,
                       idx: usize,
                       tx_input: Sender<Vec<u8>>,
                       rx_output: Receiver<Vec<u8>>,
                       players: Arc<Mutex<Players>>,
                       info: Arc<Mutex<Info>>,
                       editor: bool) {
        let mut data;

        info!("START TO LOAD THE PLUGIN");

        let gfx_sprites = self.cartridges[idx].gfx.sprites.clone();
        let gfx_map = self.cartridges[idx].map.map;

        self.code_type = self._get_code_type(idx);

        if editor {
            // Editor mode and original code type is different from Python
            match self.code_type {
                Code::LUA => {
                    info!("Loading LUA Plugin");
                    // load the lua plugin
                    self.lua_plugin.load(tx_input.clone(),
                                         rx_output.clone(),
                                         players.clone(),
                                         info.clone(),
                                         self.screen.clone(),
                                         gfx_sprites.clone(),
                                         gfx_map);
                }
                _ => (),
            }

            data = self.load_editor("./sys/editor/editor.py".to_string()).clone();
            self.cartridges[idx].edit = true;
            self.code_type = Code::PYTHON;
        } else {
            data = self.cartridges[idx].code.get_data().clone();
        }

        println!("CODE -> {:?}", data);

        match self.code_type {
            Code::LUA => {
                info!("Loading LUA Plugin");
                // load the lua plugin
                self.lua_plugin.load(tx_input.clone(),
                                     rx_output.clone(),
                                     players.clone(),
                                     info.clone(),
                                     self.screen.clone(),
                                     gfx_sprites.clone(),
                                     gfx_map);

                println!("{:?}", data);

                // load the lua code in memory
                self.lua_plugin.load_code(data);
            },
            Code::PYTHON => {
                info!("Loading PYTHON Plugin");

                self.python_plugin.load(players.clone(),
                                        info.clone(),
                                        self.screen.clone(),
                                        gfx_sprites.clone(),
                                        gfx_map);

                self.python_plugin.load_code(data);
            },
            Code::UNKNOWN => ()
        }
    }

    pub fn load_editor(&mut self, filename: String) -> String {
        let mut data = "".to_string();

        let f = File::open(filename.clone()).unwrap();
        let mut buf_reader = BufReader::new(f);

        for line in buf_reader.lines() {
            let l = line.unwrap();

            data = data + "\n" + &l;
        }

        return data;
    }

    pub fn call_init(&mut self) -> f64 {
        let current_time = time::now();

        match self.code_type {
            Code::LUA       => self.lua_plugin.init(),
            Code::PYTHON    => self.python_plugin.init(),
            Code::UNKNOWN   => (),
        }

        let diff_time =  time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) - (diff_time.num_seconds() * 1000000000) as f64;
        let elapsed_time = diff_time.num_seconds() as f64 + nanoseconds/1000000000.0;

        return elapsed_time;
    }

    pub fn call_draw(&mut self) -> f64 {
        let current_time = time::now();

        match self.code_type {
            Code::LUA       => self.draw_return = self.lua_plugin.draw(),
            Code::PYTHON    => self.draw_return = self.python_plugin.draw(),
            Code::UNKNOWN   => (),
        }

        let diff_time =  time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) - (diff_time.num_seconds() * 1000000000) as f64;
        let elapsed_time = diff_time.num_seconds() as f64 + nanoseconds/1000000000.0;

        return elapsed_time;
    }

    pub fn call_update(&mut self) -> f64 {
        let current_time = time::now();

        match self.code_type {
            Code::LUA       => self.update_return = self.lua_plugin.update(),
            Code::PYTHON    => self.update_return = self.python_plugin.update(),
            Code::UNKNOWN   => (),
        }

        let diff_time =  time::now() - current_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) - (diff_time.num_seconds() * 1000000000) as f64;
        let elapsed_time = diff_time.num_seconds() as f64 + nanoseconds/1000000000.0;

        return elapsed_time;
    }

}