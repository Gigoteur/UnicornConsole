pub mod edit;
pub mod info;
pub mod cartdata;
pub mod resolution;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fmt;
use log::{error, info, warn};
use std::fs::File;
use std::path::Path;

/* Default value */
pub const DEFAULT_MODE_WIDTH: usize = 128;
pub const DEFAULT_MODE_HEIGHT: usize = 128;

pub const DEFAULT_MAP_WIDTH: usize = 128;
pub const DEFAULT_MAP_HEIGHT: usize = 32;

#[cfg(feature = "image")]
use image;
#[cfg(feature = "image")]
use gif;

// Plugins !
use plugins::lua_plugin::plugin::LuaPlugin;
use plugins::python_plugin::plugin::PythonPlugin;
use plugins::rpython_plugin::plugin::RPythonPlugin;
use plugins::rhai_plugin::plugin::RhaiPlugin;
use plugins::wasm_plugin::plugin::WasmPlugin;

// Internal deps
use gfx;
use contexts;
use cartridge::{Cartridge, CartridgeFormat};
use sound;
use audio;

use audio::sound_rom::Sfx;

use anyhow::{Result};

include!(concat!(env!("OUT_DIR"), "/parameters.rs"));

#[derive(Clone, Debug, PartialEq)]
pub enum UnicornState {
    STOP,
    RUN,
    PAUSE,
}

#[derive(Debug)]
pub enum Code {
    UNKNOWN = 0,
    LUA = 1,
    PYTHON = 2,
    RPYTHON  = 3,
    RHAI = 4,
    WASM = 5,
}

use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum UnicornError {
    PluginError
}

impl Display for UnicornError {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        match self {
            UnicornError::PluginError => write!(
                fmt,
                "An image was passed to Surface with non-exclusive reference"
            ),
        }
    }
}

impl Error for UnicornError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            _ => None,
        }
    }
}


#[cfg_attr(rustfmt, rustfmt_skip)]
pub fn draw_logo(screen: &mut gfx::Screen) {
    let height = screen.height;

    screen
        .print(format!("Powered by Unicorn {:?}.{:?}.{:?}", VERSION, MAJOR_VERSION, MINOR_VERSION).to_string(),
        0,
        (height-16) as i32,
        7);
}

#[derive(Debug)]
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


pub struct UnicornCartridge {
    pub filename: String,
    pub full_filename: String,
    
    pub loaded: bool,

    pub font_name: String,
    
    pub cartridge: Cartridge,
    pub lua_plugin: LuaPlugin,
    pub python_plugin: PythonPlugin,
    pub rpython_plugin: RPythonPlugin,
    pub rhai_plugin: RhaiPlugin,
    pub wasm_plugin: WasmPlugin,
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
            rpython_plugin: RPythonPlugin::new(),
            rhai_plugin: RhaiPlugin::new(),
            wasm_plugin: WasmPlugin::new(),
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
            rpython_plugin: RPythonPlugin::new(),
            rhai_plugin: RhaiPlugin::new(),
            wasm_plugin: WasmPlugin::new(),
        }
    }

    pub fn zero() -> UnicornCartridge {
        UnicornCartridge {
            filename: "".to_string(),
            full_filename: "".to_string(),
            loaded: false,
            font_name: "pico-8".to_string(),
            cartridge: Cartridge::empty(),
            lua_plugin: LuaPlugin::new(),
            python_plugin: PythonPlugin::new(),
            rpython_plugin: RPythonPlugin::new(),
            rhai_plugin: RhaiPlugin::new(),
            wasm_plugin: WasmPlugin::new(),
        }
    }

    pub fn get_code_type(&mut self) -> Code {
        match self.cartridge.code.get_name().as_ref() {
            "lua" => Code::LUA,
            "python" => Code::PYTHON,
            "rpython" => Code::RPYTHON,
            "wasm" => Code::WASM,
            "rhai" => Code::RHAI,
            _ => Code::UNKNOWN,
        }
    }

    pub fn get_code_string_type(&mut self) -> String {
        match self.cartridge.code.get_name().as_ref() {
            "lua" => "lua".into(),
            "python" => "py".into(),
            "rpython" => "py".into(),
            "rhai" => "rhai".into(),
            _ => "".into()
        }
    }

    pub fn get_code(&mut self) -> String {
        self.cartridge.code.get_data().clone()
    }

    pub fn get_bytes_code(&mut self) -> Vec<u8> {
        self.cartridge.code.get_bytes_data().clone()
    }

    pub fn get_palettes(&mut self) -> HashMap<u32, gfx::palette::RGB> {
        self.cartridge.palette.colors.clone()
    }
}

impl fmt::Debug for UnicornCartridge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Unicorn Cartridge {{ cart: {:?} }}",
               self.cartridge)
    }
}

pub enum AudioSyncCommand {
    PressedKey {
        note_index: usize,
        instrument_index: usize,
        channel: usize,
    },
    ReleasedKey {
        channel: usize,
    },
    TriggerNote {
        note_index: usize,
        instrument_index: usize,
    },
    PlayPhrase {
        phrase_index: usize,
        target_bpm: f32,
    },
    StopSfx,
    PlayBgm(usize),
    PlaySfx(Sfx),
    StopBgm,
}
use std::{iter::Cycle, ops::Range};

pub struct AudioCommandBuffer {
    pub vec: Vec<AudioSyncCommand>,
}

impl AudioCommandBuffer {
    pub fn default() -> Self {
        AudioCommandBuffer { vec: Vec::new() }
    }

    pub fn push(&mut self, command: AudioSyncCommand) {
        self.vec.push(command);       
    }
}

pub struct AudioSyncHelper {
    pub sound_engine_data: sound::sound_engine::SoundEngineData,
    channel_ticker: Cycle<Range<usize>>,
    pub command_queue: Arc<Mutex<AudioCommandBuffer>>,
}

impl AudioSyncHelper {
    fn push_commands(&mut self, engine: &mut sound::sound_engine::SoundEngine) {
        let mut current_commands = self.command_queue.lock().unwrap();
        let mut channel_ticker = self.channel_ticker.clone();

        current_commands.vec
        .drain(..)
        .for_each(|command| match command {
            AudioSyncCommand::PressedKey {
                note_index,
                instrument_index,
                channel,
            } => 
            engine.send(sound::sound_engine::SoundEngineChannelType::PianoKeyPressed {
                note_index,
                instrument_index,
                channel,
            }),
            AudioSyncCommand::ReleasedKey { channel } => {
                engine.send(sound::sound_engine::SoundEngineChannelType::PianoKeyReleased { channel })
            }
            AudioSyncCommand::TriggerNote {
                note_index,
                instrument_index,
            } => engine.send(sound::sound_engine::SoundEngineChannelType::TriggerNote {
                note_index,
                instrument_index,
                channel: channel_ticker.next().unwrap(),
            }),
            AudioSyncCommand::PlayPhrase {
                phrase_index,
                target_bpm,
            } => engine.send(sound::sound_engine::SoundEngineChannelType::PlayPhrase {
                phrase_index,
                target_bpm,
            }),
            AudioSyncCommand::PlaySfx(sfx) => engine.send(sound::sound_engine::SoundEngineChannelType::PlaySfx(sfx)),
            AudioSyncCommand::StopSfx => engine.send(sound::sound_engine::SoundEngineChannelType::StopSfx),
            AudioSyncCommand::PlayBgm(song) => {
                engine.send(sound::sound_engine::SoundEngineChannelType::PlayBgm(song))
            }
            AudioSyncCommand::StopBgm => engine.send(sound::sound_engine::SoundEngineChannelType::StopBgm),
        });
    }
}


pub struct Unicorn {
    pub screen: Arc<Mutex<gfx::Screen>>,
    pub contexts: Arc<Mutex<contexts::Contexts>>,

    pub sound_engine: Option<sound::sound_engine::SoundEngine>,
    pub audio_sync_helper: Option<AudioSyncHelper>,
    pub audio_command_buffer: Arc<Mutex<AudioCommandBuffer>>,
    pub sound_rom: audio::sound_rom::SoundRom,

    pub info: Arc<Mutex<info::Info>>,

    pub debug: bool,
    pub cartridge: UnicornCartridge,
    pub state: UnicornState,
    pub fps: f64,
    pub record: Record,
    pub version: u32,
    pub major_version: u32,
    pub minor_version: u32,
    pub frame_rate: gfx::framerate::FrameRate,
}

impl Unicorn {
    pub fn new() -> Unicorn {
        info!("[Unicorn] Creating new Unicorn");

        let screen = Arc::new(Mutex::new(gfx::Screen::new(DEFAULT_MODE_WIDTH, DEFAULT_MODE_HEIGHT, DEFAULT_MAP_WIDTH, DEFAULT_MAP_HEIGHT)));
        
        Unicorn {
            screen: screen.clone(),
            contexts: Arc::new(Mutex::new(contexts::Contexts::new(2))),

            sound_engine: None,
            audio_sync_helper: None,
            audio_command_buffer: Arc::new(Mutex::new(AudioCommandBuffer::default())),
            sound_rom: audio::sound_rom::SoundRom::default(),

            info: Arc::new(Mutex::new(info::Info::new())),
            debug: false,
            
            cartridge: UnicornCartridge::zero(),
           
            state: UnicornState::STOP,
            
            fps: 0.0,
            record: Record::new(),

            version: VERSION,
            major_version: MAJOR_VERSION,
            minor_version: MINOR_VERSION,

            frame_rate: gfx::framerate::FrameRate::default(),
        }
    }

    pub fn resize_buffer(&mut self, width: usize, height: usize) {
        self.screen.lock().unwrap().resize_buffer(width, height);
    }

    pub fn width(&mut self) -> u32 {
        self.screen.lock().unwrap().mode_width() as u32
    }

    pub fn height(&mut self) -> u32 {
        self.screen.lock().unwrap().mode_height() as u32
    }
    
    pub fn is_none(&mut self) -> bool {
        self.state == UnicornState::STOP
    }

    pub fn setup(&mut self) {
        info!("[Unicorn] Setup");
        self.reset();
    }

    pub fn toggle_debug(&mut self) {
        info!("[Unicorn] Toggle Debug {:?}", self.debug);
        self.debug = !self.debug;
    }

    pub fn init_sound(&mut self) {
        let sound_rom_instance = Arc::new(sound::sound_rom_instance::SoundRomInstance::new(&self.sound_rom));

        let sound_engine = sound::sound_engine::SoundEngine::new(60, &sound_rom_instance, 64);
        let sound_engine_data = sound::sound_engine::SoundEngineData::new(sound_engine.output_sample_rate(), &sound_rom_instance);
        
        self.sound_engine = Some(sound_engine);
      //  self.sound_engine_data = Some(sound_engine_data);

        self.audio_sync_helper = Some(AudioSyncHelper {
            sound_engine_data,
            channel_ticker: (0..audio::consts::SFX_CHANNELS).cycle(),
            command_queue: Arc::new(Mutex::new(AudioCommandBuffer::default())),
        });
    }

    pub fn sync_audio(&mut self) {
        self.audio_sync_helper.as_mut().unwrap().push_commands(self.sound_engine.as_mut().unwrap());
    }

    pub fn reset(&mut self) {
        info!("[Unicorn] Reset");
        self.cartridge = UnicornCartridge::zero();
        self.state = UnicornState::STOP;
        self.debug = false;
        self.screen.lock().unwrap().reset();
    }

    pub fn debug_draw(&mut self) {
        if self.debug {
            let screen = &mut self.screen.lock().unwrap();
            let mouse_x = self.contexts.lock().unwrap().input_context.btn_mouse(0, 0);
            let mouse_y = self.contexts.lock().unwrap().input_context.btn_mouse(0, 1);

            let width = screen.width as i32;
            
            let palette_name = screen.current_palette_name.clone();

            screen.rectfill(0, 0, width, 8, 0);
            screen.force_print(format!("{:.0}FPS {:.2?} {:.2?} {:?} {:?}",
                                       self.fps,
                                       mouse_x,
                                       mouse_y,
                                       palette_name,
                                       self.state)
                                       .to_string(),
                               0,
                               0,
                               7);
                               
        }
    }

    pub fn reload(&mut self, filename: String) {
        info!("Reload the cartridge !");
        self.load_cartridge(filename);
        self.init();
    }

    pub fn init(&mut self) {
        self.call_init();
    }

    pub fn update(&mut self) -> bool {
        self.sync_audio();

        match self.state {
            UnicornState::STOP => {

            }
            UnicornState::PAUSE => {

            }
            UnicornState::RUN => {
                self.call_update();
            }
        }
        true
    }

    pub fn draw(&mut self) {
        match self.state {
            UnicornState::STOP => {
            }
            UnicornState::PAUSE => {
            }
            UnicornState::RUN => {
                self.call_draw();
            }
        }

        self.debug_draw();

        #[cfg(feature = "image")]
        if self.is_recording() {
            self.record();
        }
    }

    pub fn is_recording(&self) -> bool {
        self.record.recording
    }

    #[cfg(not(feature = "image"))]
    pub fn start_record(&mut self, filename: &str) {
        warn!("[Unicorn][Record] Record feature not enable (image feature)");
        self.record.recording = false;
    }

    #[cfg(feature = "image")]
    pub fn start_record(&mut self, filename: &str) {
        info!("[Unicorn][Record] Start to record the frame");

        if self.is_recording() {
            self.stop_record();
        } else {
            self.record.recording = true;
            self.record.images.clear();
            self.record.filename = filename.to_string();
        }
    }

    #[cfg(feature = "image")]
    pub fn record(&mut self) {
        info!("[Unicorn][Record] Recording the frame {:?}", self.record.images.len());

        if self.record.nb % 2 == 0 {
            let mut buffer: Vec<u8> = Vec::new();
            let screen = &mut self.screen.lock().unwrap();

            for x in 0..screen.width {
                for y in 0..screen.height {
                    let rgb_value = screen.pget_rgba(x as u32, y as u32);

                    buffer.push(rgb_value.r);
                    buffer.push(rgb_value.g);
                    buffer.push(rgb_value.b);
                    buffer.push(rgb_value.a);
                }
            }
            self.record.images.append(&mut buffer);
       }

       self.record.nb += 1;
    }

    #[cfg(not(feature = "image"))]
    pub fn stop_record(&mut self) -> Result<()> {
        warn!("[Unicorn][Record] Record feature not enable (image feature)");
        Ok(())
    }

    #[cfg(feature = "image")]
    pub fn stop_record(&mut self) -> Result<()> {
        
        info!("[Unicorn][Record] Stop to record the frame {:?}",
              self.record.images.len());

        let screen = &self.screen.lock().unwrap();

        self.record.recording = false;

        let mut file_out = File::create(&self.record.filename).unwrap();
        let mut encoder = image::codecs::gif::GifEncoder::new_with_speed(file_out, 30);
    
        encoder.set_repeat(image::codecs::gif::Repeat::Infinite).unwrap();

        let mut idx = 0;
        for i in 0..self.record.images.len() / (screen.width * screen.height * 4) {
            info!("[Unicorn][Record] Generate frame {:?} {:?}/{:?}",
                  i,
                  idx,
                  self.record.images.len());

            let mut buffer: Vec<u8> = Vec::new();

            for _ in 0..screen.width {
                for _ in 0..screen.height {
                    buffer.push(self.record.images[idx]);
                    buffer.push(self.record.images[idx + 1]);
                    buffer.push(self.record.images[idx + 2]);
                    buffer.push(self.record.images[idx + 3]);
                    idx += 4;
                }
            }

            info!("[Unicorn][Record] Creating ImageBuffer {:?}", buffer.len());

            let image =
                image::ImageBuffer::from_raw(screen.height as u32, screen.width as u32, buffer)
                    .unwrap();

            info!("[Unicorn][Record] Rotating image");
            let image = image::imageops::rotate270(&image);
            let image = image::imageops::flip_vertical(&image);
            

            info!("[Unicorn][Record] Creating gif Frame");
            let mut frame = image::Frame::new(image);
            encoder.encode_frame(frame).unwrap();
        }

        info!("[Unicorn][Record] GIF created in {:?}", self.record.filename);
        Ok(())
    }

    #[cfg(not(feature = "image"))]
    pub fn screenshot(&mut self, filename: &str) {
        warn!("[Unicorn] Screenshot feature not enable (image feature)")
    }

    #[cfg(feature = "image")]
    pub fn screenshot(&mut self, filename: &str) {
        let screen = &mut self.screen.lock().unwrap();

        info!("[Unicorn] Taking screenshot {:?}x{:?} in {:?}", screen.width, screen.height, filename);

        let mut buffer: Vec<u8> = vec![0; (screen.width*screen.height) * 4];

        let mut idx = 0;
        for x in 0..screen.width {
            for y in 0..screen.height {
                let rgb_value = screen.pget_rgba(x as u32, y as u32);

                buffer[idx] = rgb_value.r;
                buffer[idx + 1] = rgb_value.g;
                buffer[idx + 2] = rgb_value.b;
                buffer[idx + 3] = rgb_value.a;
                
                idx += 4;
            }
        }

        let image = image::ImageBuffer::from_raw(screen.height as u32, screen.width as u32, buffer)
            .unwrap();
        let image = image::DynamicImage::ImageRgba8(image)
            .rotate270()
            .flipv();

        image.save_with_format(Path::new(filename), image::ImageFormat::Png).unwrap();
    }

    pub fn set_code(&mut self, code: String) {
        self.cartridge.cartridge.code.set_data(code);
    }

    pub fn save_current_cartridge(&mut self) {
        let screen = &self.screen.lock().unwrap();


        let output_filename = self.cartridge.filename.clone();
        info!("[Unicorn][SAVE] Saving the current cartridge in {:?}",
              output_filename);

        info!("[Unicorn][SAVE] Set the new sprites");
        self.cartridge.cartridge.gfx.set_sprites(screen.sprites.clone());
        info!("[Unicorn][SAVE] Set the new map");
        self.cartridge.cartridge.map.set_map(screen.map.clone());
        info!("[Unicorn][SAVE] Set the new flags");
        self.cartridge.cartridge.gff.set_flags(screen.sprites.clone());
        //info!("[Unicorn][SAVE] Set the new palette");
        //screen.palette.set_colors(screen.palettes.get_colors());

        match self.cartridge.cartridge.format {
            CartridgeFormat::UnicornFormat => {
                self.cartridge.cartridge.save_in_unicorn(&output_filename,
                                                format!("{:?}.{:?}.{:?}",
                                                        self.version,
                                                        self.major_version,
                                                        self.minor_version)
                                                        .as_str());
            }
            _ => {}
        }
    }

    pub fn switch_pause(&mut self) {
        info!("[Unicorn] Switch pause");

        let screen = &mut self.screen.lock().unwrap();

        match self.state {
            UnicornState::STOP => {

            }
            UnicornState::PAUSE => {
                self.state = UnicornState::RUN;
                /* Restore previous state */
                screen.restore();
                screen.font(&self.cartridge.font_name.clone());
            }
            UnicornState::RUN => {
                /* Save state */
                screen.save();
                self.cartridge.font_name = screen.get_font();

                screen.font("pico-8");
                self.state = UnicornState::PAUSE;
            }
        }
        info!("[Unicorn] End Switch pause");
    }

    pub fn _setup_screen(&mut self) {
        info!("[Unicorn] Setup screen");

        info!("[Unicorn] Copying sprites ...");
        self.screen
            .lock()
            .unwrap()
            .set_sprites(self.cartridge.cartridge.gfx.sprites.clone());

        info!("[Unicorn] Copying gff flags ...");
        self.screen
            .lock()
            .unwrap()
            .set_sprites_flags(self.cartridge.cartridge.gff.flags.clone());

        info!("[Unicorn] Copying map ...");
        self.screen
            .lock()
            .unwrap()
            .set_map(self.cartridge.cartridge.map.map.clone());

        info!("[Unicorn] Copying palette ...");
        self.screen.lock().unwrap().set_palette_colors(self.cartridge.cartridge.palette.colors.clone());
    }

    pub fn _load_cartridge(&mut self)
                           -> Result<()> {
        info!("[Unicorn] Loading cartridge");

        let data = self.cartridge.get_code();
        let data_bytes = self.cartridge.get_bytes_code();

        match self.cartridge.get_code_type() {
            Code::LUA => {
                info!("[Unicorn] Loading LUA Plugin");

                self.cartridge
                        .lua_plugin
                        .load(self.contexts.clone(),
                            self.info.clone(),
                            self.screen.clone(),
                            self.audio_sync_helper.as_mut().unwrap().command_queue.clone())?;

                self.cartridge.lua_plugin.load_code(data.clone())?;
            }
            Code::RPYTHON => {
                info!("[Unicorn] Loading RPYTHON Plugin");

                self.cartridge
                    .rpython_plugin
                    .load(self.contexts.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.audio_sync_helper.as_mut().unwrap().command_queue.clone())?;

                self.cartridge.rpython_plugin.load_code(data.clone())?;
            }
            Code::PYTHON => {
                info!("[Unicorn] Loading PYTHON Plugin");

                self.cartridge
                    .python_plugin
                    .load(self.contexts.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.audio_sync_helper.as_mut().unwrap().command_queue.clone())?;

                self.cartridge.python_plugin.load_code(data.clone())?;
            }
            Code::RHAI => {
                info!("[Unicorn] Loading RHAI Plugin");

                self.cartridge
                    .rhai_plugin
                    .load(self.contexts.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.audio_sync_helper.as_mut().unwrap().command_queue.clone())?;

                self.cartridge.rhai_plugin.load_code(data.clone())?;
            }
            Code::WASM => {
                info!("[Unicorn] Loading WASM Plugin");

                self.cartridge
                    .wasm_plugin
                    .load(self.contexts.clone(),
                          self.info.clone(),
                          self.screen.clone(),
                          self.audio_sync_helper.as_mut().unwrap().command_queue.clone())?;

                self.cartridge.wasm_plugin.load_code(&data_bytes)?;
            }
            _ => (),
        }

        info!("[Unicorn] CARTRIDGE LOADED SUCCESSFULLY");

        Ok(())
    }

    pub fn load_cartridge(&mut self, filename: String) -> bool {
        info!("[Unicorn] Load cartridge from {:?}", filename);

        self.reset();
        self.init_sound();


        let cartridge;
        if filename.contains(".corn") {
            match Cartridge::from_unicorn_file(filename.as_str()) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the unicorn cartridge {:?}", e),
            }
        } else if filename.contains(".acorn") {
            match Cartridge::from_dunicorn_file(filename.as_str()) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the dUnicorn cartridge {:?}", e),
            }
        } else if filename.contains(".png") {
            match Cartridge::from_png_file(filename.as_str()) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the Pico8 PNG cartridge {:?}", e),
            }
        } else if filename.contains(".p8") {
            match Cartridge::from_p8_file(filename.as_str()) {
                Ok(c) => cartridge = c,
                Err(e) => panic!("[Unicorn] Impossible to load the Pico8 P8 cartridge {:?}", e),
            }
        } else {
            panic!("[Unicorn] Unknown file format !");
        }

        self.cartridge = UnicornCartridge::new(cartridge, filename);
        self.resize_buffer(self.cartridge.cartridge.mode_width, self.cartridge.cartridge.mode_height);
        self._setup_screen();

        match self._load_cartridge() {
            Ok(()) => self.state = UnicornState::RUN,
            Err(err) => error!("[Unicorn] [Impossible to load the cartridge]: {}", err),
        }

        true
    }

    pub fn call_init(&mut self) {
        info!("[Unicorn] CALL INIT {:?} {:?}", self.cartridge.get_code_type(), self.state);

        if self.state == UnicornState::RUN {
            match self.cartridge.get_code_type() {
                Code::LUA => match self.cartridge.lua_plugin.init() {
                    _ => (),
                }
                Code::RPYTHON => match self.cartridge.rpython_plugin.init() {
                    _ => (),
                }
                Code::RHAI => match self.cartridge.rhai_plugin.init() {
                    _ => (),
                }
                Code::PYTHON => match self.cartridge.python_plugin.init() {
                    _ => (),
                }
                Code::WASM =>  match self.cartridge.wasm_plugin.init() {
                    _ => (),
                }

                _ => error!("[Unicorn] Impossible to match a plugin"),
            }
        }
    }

    pub fn call_draw(&mut self) {
        if self.state == UnicornState::RUN {
            match self.cartridge.get_code_type()  {
                Code::LUA => {
                    match self.cartridge.lua_plugin.draw() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_draw / lua]: {}", err),
                    }
                }
                Code::RPYTHON => {
                    match self.cartridge.rpython_plugin.draw() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_draw / rpython]: {}", err),
                    }
                }
                Code::RHAI => {
                    match self.cartridge.rhai_plugin.draw() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_draw / rhai]: {}", err),
                    }
                }
                Code::PYTHON => {
                    match self.cartridge.python_plugin.draw() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_draw / python]: {}", err),
                    }
                }
                Code::WASM => {
                    match self.cartridge.wasm_plugin.draw() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_draw / wasm]: {}", err),
                    }
                }

                _ => (),
            }
        }
    }

    pub fn call_update(&mut self) {
        if self.state == UnicornState::RUN {
            match self.cartridge.get_code_type() {
                Code::LUA => {
                    match self.cartridge.lua_plugin.update() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_update / lua]: {}", err),
                    }
                }
                Code::RPYTHON => {
                    match self.cartridge.rpython_plugin.update() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_update / rpython]: {}", err),
                    }
                }
                Code::RHAI => {
                    match self.cartridge.rhai_plugin.update() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_update / rhai]: {}", err),
                    }
                }
                Code::PYTHON => {
                    match self.cartridge.python_plugin.update() {
                            Ok(()) => (),
                            Err(err) => error!("[Unicorn] [call_update / python]: {}", err),
                        }
                }
                Code::WASM => {
                    match self.cartridge.wasm_plugin.update() {
                        Ok(()) => (),
                        Err(err) => error!("[Unicorn] [call_update / wasm]: {}", err),
                    }
                }
                _ => (),
            }
        }
    }
}
