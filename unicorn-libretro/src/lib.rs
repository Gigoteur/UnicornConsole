extern crate unicorn;

#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

#[macro_use]
extern crate libretro_backend;

use libretro_backend::{CoreInfo, AudioVideoInfo, PixelFormat, GameData, LoadGameResult, Region,
                       RuntimeHandle, JoypadButton};

use std::slice;
use std::mem;

struct UnicornCore {
    uc: unicorn::unicorn::Unicorn,
    framebuffer: [u32; 400 * 240],
    audio_buffer: Vec<i16>,
    mouse_x: i16,
    mouse_y: i16,
    game_data: Option<GameData>,
}

#[inline]
pub fn as_bytes<T: Copy>(array: &[T]) -> &[u8] {
    unsafe {
        slice::from_raw_parts(mem::transmute(array.as_ptr()),
                              mem::size_of::<T>() * array.len())
    }
}

pub fn map_libretrocancode(button: JoypadButton) -> unicorn::config::scancode::Scancode {
    match button {
        JoypadButton::A => unicorn::config::scancode::Scancode::Z,
        JoypadButton::B => unicorn::config::scancode::Scancode::X,
        JoypadButton::Left => unicorn::config::scancode::Scancode::Left,
        JoypadButton::Right => unicorn::config::scancode::Scancode::Right,
        JoypadButton::Up => unicorn::config::scancode::Scancode::Up,
        JoypadButton::Down => unicorn::config::scancode::Scancode::Down,
        _ => unicorn::config::scancode::Scancode::NONE,
    }
}

impl UnicornCore {
    fn new() -> UnicornCore {
        UnicornCore {
            uc: unicorn::unicorn::Unicorn::new(),
            framebuffer: [0; 400 * 240],
            audio_buffer: Vec::with_capacity(44100),
            mouse_x: 0,
            mouse_y: 0,
            game_data: None,
        }
    }
}

impl Default for UnicornCore {
    fn default() -> Self {

        let logger_config = fern::DispatchConfig {
            format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
                format!("[{}][{}] {}",
                        time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(),
                        level,
                        msg)
            }),
            output: vec![fern::OutputConfig::stdout()],
            level: log::LogLevelFilter::Trace,
        };

        if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Info) {
            panic!("Failed to initialize global logger: {}", e);
        }

        Self::new()
    }
}


impl libretro_backend::Core for UnicornCore {
    fn info() -> CoreInfo {
        CoreInfo::new("Unicorn", env!("CARGO_PKG_VERSION")).supports_roms_with_extension("uni")
    }

    fn on_load_game(&mut self, game_data: GameData) -> LoadGameResult {
        info!("[LIBRETRO][ON_LOAD_GAME]");

        self.uc.setup();

        if game_data.is_empty() {
            return LoadGameResult::Failed(game_data);
        }

        // Memory
        let result = if let Some(data) = game_data.data() {
            let data_final: Vec<u8> = unicorn::unicorn::array_to_vec(data);
            self.uc.load_cartridge_raw("test.uni", data_final, false)
            // Path
        } else if let Some(path) = game_data.path() {
            self.uc.load_cartridge(path, path, false)
        } else {
            unreachable!();
        };

        match result {
            true => {
                self.game_data = Some(game_data);
                let av_info = AudioVideoInfo::new()
                    .video( 400, 240, 60.0, PixelFormat::ARGB8888 )
                    //.audio( 44100.0 )
                    .region( Region::NTSC );

                self.uc.init();

                LoadGameResult::Success(av_info)
            }
            false => LoadGameResult::Failed(game_data),
        }
    }

    fn on_unload_game(&mut self) -> GameData {
        info!("[LIBRETRO][ON_UNLOAD_GAME]");

        self.game_data.take().unwrap()
    }

    fn on_run(&mut self, handle: &mut RuntimeHandle) {
        debug!("[LIBRETRO][ON_RUN]");

        macro_rules! update_controllers {
            ( $( $button:ident ),+ ) => (
                $(
                    if handle.is_joypad_button_pressed( 0, JoypadButton::$button ) {
                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .key_down(unicorn::config::scancode::Mod::NONE,
                                      map_libretrocancode(JoypadButton::$button),
                                      false,
                                      self.uc.info.lock().unwrap().elapsed_time);

                    } else {
                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .key_up(unicorn::config::scancode::Mod::NONE,
                                    map_libretrocancode(JoypadButton::$button));
                    }
                 )+
            )
        }

        update_controllers!(A, B, Start, Select, Left, Up, Right, Down);

        self.mouse_x += handle.mouse_x();
        self.mouse_y += handle.mouse_y();

        debug!("MOUSE {:?} {:?}", self.mouse_x, self.mouse_y);

        self.uc
            .players
            .lock()
            .unwrap()
            .set_mouse_x(self.mouse_x as i32);

        self.uc
            .players
            .lock()
            .unwrap()
            .set_mouse_y(self.mouse_y as i32);

        self.uc.update();
        self.uc.draw();
        self.uc.update_sound();

        self.uc.info.lock().unwrap().update();
        self.uc
            .players
            .lock()
            .unwrap()
            .update(self.uc.info.lock().unwrap().elapsed_time);

        let mut palette = unicorn::unicorn::PALETTE.lock().unwrap();
        let framebuffer = &self.uc.screen.lock().unwrap().frame_buffer;

        for (pixel_in, pixel_out) in framebuffer.iter().zip(self.framebuffer.iter_mut()) {
            let rgb = palette.get_rgb(*pixel_in as u32);
            *pixel_out = ((rgb.r as u32) << 16) | ((rgb.g as u32) << 8) | ((rgb.b as u32));
        }

        let video_frame = as_bytes(&self.framebuffer[..]);
        handle.upload_video_frame(video_frame);

        //   self.audio_buffer.push( 0 );
        //   self.audio_buffer.push( 0 );


        //   handle.upload_audio_frame( &self.audio_buffer[..] );
        //   self.audio_buffer.clear();
    }

    fn on_reset(&mut self) {
        info!("[LIBRETRO][RESET]");
    }
}


libretro_core!(UnicornCore);