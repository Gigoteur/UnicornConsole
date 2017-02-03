pub mod fps;
pub mod frametimes;

use time;

use sdl2;
use std::sync::{Arc, Mutex};
use sdl2::Sdl;
use sdl2::EventPump;
use sdl2::VideoSubsystem;
use std::time::{Duration, Instant};
use sdl2::event::{Event, WindowEvent};

use std::error::Error;
use std::path::Path;

use chrono::Local;

use sdl2::controller::{Axis, Button};
use sdl2::keyboard::Keycode;

#[macro_use]
use chan;
use chan::{Receiver, Sender};

use renderer;
use px8;
use config;
use config::keys::{PX8Key, map_axis, map_button, map_keycode, map_button_joystick, map_axis_joystick};
use config::controllers;

use gfx::{Scale};

#[cfg(target_os = "emscripten")]
use px8::emscripten::{emscripten};

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

pub struct Channels {
    tx_input: Sender<Vec<u8>>,
    rx_input: Receiver<Vec<u8>>,
    tx_output: Sender<Vec<u8>>,
    rx_output: Receiver<Vec<u8>>,
}

impl Channels {
    pub fn new() -> Channels {
        let (tx_input, rx_input): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = chan::sync(0);
        let (tx_output, rx_output): (Sender<Vec<u8>>, Receiver<Vec<u8>>) = chan::sync(0);

        Channels {
            tx_input: tx_input,
            rx_input: rx_input,
            tx_output: tx_output,
            rx_output: rx_output,
        }
    }
}

pub struct Frontend {
    sdl: Sdl,
    event_pump: EventPump,
    renderer: renderer::renderer::Renderer,
    controllers: controllers::Controllers,
    times: frametimes::FrameTimes,
    px8: px8::Px8New,
    info: Arc<Mutex<px8::info::Info>>,
    players: Arc<Mutex<config::Players>>,
    channels: Channels,
    start_time: time::Tm,
    elapsed_time: f64,
    delta: Duration,
    scale: Scale,
    fps_counter: fps::FpsCounter,
}

impl Frontend {
    pub fn init(scale: Scale, fullscreen: bool) -> FrontendResult<Frontend> {
        info!("Frontend: SDL2 init");
        let sdl = try!(sdl2::init());

        info!("Frontend: SDL2 Video init");
        let sdl_video = try!(sdl.video());

        info!("Frontend: SDL2 event pump");
        let event_pump = try!(sdl.event_pump());

        info!("Frontend: creating renderer");
        let renderer = renderer::renderer::Renderer::new(sdl_video, fullscreen, scale).unwrap();

        // Disable mouse in the window
        sdl.mouse().show_cursor(true);

        Ok(Frontend {
            sdl: sdl,
            event_pump: event_pump,
            renderer: renderer,
            controllers: controllers::Controllers::new(),
            times: frametimes::FrameTimes::new(Duration::from_secs(1) / 60),
            px8: px8::Px8New::new(),
            info: Arc::new(Mutex::new(px8::info::Info::new())),
            players: Arc::new(Mutex::new(config::Players::new())),
            channels: Channels::new(),
            start_time: time::now(),
            elapsed_time: 0.,
            delta: Duration::from_secs(0),
            scale: scale,
            fps_counter : fps::FpsCounter::new(),
        })
    }

    pub fn run_cartridge(&mut self, filename: String, editor: bool, server: bool) {
        self.start_time = time::now();
        self.times.reset();

        info!("Frontend: initialise controllers");
        self.init_controllers();

        info!("Frontend: initialise PX8");
        self.px8.init();

        self._run_cartridge(filename, editor);
    }

    pub fn update_time(&mut self) {
        let new_time = time::now();
        let diff_time = new_time - self.start_time;
        let nanoseconds = (diff_time.num_nanoseconds().unwrap() as f64) - (diff_time.num_seconds() * 1000000000) as f64;

        self.elapsed_time = diff_time.num_seconds() as f64 + nanoseconds / 1000000000.0;

        self.info.lock().unwrap().elapsed_time = self.elapsed_time;

        self.players.lock().unwrap().update(self.elapsed_time);
    }

    pub fn init_controllers(&mut self) {
        info!("Init Controllers");

        let game_controller_subsystem = self.sdl.game_controller().unwrap();

        info!("Loading the database of Game Controller");
        info!("-> {:?}", game_controller_subsystem.load_mappings(Path::new("./sys/config/gamecontrollerdb.txt")));

        let available =
        match game_controller_subsystem.num_joysticks() {
            Ok(n) => n,
            Err(e) => panic!("can't enumerate joysticks: {}", e),
        };

        info!("[CONTROLLER] {} joysticks available", available);

        for id in 0..available {
            if game_controller_subsystem.is_game_controller(id) {
                info!("Attempting to open controller {}", id);

                match game_controller_subsystem.open(id) {
                    Ok(c) => {
                        // We managed to find and open a game controller,
                        // exit the loop
                        info!("[CONTROLLER] Success: opened \"{}\"", c.name());
                        info!("[CONTROLLER] Success: opened \"{}\"", c.mapping());

                        self.controllers.push_controller(id, Some(c).unwrap());
                        break;
                    },
                    Err(e) => error!("failed: {:?}", e),
                }
            } else {
                info!("[CONTROLLER] {} is not a game controller", id);
            }
        }

        let joystick_subsystem = self.sdl.joystick().unwrap();

        let available =
        match joystick_subsystem.num_joysticks() {
            Ok(n) => n,
            Err(e) => panic!("can't enumerate joysticks: {}", e),
        };

        info!("[JOYSTICK] {} joysticks available", available);

        // Iterate over all available joysticks and stop once we manage to
        // open one.
        for id in 0..available {
            match joystick_subsystem.open(id) {
                Ok(c) => {
                    info!("[JOYSTICK] Success: opened \"{}\"", c.name());
                    self.controllers.push_joystick(id, Some(c).unwrap());
                },
                Err(e) => error!("failed: {:?}", e),
            }
        }
    }

    pub fn _run_cartridge(&mut self, filename: String, editor: bool) {
        self.px8.load_cartridge(filename.clone(),
                                self.channels.tx_input.clone(),
                                self.channels.rx_output.clone(),
                                self.players.clone(),
                                self.info.clone(),
                                editor);

        // Call the init of the cartridge
        self.px8.init_time = self.px8.call_init() * 1000.0;

        self.handle_event(editor);
    }

    #[cfg(not(target_os = "emscripten"))]
    fn handle_event(&mut self, editor: bool) {
        info!("Handle Event");

        'main: loop {
            let delta = self.times.update();

            self.fps_counter.update(self.times.get_last_time());

            self.px8.fps = self.fps_counter.get_fps();

            let mouse_state = self.event_pump.mouse_state();
            let (width, height) = self.renderer.get_dimensions();

            self.players.lock().unwrap().set_mouse_x(mouse_state.x() / (width as i32 / px8::SCREEN_WIDTH as i32));
            self.players.lock().unwrap().set_mouse_y(mouse_state.y() / (height as i32 / px8::SCREEN_HEIGHT as i32));
            self.players.lock().unwrap().set_mouse_state(mouse_state);

            if mouse_state.left() {
                debug!("MOUSE X {:?} Y {:?}",
                      mouse_state.x() / (width as i32 / px8::SCREEN_WIDTH as i32),
                      mouse_state.y() / (height as i32 / px8::SCREEN_HEIGHT as i32));
            }

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::KeyDown { keycode: Some(keycode), .. } if keycode == Keycode::Escape => break 'main,
                    Event::Window { win_event: WindowEvent::SizeChanged(_, _), .. } => {
                        self.renderer.update_dimensions();
                    },
                    Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                        if let (Some(key), player) = map_keycode(keycode) {
                            self.players.lock().unwrap().key_down(player, key, repeat, self.elapsed_time);
                        }

                        if keycode == Keycode::F2 {
                            self.px8.toggle_info_overlay();
                        } else if keycode == Keycode::F3 {
                            let dt = Local::now();
                            self.px8.screenshot("screenshot-".to_string() + &dt.format("%Y-%m-%d-%H-%M-%S.png").to_string());
                        } else if keycode == Keycode::F4 {
                            let record_screen = self.px8.is_recording();
                            if ! record_screen {
                                let dt = Local::now();
                                self.px8.start_record("record-".to_string() + &dt.format("%Y-%m-%d-%H-%M-%S.gif").to_string());
                            } else {
                                self.px8.stop_record(self.scale.factor());
                            }
                        } else if keycode == Keycode::F5 {
                            if editor {
                                let dt = Local::now();
                                self.px8.save_current_cartridge(dt.format("%Y-%m-%d-%H-%M-%S").to_string());
                            }
                        } else if keycode == Keycode::F6 && editor {
                            self.px8.switch_code();
                            // Call the init of the new code
                            self.px8.init_time = self.px8.call_init() * 1000.0;
                        }

                        if self.players.lock().unwrap().get_value_quick(0, 7) == 1 {
                            self.px8.switch_pause();
                        }
                    },
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        if let (Some(key), player) = map_keycode(keycode) {
                            self.players.lock().unwrap().key_up(player, key)
                        }
                    },

                    Event::ControllerButtonDown { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        info!("ID [{:?}] Controller button Down {:?}", id, button);
                        if let Some(key) = map_button(button) {
                            self.players.lock().unwrap().key_down(0, key, false, self.elapsed_time)
                        }
                    },

                    Event::ControllerButtonUp { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        info!("ID [{:?}] Controller button UP {:?}", id, button);
                        if let Some(key) = map_button(button) {
                            self.players.lock().unwrap().key_up(0, key)
                        }
                    },

                    Event::ControllerAxisMotion { which: id, axis, value, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        info!("ID [{:?}] Controller Axis Motion {:?} {:?}", id, axis, value);

                        if let Some((key, state)) = map_axis(axis, value) {
                            info!("Key {:?} State {:?}", key, state);


                            if axis == Axis::LeftX && value == 128 {
                                self.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis == Axis::LeftY && value == -129 {
                                self.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.players.lock().unwrap().key_down(0, key, false, self.elapsed_time)
                                } else {
                                    self.players.lock().unwrap().key_up(0, key)
                                }
                            }
                        }
                    },

                    Event::JoyAxisMotion { which: id, axis_idx, value, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        info!("ID [{:?}] Joystick Axis Motion {:?} {:?}", id, axis_idx, value);

                        if let Some((key, state)) = map_axis_joystick(axis_idx, value) {
                            info!("Joystick Key {:?} State {:?}", key, state);

                            if axis_idx == 0 && value == 128 {
                                self.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis_idx == 1 && value == -129 {
                                self.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.players.lock().unwrap().key_down(0, key, false, self.elapsed_time)
                                } else {
                                    self.players.lock().unwrap().key_up(0, key)
                                }
                            }
                        }
                    },

                    Event::JoyButtonDown { which: id, button_idx, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        info!("ID [{:?}] Joystick button DOWN {:?}", id, button_idx);
                        if let Some(key) = map_button_joystick(button_idx) {
                            self.players.lock().unwrap().key_down(0, key, false, self.elapsed_time)
                        }
                    },

                    Event::JoyButtonUp { which: id, button_idx, .. } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        info!("ID [{:?}] Joystick Button UP {:?}", id, button_idx);
                        if let Some(key) = map_button_joystick(button_idx) {
                            self.players.lock().unwrap().key_up(0, key)
                        }
                    },

                    _ => (),
                }
            }

            if !self.px8.update() {
                break 'main;
            }

            self.px8.draw();

            self.px8.debug_update();

            self.update_time();
            self.blit();
        }
    }

    #[cfg(target_os = "emscripten")]
    fn handle_event(&mut self, editor: bool, players: Arc<Mutex<config::Players>>) {
        emscripten::set_main_loop_callback(|| {
            let delta = self.times.update();

            self.fps_counter.update(self.times.get_last_time());

            self.px8.fps = self.fps_counter.get_fps();

            let mouse_state = self.event_pump.mouse_state();
            let (width, height) = self.renderer.get_dimensions();

            players.lock().unwrap().set_mouse_x(mouse_state.x() / (width as i32 / px8::SCREEN_WIDTH as i32));
            players.lock().unwrap().set_mouse_y(mouse_state.y() / (height as i32 / px8::SCREEN_HEIGHT as i32));
            players.lock().unwrap().set_mouse_state(mouse_state);

            if mouse_state.left() {
                debug!("MOUSE X {:?} Y {:?}",
                       mouse_state.x() / (width as i32 / px8::SCREEN_WIDTH as i32),
                       mouse_state.y() / (height as i32 / px8::SCREEN_HEIGHT as i32));
            }

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Window { win_event: WindowEvent::SizeChanged(_, _), .. } => {
                        self.renderer.update_dimensions();
                    },
                    Event::KeyDown { keycode: Some(keycode), repeat, .. } => {
                        if let (Some(key), player) = map_keycode(keycode) {
                            players.lock().unwrap().key_down(player, key, repeat, self.elapsed_time);
                        }

                        if keycode == Keycode::F2 {
                            self.px8.toggle_info_overlay();
                        }

                        let pause = players.lock().unwrap().get_value_quick(0, 7) == 1;
                        if pause {
                            self.px8.pause();
                        }
                    },
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        if let (Some(key), player) = map_keycode(keycode) { players.lock().unwrap().key_up(player, key) }
                    },

                    Event::ControllerButtonDown { which: id, button, .. } => {
                        info!("Controller button Down {:?} {:?}", id, button);
                        if let Some(key) = map_button(button) { players.lock().unwrap().key_down(0, key, false, self.elapsed_time) }
                    },

                    Event::ControllerButtonUp { which: id, button, .. } => {
                        info!("Controller button UP {:?} {:?}", id, button);
                        if let Some(key) = map_button(button) { players.lock().unwrap().key_up(0, key) }
                    },

                    Event::JoyAxisMotion { which: id, axis_idx, value, .. } => {
                        info!("Joystick Axis Motion {:?} {:?} {:?}", id, axis_idx, value);

                        if let Some((key, state)) = map_axis_joystick(axis_idx, value) {
                            info!("Joystick Key {:?} State {:?}", key, state);

                            if axis_idx == 0 && value == 128 {
                                players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis_idx == 1 && value == -129 {
                                players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    players.lock().unwrap().key_down(0, key, false, self.elapsed_time)
                                } else {
                                    players.lock().unwrap().key_up(0, key)
                                }
                            }
                        }
                    },

                    Event::JoyButtonDown { which: id, button_idx, .. } => {
                        info!("Joystick button DOWN {:?} {:?}", id, button_idx);
                        if let Some(key) = map_button_joystick(button_idx) { players.lock().unwrap().key_down(0, key, false, self.elapsed_time) }
                    },

                    Event::JoyButtonUp { which: id, button_idx, .. } => {
                        info!("Joystick Button {:?} {:?} UP", id, button_idx);
                        if let Some(key) = map_button_joystick(button_idx) { players.lock().unwrap().key_up(0, key) }
                    },

                    _ => (),
                }
            }

            self.px8.update();
            self.px8.draw();

            self.px8.debug_update();

            self.update_time(players.clone());
            self.blit();
        });
    }

    pub fn blit(&mut self) {
        self.renderer.blit(&self.px8.screen.lock().unwrap().back_buffer);
        self.times.limit();
    }
}