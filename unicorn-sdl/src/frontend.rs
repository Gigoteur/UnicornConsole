use sdl2;
use sdl2::Sdl;
use sdl2::EventPump;
use std::time::Duration;
use sdl2::event::{Event, WindowEvent};

use std::path::Path;

use chrono::prelude::*;
use std::time::Instant;

use sdl2::controller::Axis;
use sdl2::keyboard::Scancode;
use sdl2::mouse::MouseButton;

use renderer;
use fps;
use frametimes;
use controllers;
use input::{map_axis, map_button, map_button_joystick, map_axis_joystick};

#[cfg(target_os = "emscripten")]
use emscripten;

use unicorn;
use unicorn::gfx::Scale;
use unicorn::config::scancode;


pub fn map_sdlscancode(code: Scancode) -> scancode::Scancode {
    match code {
        Scancode::A => scancode::Scancode::A,
        Scancode::B => scancode::Scancode::B,
        Scancode::C => scancode::Scancode::C,
        Scancode::D => scancode::Scancode::D,
        Scancode::E => scancode::Scancode::E,
        Scancode::F => scancode::Scancode::F,
        Scancode::G => scancode::Scancode::G,
        Scancode::H => scancode::Scancode::H,
        Scancode::I => scancode::Scancode::I,
        Scancode::J => scancode::Scancode::J,
        Scancode::K => scancode::Scancode::K,
        Scancode::L => scancode::Scancode::L,
        Scancode::M => scancode::Scancode::M,
        Scancode::N => scancode::Scancode::N,
        Scancode::O => scancode::Scancode::O,
        Scancode::P => scancode::Scancode::P,
        Scancode::Q => scancode::Scancode::Q,
        Scancode::R => scancode::Scancode::R,
        Scancode::S => scancode::Scancode::S,
        Scancode::T => scancode::Scancode::T,
        Scancode::U => scancode::Scancode::U,
        Scancode::V => scancode::Scancode::V,
        Scancode::W => scancode::Scancode::W,
        Scancode::X => scancode::Scancode::X,
        Scancode::Y => scancode::Scancode::Y,
        Scancode::Z => scancode::Scancode::Z,
        Scancode::Num1 => scancode::Scancode::Num1,
        Scancode::Num2 => scancode::Scancode::Num2,
        Scancode::Num3 => scancode::Scancode::Num3,
        Scancode::Num4 => scancode::Scancode::Num4,
        Scancode::Num5 => scancode::Scancode::Num5,
        Scancode::Num6 => scancode::Scancode::Num6,
        Scancode::Num7 => scancode::Scancode::Num7,
        Scancode::Num8 => scancode::Scancode::Num8,
        Scancode::Num9 => scancode::Scancode::Num9,
        Scancode::Num0 => scancode::Scancode::Num0,
        Scancode::Return => scancode::Scancode::Return,
        Scancode::Escape => scancode::Scancode::Escape,
        Scancode::Backspace => scancode::Scancode::Backspace,
        Scancode::Tab => scancode::Scancode::Tab,
        Scancode::Space => scancode::Scancode::Space,
        Scancode::Minus => scancode::Scancode::Minus,
        Scancode::Equals => scancode::Scancode::Equals,
        Scancode::LeftBracket => scancode::Scancode::LeftBracket,
        Scancode::RightBracket => scancode::Scancode::RightBracket,
        Scancode::Backslash => scancode::Scancode::Backslash,
        Scancode::NonUsHash => scancode::Scancode::NonUsHash,
        Scancode::Semicolon => scancode::Scancode::Semicolon,
        Scancode::Apostrophe => scancode::Scancode::Apostrophe,
        Scancode::Grave => scancode::Scancode::Grave,
        Scancode::Comma => scancode::Scancode::Comma,
        Scancode::Period => scancode::Scancode::Period,
        Scancode::Slash => scancode::Scancode::Slash,
        Scancode::CapsLock => scancode::Scancode::CapsLock,
        Scancode::F1 => scancode::Scancode::F1,
        Scancode::F2 => scancode::Scancode::F2,
        Scancode::F3 => scancode::Scancode::F3,
        Scancode::F4 => scancode::Scancode::F4,
        Scancode::F5 => scancode::Scancode::F5,
        Scancode::F6 => scancode::Scancode::F6,
        Scancode::F7 => scancode::Scancode::F7,
        Scancode::F8 => scancode::Scancode::F8,
        Scancode::F9 => scancode::Scancode::F9,
        Scancode::F10 => scancode::Scancode::F10,
        Scancode::F11 => scancode::Scancode::F11,
        Scancode::F12 => scancode::Scancode::F12,
        Scancode::PrintScreen => scancode::Scancode::PrintScreen,
        Scancode::ScrollLock => scancode::Scancode::ScrollLock,
        Scancode::Pause => scancode::Scancode::Pause,
        Scancode::Insert => scancode::Scancode::Insert,
        Scancode::Home => scancode::Scancode::Home,
        Scancode::PageUp => scancode::Scancode::PageUp,
        Scancode::Delete => scancode::Scancode::Delete,
        Scancode::End => scancode::Scancode::End,
        Scancode::PageDown => scancode::Scancode::PageDown,
        Scancode::Right => scancode::Scancode::Right,
        Scancode::Left => scancode::Scancode::Left,
        Scancode::Down => scancode::Scancode::Down,
        Scancode::Up => scancode::Scancode::Up,
        Scancode::NumLockClear => scancode::Scancode::NumLockClear,
        Scancode::KpDivide => scancode::Scancode::KpDivide,
        Scancode::KpMultiply => scancode::Scancode::KpMultiply,
        Scancode::KpMinus => scancode::Scancode::KpMinus,
        Scancode::KpPlus => scancode::Scancode::KpPlus,
        Scancode::KpEnter => scancode::Scancode::KpEnter,
        Scancode::Kp1 => scancode::Scancode::Kp1,
        Scancode::Kp2 => scancode::Scancode::Kp2,
        Scancode::Kp3 => scancode::Scancode::Kp3,
        Scancode::Kp4 => scancode::Scancode::Kp4,
        Scancode::Kp5 => scancode::Scancode::Kp5,
        Scancode::Kp6 => scancode::Scancode::Kp6,
        Scancode::Kp7 => scancode::Scancode::Kp7,
        Scancode::Kp8 => scancode::Scancode::Kp8,
        Scancode::Kp9 => scancode::Scancode::Kp9,
        Scancode::Kp0 => scancode::Scancode::Kp0,
        Scancode::KpPeriod => scancode::Scancode::KpPeriod,
        Scancode::NonUsBackslash => scancode::Scancode::NonUsBackslash,
        Scancode::Application => scancode::Scancode::Application,
        Scancode::Power => scancode::Scancode::Power,
        Scancode::KpEquals => scancode::Scancode::KpEquals,
        Scancode::F13 => scancode::Scancode::F13,
        Scancode::F14 => scancode::Scancode::F14,
        Scancode::F15 => scancode::Scancode::F15,
        Scancode::F16 => scancode::Scancode::F16,
        Scancode::F17 => scancode::Scancode::F17,
        Scancode::F18 => scancode::Scancode::F18,
        Scancode::F19 => scancode::Scancode::F19,
        Scancode::F20 => scancode::Scancode::F20,
        Scancode::F21 => scancode::Scancode::F21,
        Scancode::F22 => scancode::Scancode::F22,
        Scancode::F23 => scancode::Scancode::F23,
        Scancode::F24 => scancode::Scancode::F24,
        Scancode::Execute => scancode::Scancode::Execute,
        Scancode::Help => scancode::Scancode::Help,
        Scancode::Menu => scancode::Scancode::Menu,
        Scancode::Select => scancode::Scancode::Select,
        Scancode::Stop => scancode::Scancode::Stop,
        Scancode::Again => scancode::Scancode::Again,
        Scancode::Undo => scancode::Scancode::Undo,
        Scancode::Cut => scancode::Scancode::Cut,
        Scancode::Copy => scancode::Scancode::Copy,
        Scancode::Paste => scancode::Scancode::Paste,
        Scancode::Find => scancode::Scancode::Find,
        Scancode::Mute => scancode::Scancode::Mute,
        Scancode::VolumeUp => scancode::Scancode::VolumeUp,
        Scancode::VolumeDown => scancode::Scancode::VolumeDown,
        Scancode::KpComma => scancode::Scancode::KpComma,
        Scancode::KpEqualsAS400 => scancode::Scancode::KpEqualsAS400,
        Scancode::International1 => scancode::Scancode::International1,
        Scancode::International2 => scancode::Scancode::International2,
        Scancode::International3 => scancode::Scancode::International3,
        Scancode::International4 => scancode::Scancode::International4,
        Scancode::International5 => scancode::Scancode::International5,
        Scancode::International6 => scancode::Scancode::International6,
        Scancode::International7 => scancode::Scancode::International7,
        Scancode::International8 => scancode::Scancode::International8,
        Scancode::International9 => scancode::Scancode::International9,
        Scancode::Lang1 => scancode::Scancode::Lang1,
        Scancode::Lang2 => scancode::Scancode::Lang2,
        Scancode::Lang3 => scancode::Scancode::Lang3,
        Scancode::Lang4 => scancode::Scancode::Lang4,
        Scancode::Lang5 => scancode::Scancode::Lang5,
        Scancode::Lang6 => scancode::Scancode::Lang6,
        Scancode::Lang7 => scancode::Scancode::Lang7,
        Scancode::Lang8 => scancode::Scancode::Lang8,
        Scancode::Lang9 => scancode::Scancode::Lang9,
        Scancode::AltErase => scancode::Scancode::AltErase,
        Scancode::SysReq => scancode::Scancode::SysReq,
        Scancode::Cancel => scancode::Scancode::Cancel,
        Scancode::Clear => scancode::Scancode::Clear,
        Scancode::Prior => scancode::Scancode::Prior,
        Scancode::Return2 => scancode::Scancode::Return2,
        Scancode::Separator => scancode::Scancode::Separator,
        Scancode::Out => scancode::Scancode::Out,
        Scancode::Oper => scancode::Scancode::Oper,
        Scancode::ClearAgain => scancode::Scancode::ClearAgain,
        Scancode::CrSel => scancode::Scancode::CrSel,
        Scancode::ExSel => scancode::Scancode::ExSel,
        Scancode::Kp00 => scancode::Scancode::Kp00,
        Scancode::Kp000 => scancode::Scancode::Kp000,
        Scancode::ThousandsSeparator => scancode::Scancode::ThousandsSeparator,
        Scancode::DecimalSeparator => scancode::Scancode::DecimalSeparator,
        Scancode::CurrencyUnit => scancode::Scancode::CurrencyUnit,
        Scancode::CurrencySubUnit => scancode::Scancode::CurrencySubUnit,
        Scancode::KpLeftParen => scancode::Scancode::KpLeftParen,
        Scancode::KpRightParen => scancode::Scancode::KpRightParen,
        Scancode::KpLeftBrace => scancode::Scancode::KpLeftBrace,
        Scancode::KpRightBrace => scancode::Scancode::KpRightBrace,
        Scancode::KpTab => scancode::Scancode::KpTab,
        Scancode::KpBackspace => scancode::Scancode::KpBackspace,
        Scancode::KpA => scancode::Scancode::KpA,
        Scancode::KpB => scancode::Scancode::KpB,
        Scancode::KpC => scancode::Scancode::KpC,
        Scancode::KpD => scancode::Scancode::KpD,
        Scancode::KpE => scancode::Scancode::KpE,
        Scancode::KpF => scancode::Scancode::KpF,
        Scancode::KpXor => scancode::Scancode::KpXor,
        Scancode::KpPower => scancode::Scancode::KpPower,
        Scancode::KpPercent => scancode::Scancode::KpPercent,
        Scancode::KpLess => scancode::Scancode::KpLess,
        Scancode::KpGreater => scancode::Scancode::KpGreater,
        Scancode::KpAmpersand => scancode::Scancode::KpAmpersand,
        Scancode::KpDblAmpersand => scancode::Scancode::KpDblAmpersand,
        Scancode::KpVerticalBar => scancode::Scancode::KpVerticalBar,
        Scancode::KpDblVerticalBar => scancode::Scancode::KpDblVerticalBar,
        Scancode::KpColon => scancode::Scancode::KpColon,
        Scancode::KpHash => scancode::Scancode::KpHash,
        Scancode::KpSpace => scancode::Scancode::KpSpace,
        Scancode::KpAt => scancode::Scancode::KpAt,
        Scancode::KpExclam => scancode::Scancode::KpExclam,
        Scancode::KpMemStore => scancode::Scancode::KpMemStore,
        Scancode::KpMemRecall => scancode::Scancode::KpMemRecall,
        Scancode::KpMemClear => scancode::Scancode::KpMemClear,
        Scancode::KpMemAdd => scancode::Scancode::KpMemAdd,
        Scancode::KpMemSubtract => scancode::Scancode::KpMemSubtract,
        Scancode::KpMemMultiply => scancode::Scancode::KpMemMultiply,
        Scancode::KpMemDivide => scancode::Scancode::KpMemDivide,
        Scancode::KpPlusMinus => scancode::Scancode::KpPlusMinus,
        Scancode::KpClear => scancode::Scancode::KpClear,
        Scancode::KpClearEntry => scancode::Scancode::KpClearEntry,
        Scancode::KpBinary => scancode::Scancode::KpBinary,
        Scancode::KpOctal => scancode::Scancode::KpOctal,
        Scancode::KpDecimal => scancode::Scancode::KpDecimal,
        Scancode::KpHexadecimal => scancode::Scancode::KpHexadecimal,
        Scancode::LCtrl => scancode::Scancode::LCtrl,
        Scancode::LShift => scancode::Scancode::LShift,
        Scancode::LAlt => scancode::Scancode::LAlt,
        Scancode::LGui => scancode::Scancode::LGui,
        Scancode::RCtrl => scancode::Scancode::RCtrl,
        Scancode::RShift => scancode::Scancode::RShift,
        Scancode::RAlt => scancode::Scancode::RAlt,
        Scancode::RGui => scancode::Scancode::RGui,
        Scancode::Mode => scancode::Scancode::Mode,
        Scancode::AudioNext => scancode::Scancode::AudioNext,
        Scancode::AudioPrev => scancode::Scancode::AudioPrev,
        Scancode::AudioStop => scancode::Scancode::AudioStop,
        Scancode::AudioPlay => scancode::Scancode::AudioPlay,
        Scancode::AudioMute => scancode::Scancode::AudioMute,
        Scancode::MediaSelect => scancode::Scancode::MediaSelect,
        Scancode::Www => scancode::Scancode::Www,
        Scancode::Mail => scancode::Scancode::Mail,
        Scancode::Calculator => scancode::Scancode::Calculator,
        Scancode::Computer => scancode::Scancode::Computer,
        Scancode::AcSearch => scancode::Scancode::AcSearch,
        Scancode::AcHome => scancode::Scancode::AcHome,
        Scancode::AcBack => scancode::Scancode::AcBack,
        Scancode::AcForward => scancode::Scancode::AcForward,
        Scancode::AcStop => scancode::Scancode::AcStop,
        Scancode::AcRefresh => scancode::Scancode::AcRefresh,
        Scancode::AcBookmarks => scancode::Scancode::AcBookmarks,
        Scancode::BrightnessDown => scancode::Scancode::BrightnessDown,
        Scancode::BrightnessUp => scancode::Scancode::BrightnessUp,
        Scancode::DisplaySwitch => scancode::Scancode::DisplaySwitch,
        Scancode::KbdIllumToggle => scancode::Scancode::KbdIllumToggle,
        Scancode::KbdIllumDown => scancode::Scancode::KbdIllumDown,
        Scancode::KbdIllumUp => scancode::Scancode::KbdIllumUp,
        Scancode::Eject => scancode::Scancode::Eject,
        Scancode::Sleep => scancode::Scancode::Sleep,
        Scancode::App1 => scancode::Scancode::App1,
        Scancode::App2 => scancode::Scancode::App2,
        Scancode::Num => scancode::Scancode::Num,
    }
}

pub fn map_sdlmod(keymod: sdl2::keyboard::Mod) -> scancode::Mod {
    match keymod {
        sdl2::keyboard::Mod::LCTRLMOD => scancode::Mod::LCTRLMOD,
        sdl2::keyboard::Mod::RCTRLMOD => scancode::Mod::RCTRLMOD,
        sdl2::keyboard::Mod::LGUIMOD => scancode::Mod::LGUIMOD,
        sdl2::keyboard::Mod::RGUIMOD => scancode::Mod::RGUIMOD,
        _ => scancode::Mod::NONE,
    }
}

#[derive(Clone, Debug)]
pub enum FrontendError {
    Sdl(String),
    Renderer(String),
    Other(String),
}

pub type FrontendResult<T> = Result<T, FrontendError>;

impl From<sdl2::IntegerOrSdlError> for FrontendError {
    fn from(e: sdl2::IntegerOrSdlError) -> FrontendError {
        FrontendError::Sdl(format!("{:?}", e))
    }
}

impl From<sdl2::video::WindowBuildError> for FrontendError {
    fn from(e: sdl2::video::WindowBuildError) -> FrontendError {
        FrontendError::Renderer(format!("{:?}", e))
    }
}

impl From<String> for FrontendError {
    fn from(e: String) -> FrontendError {
        FrontendError::Other(e)
    }
}

pub struct Frontend {
    sdl: Sdl,
    event_pump: EventPump,
    renderer: renderer::renderer::Renderer,
    controllers: controllers::Controllers,
    times: frametimes::FrameTimes,
    pub uc: unicorn::unicorn::Unicorn,
    fps_counter: fps::FpsCounter,
}


impl Frontend {
    pub fn init(scale: Scale,
                fullscreen: bool,
                opengl: bool,
                show_mouse: bool)
                -> FrontendResult<Frontend> {
        info!("[Frontend] SDL2 init");
        let sdl_context = sdl2::init()?;

        info!("[Frontend] SDL2 Video init");
        let sdl_video = sdl_context.video()?;

        info!("[Frontend] SDL2 event pump");
        let event_pump = sdl_context.event_pump()?;

        info!("[Frontend] SDL2 audio");
        sdl_context.audio()?;

        let uc = unicorn::unicorn::Unicorn::new();

        let renderer = {
            let screen = &mut uc.screen.lock().unwrap();

            info!("[Frontend] creating renderer");
            renderer::renderer::Renderer::new(sdl_video, screen, fullscreen, opengl, scale).unwrap()
        };

        info!("[Frontend] Disable mouse cursor ? {:?}", show_mouse);
        sdl_context.mouse().show_cursor(show_mouse);

        Ok(Frontend {
            sdl: sdl_context,
            event_pump: event_pump,
            renderer: renderer,
            controllers: controllers::Controllers::new(),
            times: frametimes::FrameTimes::new(Duration::from_secs(1) / 60),
            uc: uc,
            fps_counter: fps::FpsCounter::new(),
        })
    }

    pub fn start(&mut self) {
        info!("[Frontend] Start");

        self.times.reset();

        info!("[Frontend] initialise Unicorn");
        self.uc.setup();
    }

    pub fn init_controllers(&mut self, pathdb: String) {
        info!("[Frontend] Init Controllers");

        let game_controller_subsystem = self.sdl.game_controller().unwrap();

        info!("[Frontend] Loading the database of Game Controller");
        info!("[Frontend] -> {:?}",
              game_controller_subsystem.load_mappings(Path::new(&pathdb)));

        let available = match game_controller_subsystem.num_joysticks() {
            Ok(n) => n,
            Err(e) => panic!("[Frontend] can't enumerate joysticks: {}", e),
        };

        info!("[Frontend][CONTROLLER] {} joysticks available", available);

        for id in 0..available {
            if game_controller_subsystem.is_game_controller(id) {
                info!("[Frontend][CONTROLLER] Attempting to open controller {}",
                      id);

                match game_controller_subsystem.open(id) {
                    Ok(c) => {
                        // We managed to find and open a game controller,
                        // exit the loop
                        info!("[Frontend][CONTROLLER] Success: opened \"{}\"", c.name());
                        info!("[Frontend][CONTROLLER] Success: opened \"{}\"", c.mapping());

                        self.controllers.push_controller(id, Some(c).unwrap());
                        break;
                    }
                    Err(e) => error!("[Frontend][CONTROLLER] failed: {:?}", e),
                }
            } else {
                info!("[Frontend][CONTROLLER] {} is not a game controller", id);
            }
        }

        let joystick_subsystem = self.sdl.joystick().unwrap();

        let available = match joystick_subsystem.num_joysticks() {
            Ok(n) => n,
            Err(e) => panic!("[Frontend][JOYSTICK] can't enumerate joysticks: {}", e),
        };

        info!("[Frontend][JOYSTICK] {} joysticks available", available);

        // Iterate over all available joysticks and stop once we manage to
        // open one.
        for id in 0..available {
            match joystick_subsystem.open(id) {
                Ok(c) => {
                    info!("[Frontend][JOYSTICK] Success: opened \"{}\"", c.name());
                    self.controllers.push_joystick(id, Some(c).unwrap());
                }
                Err(e) => error!("[Frontend][JOYSTICK] failed: {:?}", e),
            }
        }
    }

    #[allow(dead_code)]
    pub fn run_native_cartridge(&mut self) {
        self.uc.current_code_type = unicorn::unicorn::Code::RUST;
        self.uc.init();

        self.handle_event();
    }

    pub fn run_cartridge(&mut self, filename: &str, editor: bool) {
        let success = self.uc.load_cartridge(filename, filename, editor);

        if success {
            info!("[Frontend] Successfully loaded the cartridge");
            // Call the init of the cartridge
            self.handle_event();
        } else {
            error!("[Frontend] Failed to load the cartridge");
        }
    }

    pub fn run_cartridge_raw(&mut self, filename: &str, data: Vec<u8>, editor: bool) {
        let success = self.uc.load_cartridge_raw(filename, data, editor);

        if success {
            info!("[Frontend] Successfully loaded the cartridge");
            // Call the init of the cartridge
            self.handle_event();
        } else {
            error!("[Frontend] Failed to load the cartridge");
        }
    }

    pub fn run_interactive(&mut self) {
        self.uc.init_interactive();
        self.handle_event();
    }


    #[cfg(not(target_os = "emscripten"))]
    fn handle_event(&mut self) {
        let mut previous_frame_time = Instant::now();
        
        'main: loop {
            self.times.update();

            self.fps_counter.update(self.times.get_last_time());

            self.uc.fps = self.fps_counter.get_fps();

            let mouse_state = self.event_pump.mouse_state();
            let (width, height) = self.renderer.get_dimensions();

            let mouse_state_x = (mouse_state.x() as f32 * (400.0 / width as f32)) as i32;
            let mouse_state_y = (mouse_state.y() as f32 * (240.0 / height as f32)) as i32;

            self.uc
                .players
                .lock()
                .unwrap()
                .set_mouse_x(mouse_state_x);

            self.uc
                .players
                .lock()
                .unwrap()
                .set_mouse_y(mouse_state_y);

            self.uc.players.lock().unwrap().clear_text();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::Window { win_event: WindowEvent::SizeChanged(_, _), .. } => {
                        // self.renderer
                        //    .update_viewport(&self.uc.screen.lock().unwrap());
                    }
                    Event::Window { win_event: WindowEvent::Close, .. } => {
                        break 'main;
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        let mut left = false;
                        let mut right = false;
                        let mut middle = false;

                        match mouse_btn {
                            MouseButton::Left => left = true,
                            MouseButton::Right => right = true,
                            MouseButton::Middle => middle = true,
                            _ => {}
                        }

                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .mouse_button_down(left,
                                               right,
                                               middle,
                                               self.uc.info.lock().unwrap().elapsed_time);
                    }
                    Event::MouseButtonUp { mouse_btn: _, .. } => {
                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .mouse_button_up();
                    }
                    Event::TextInput { text, .. } => {
                        // info!("TEXT INPUT {:?}", text);
                        if text.len() == 1 {
                            self.uc.players.lock().unwrap().set_text(text.clone());
                        }
                    }
                    Event::KeyDown { scancode: Some(scancode), keycode: _, keymod, repeat, .. } => {
                        // info!("KEY DOWN {:?} {:?} {:?}", scancode, keycode, keymod);

                        if scancode == Scancode::AcHome {
                            break 'main;
                        }

                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .key_down(map_sdlmod(keymod),
                                      map_sdlscancode(scancode),
                                      repeat,
                                      self.uc.info.lock().unwrap().elapsed_time);

                        if scancode == Scancode::F2 {
                            self.uc.configuration.lock().unwrap().toggle_info_overlay();
                        } else if scancode == Scancode::F3 {
                            let dt = Utc::now();
                            self.uc
                                .screenshot(&("screenshot-".to_string() +
                                              &dt.format("%Y-%m-%d-%H-%M-%S.png").to_string()));
                        } else if scancode == Scancode::F4 {
                            let record_screen = self.uc.is_recording();
                            if !record_screen {
                                let dt = Utc::now();
                                self.uc
                                    .start_record(&("record-".to_string() +
                                                    &dt.format("%Y-%m-%d-%H-%M-%S.gif")
                                        .to_string()));
                            } else {
                                self.uc.stop_record();
                            }
                        } else if scancode == Scancode::F5 {
                            self.uc.save_current_cartridge();
                        } else if scancode == Scancode::F6 || scancode == Scancode::AcBack {
                            self.uc.switch_code();
                        }

                        if self.uc.players.lock().unwrap().get_value_quick(0, 7) == 1 {
                            self.uc.switch_pause();
                        }
                    }
                    Event::KeyUp { scancode: Some(scancode), keymod, .. } => {
                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .key_up(map_sdlmod(keymod), map_sdlscancode(scancode));
                    }

                    Event::ControllerButtonDown { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button(button) {
                            self.uc
                                .players
                                .lock()
                                .unwrap()
                                .key_down_direct(0,
                                                 key,
                                                 false,
                                                 self.uc.info.lock().unwrap().elapsed_time)
                        }
                    }

                    Event::ControllerButtonUp { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button(button) {
                            self.uc.players.lock().unwrap().key_up_direct(0, key);
                        }
                    }

                    Event::ControllerAxisMotion { which: id, axis, value, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some((key, state)) = map_axis(axis, value) {
                            if axis == Axis::LeftX && value == 128 {
                                self.uc.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis == Axis::LeftY && value == -129 {
                                self.uc.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.uc
                                        .players
                                        .lock()
                                        .unwrap()
                                        .key_down_direct(0,
                                                         key,
                                                         false,
                                                         self.uc.info.lock().unwrap().elapsed_time);
                                } else {
                                    self.uc.players.lock().unwrap().key_up_direct(0, key);
                                }
                            }
                        }
                    }

                    Event::JoyAxisMotion { which: id, axis_idx, value, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some((key, state)) = map_axis_joystick(axis_idx, value) {
                            if axis_idx == 0 && value == 128 {
                                self.uc.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis_idx == 1 && value == -129 {
                                self.uc.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.uc
                                        .players
                                        .lock()
                                        .unwrap()
                                        .key_down_direct(0,
                                                         key,
                                                         false,
                                                         self.uc.info.lock().unwrap().elapsed_time);
                                } else {
                                    self.uc.players.lock().unwrap().key_up_direct(0, key);
                                }
                            }
                        }
                    }

                    Event::JoyButtonDown { which: id, button_idx, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button_joystick(button_idx) {
                            self.uc
                                .players
                                .lock()
                                .unwrap()
                                .key_down_direct(0,
                                                 key,
                                                 false,
                                                 self.uc.info.lock().unwrap().elapsed_time);
                        }
                    }

                    Event::JoyButtonUp { which: id, button_idx, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button_joystick(button_idx) {
                            self.uc.players.lock().unwrap().key_up_direct(0, key);
                        }
                    }

                    _ => (),
                }
            }

            if !self.uc.update() {
                info!("[Frontend] End of requested");
                self.uc.stop();
                break 'main;
            }

            self.uc.draw();

            let now = Instant::now();
            let dt = now.duration_since(previous_frame_time);
            previous_frame_time = now;
            self.uc.update_time(dt);
            self.blit();
        }
    }


    #[cfg(target_os = "emscripten")]
    fn handle_event(&mut self) {
        let mut previous_frame_time = Instant::now();
        
        emscripten::set_main_loop_callback(|| {
            self.times.update();

            self.fps_counter.update(self.times.get_last_time());

            self.uc.fps = self.fps_counter.get_fps();

            let mouse_state = self.event_pump.mouse_state();
            let (width, height) = self.renderer.get_dimensions();

            let mouse_state_x = (mouse_state.x() as f32 * (400.0 / width as f32)) as i32;
            let mouse_state_y = (mouse_state.y() as f32 * (240.0 / height as f32)) as i32;

            self.uc
                .players
                .lock()
                .unwrap()
                .set_mouse_x(mouse_state_x);

            self.uc
                .players
                .lock()
                .unwrap()
                .set_mouse_y(mouse_state_y);

            self.uc.players.lock().unwrap().clear_text();

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => return,
                    Event::Window { win_event: WindowEvent::Close, .. } => {
                        return;
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        let mut left = false;
                        let mut right = false;
                        let mut middle = false;

                        match mouse_btn {
                            MouseButton::Left => left = true,
                            MouseButton::Right => right = true,
                            MouseButton::Middle => middle = true,
                            _ => {}
                        }

                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .mouse_button_down(left,
                                               right,
                                               middle,
                                               self.uc.info.lock().unwrap().elapsed_time);
                    }
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .mouse_button_up();
                    }
                    Event::TextInput { text, .. } => {
                        if text.len() == 1 {
                            self.uc.players.lock().unwrap().set_text(text.clone());
                        }
                    }
                    Event::KeyDown { scancode: Some(scancode), keycode, keymod, repeat, .. } => {
                        if scancode == Scancode::AcHome {
                            return;
                        }

                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .key_down(map_sdlmod(keymod),
                                      map_sdlscancode(scancode),
                                      repeat,
                                      self.uc.info.lock().unwrap().elapsed_time);

                        if scancode == Scancode::F2 {
                            self.uc.configuration.lock().unwrap().toggle_info_overlay();
                        } else if scancode == Scancode::F3 {
                            let dt = Utc::now();
                            self.uc
                                .screenshot(&("screenshot-".to_string() +
                                              &dt.format("%Y-%m-%d-%H-%M-%S.png").to_string()));
                        } else if scancode == Scancode::F4 {
                            let record_screen = self.uc.is_recording();
                            if !record_screen {
                                let dt = Utc::now();
                                self.uc
                                    .start_record(&("record-".to_string() +
                                                    &dt.format("%Y-%m-%d-%H-%M-%S.gif")
                                        .to_string()));
                            } else {
                                self.uc.stop_record();
                            }
                        } else if scancode == Scancode::F5 {
                            self.uc.save_current_cartridge();
                        } else if scancode == Scancode::F6 || scancode == Scancode::AcBack {
                            self.uc.switch_code();
                        }

                        if self.uc.players.lock().unwrap().get_value_quick(0, 7) == 1 {
                            self.uc.switch_pause();
                        }
                    }
                    Event::KeyUp { scancode: Some(scancode), keymod, .. } => {
                        self.uc
                            .players
                            .lock()
                            .unwrap()
                            .key_up(map_sdlmod(keymod), map_sdlscancode(scancode));
                    }

                    Event::ControllerButtonDown { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button(button) {
                            self.uc
                                .players
                                .lock()
                                .unwrap()
                                .key_down_direct(0,
                                                 key,
                                                 false,
                                                 self.uc.info.lock().unwrap().elapsed_time)
                        }
                    }

                    Event::ControllerButtonUp { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button(button) {
                            self.uc.players.lock().unwrap().key_up_direct(0, key);
                        }
                    }

                    Event::ControllerAxisMotion { which: id, axis, value, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some((key, state)) = map_axis(axis, value) {
                            if axis == Axis::LeftX && value == 128 {
                                self.uc.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis == Axis::LeftY && value == -129 {
                                self.uc.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.uc
                                        .players
                                        .lock()
                                        .unwrap()
                                        .key_down_direct(0,
                                                         key,
                                                         false,
                                                         self.uc.info.lock().unwrap().elapsed_time);
                                } else {
                                    self.uc.players.lock().unwrap().key_up_direct(0, key);
                                }
                            }
                        }
                    }

                    Event::JoyAxisMotion { which: id, axis_idx, value, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some((key, state)) = map_axis_joystick(axis_idx, value) {
                            if axis_idx == 0 && value == 128 {
                                self.uc.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis_idx == 1 && value == -129 {
                                self.uc.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.uc
                                        .players
                                        .lock()
                                        .unwrap()
                                        .key_down_direct(0,
                                                         key,
                                                         false,
                                                         self.uc.info.lock().unwrap().elapsed_time);
                                } else {
                                    self.uc.players.lock().unwrap().key_up_direct(0, key);
                                }
                            }
                        }
                    }

                    Event::JoyButtonDown { which: id, button_idx, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button_joystick(button_idx) {
                            self.uc
                                .players
                                .lock()
                                .unwrap()
                                .key_down_direct(0,
                                                 key,
                                                 false,
                                                 self.uc.info.lock().unwrap().elapsed_time);
                        }
                    }

                    Event::JoyButtonUp { which: id, button_idx, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button_joystick(button_idx) {
                            self.uc.players.lock().unwrap().key_up_direct(0, key);
                        }
                    }

                    _ => (),
                }
            }

            if !self.uc.update() {
                info!("[Frontend] End of requested");
                self.uc.stop();
                return;
            }

            self.uc.draw();
            self.uc.update_sound();

            let now = Instant::now();
            let dt = now.duration_since(previous_frame_time);
            previous_frame_time = now;
            self.uc.update_time(dt);
            self.blit();
        });
    }

    pub fn blit(&mut self) {
        self.renderer.blit(&mut self.uc.screen.lock().unwrap());
        self.times.limit();
    }
}