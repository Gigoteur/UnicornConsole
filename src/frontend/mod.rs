pub mod fps;
pub mod frametimes;

use sdl2;
use sdl2::Sdl;
use sdl2::EventPump;
use std::time::Duration;
use sdl2::event::{Event, WindowEvent};

use std::path::Path;

use chrono::Local;

use sdl2::controller::Axis;
use sdl2::keyboard::{Keycode, Scancode};

use renderer;
use px8;
use config::keys::{map_axis, map_button, map_button_joystick, map_axis_joystick};
use config::controllers;

use gfx::Scale;

#[cfg(target_os = "emscripten")]
use px8::emscripten::emscripten;

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
    pub px8: px8::PX8,
    elapsed_time: f64,
    scale: Scale,
    fps_counter: fps::FpsCounter,
}


impl Frontend {
    pub fn init(scale: Scale,
                fullscreen: bool,
                opengl: bool,
                show_mouse: bool)
                -> FrontendResult<Frontend> {
        info!("[Frontend] SDL2 init");
        let sdl_context = try!(sdl2::init());

        info!("[Frontend] SDL2 Video init");
        let sdl_video = try!(sdl_context.video());

        info!("[Frontend] SDL2 event pump");
        let event_pump = try!(sdl_context.event_pump());

        info!("[Frontend] SDL2 audio");
        try!(sdl_context.audio());

        let px8 = px8::PX8::new();

        let renderer = {
            let mut screen = &mut px8.screen.lock().unwrap();

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
               px8: px8,
               elapsed_time: 0.,
               scale: scale,
               fps_counter: fps::FpsCounter::new(),
           })
    }

    pub fn start(&mut self, pathdb: String) {
        info!("[Frontend] Start");

        self.times.reset();

        info!("[Frontend] initialise controllers");
        self.init_controllers(pathdb);

        info!("[Frontend] initialise PX8");
        self.px8.setup();
    }

    pub fn update_time(&mut self) {
        self.px8.info.lock().unwrap().update();
        self.px8
            .players
            .lock()
            .unwrap()
            .update(self.px8.info.lock().unwrap().elapsed_time);
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
        self.px8.current_code_type = px8::Code::RUST;
        self.px8.init();

        self.handle_event();
    }

    pub fn run_cartridge(&mut self, filename: &str, editor: bool, mode: px8::PX8Mode) {
        let success = self.px8.load_cartridge(filename, filename, editor, mode);

        if success {
            info!("[Frontend] Successfully loaded the cartridge");
            // Call the init of the cartridge
            self.handle_event();
        } else {
            error!("[Frontend] Failed to load the cartridge");
        }
    }

    #[allow(dead_code)]
    pub fn run_cartridge_raw(&mut self,
                             filename: &str,
                             data: Vec<u8>,
                             editor: bool,
                             mode: px8::PX8Mode) {
        let success = self.px8.load_cartridge_raw(filename, data, editor, mode);

        if success {
            info!("[Frontend] Successfully loaded the cartridge");
            // Call the init of the cartridge
            self.handle_event();
        } else {
            error!("[Frontend] Failed to load the cartridge");
        }
    }

    pub fn run_interactive(&mut self) {
        self.px8.init_interactive();
        self.handle_event();
    }


    #[cfg(not(target_os = "emscripten"))]
    fn handle_event(&mut self) {
        'main: loop {
            self.times.update();

            self.fps_counter.update(self.times.get_last_time());

            self.px8.fps = self.fps_counter.get_fps();


            let mouse_state = self.event_pump.mouse_state();

            let (mouse_viewport_x, mouse_viewport_y) = {
                let screen = &self.px8.screen.lock().unwrap();
                self.renderer
                    .window_coords_to_viewport_coords(screen, mouse_state.x(), mouse_state.y())
            };

            self.px8
                .players
                .lock()
                .unwrap()
                .set_mouse_x(mouse_viewport_x);

            self.px8
                .players
                .lock()
                .unwrap()
                .set_mouse_y(mouse_viewport_y);

            for event in self.event_pump.poll_iter() {

                match event {
                    Event::Quit { .. } => break 'main,
                    Event::Window { win_event: WindowEvent::SizeChanged(_, _), .. } => {
                        self.renderer
                            .update_viewport(&self.px8.screen.lock().unwrap());
                    }
                    Event::Window { win_event: WindowEvent::Close, .. } => {
                        break 'main;
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        self.px8
                            .players
                            .lock()
                            .unwrap()
                            .mouse_button_down(mouse_btn, self.elapsed_time);
                    }
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        self.px8
                            .players
                            .lock()
                            .unwrap()
                            .mouse_button_up(mouse_btn, self.elapsed_time);
                    }
                    Event::KeyDown {
                        keycode: Some(keycode),
                        keymod: keymod,
                        repeat,
                        ..
                    } => {
                        if keycode == Keycode::AcHome {
                            break 'main
                        }

                        self.px8
                            .players
                            .lock()
                            .unwrap()
                            .key_down(keymod, keycode, repeat, self.elapsed_time);

                        if keycode == Keycode::F2 {
                            self.px8.configuration.lock().unwrap().toggle_info_overlay();
                        } else if keycode == Keycode::F3 {
                            let dt = Local::now();
                            self.px8
                                .screenshot(&("screenshot-".to_string() +
                                              &dt.format("%Y-%m-%d-%H-%M-%S.png").to_string()));
                        } else if keycode == Keycode::F4 {
                            let record_screen = self.px8.is_recording();
                            if !record_screen {
                                let dt = Local::now();
                                self.px8
                                    .start_record(&("record-".to_string() +
                                                    &dt.format("%Y-%m-%d-%H-%M-%S.gif")
                                                         .to_string()));
                            } else {
                                self.px8.stop_record();
                            }
                        } else if keycode == Keycode::F5 {
                            self.px8.save_current_cartridge();
                        } else if keycode == Keycode::F6 || keycode == Keycode::AcBack {
                            self.px8.switch_code();
                            self.px8.init();
                        } else if keycode == Keycode::F7 {
                            self.px8.next_palette();
                        }
 
                        if self.px8.players.lock().unwrap().get_value_quick(0, 7) == 1 {
                            self.px8.switch_pause();
                        }
                    }
                    Event::KeyUp { keycode: Some(keycode), keymod: keymod, .. } => {
                        self.px8.players.lock().unwrap().key_up(keymod, keycode);
                    }

                    Event::ControllerButtonDown { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button(button) {
                            self.px8
                                .players
                                .lock()
                                .unwrap()
                                .key_down_direct(0, key, false, self.elapsed_time)
                        }
                    }

                    Event::ControllerButtonUp { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button(button) {
                            self.px8.players.lock().unwrap().key_up_direct(0, key);
                        }
                    }

                    Event::ControllerAxisMotion {
                        which: id,
                        axis,
                        value,
                        ..
                    } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some((key, state)) = map_axis(axis, value) {
                            if axis == Axis::LeftX && value == 128 {
                                self.px8.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis == Axis::LeftY && value == -129 {
                                self.px8.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.px8
                                        .players
                                        .lock()
                                        .unwrap()
                                        .key_down_direct(0, key, false, self.elapsed_time);
                                } else {
                                    self.px8.players.lock().unwrap().key_up_direct(0, key);
                                }
                            }
                        }
                    }

                    Event::JoyAxisMotion {
                        which: id,
                        axis_idx,
                        value,
                        ..
                    } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some((key, state)) = map_axis_joystick(axis_idx, value) {
                            if axis_idx == 0 && value == 128 {
                                self.px8.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis_idx == 1 && value == -129 {
                                self.px8.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.px8
                                        .players
                                        .lock()
                                        .unwrap()
                                        .key_down_direct(0, key, false, self.elapsed_time);
                                } else {
                                    self.px8.players.lock().unwrap().key_up_direct(0, key);
                                }
                            }
                        }
                    }

                    Event::JoyButtonDown {
                        which: id,
                        button_idx,
                        ..
                    } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button_joystick(button_idx) {
                            self.px8
                                .players
                                .lock()
                                .unwrap()
                                .key_down_direct(0, key, false, self.elapsed_time);
                        }
                    }

                    Event::JoyButtonUp {
                        which: id,
                        button_idx,
                        ..
                    } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button_joystick(button_idx) {
                            self.px8.players.lock().unwrap().key_up_direct(0, key);
                        }
                    }

                    _ => (),
                }
            }

            if !self.px8.update() {
                info!("[Frontend] End of requested");
                self.px8.stop();
                break 'main;
            }

            self.px8.draw();
            self.px8.update_sound();

            self.update_time();

            self.blit();
        }
    }

    #[cfg(target_os = "emscripten")]
    fn handle_event(&mut self, editor: bool) {
        emscripten::set_main_loop_callback(|| {
            self.times.update();

            self.fps_counter.update(self.times.get_last_time());

            self.px8.fps = self.fps_counter.get_fps();

            let mouse_state = self.event_pump.mouse_state();

            let (mouse_viewport_x, mouse_viewport_y) = {
                let screen = &px8.screen.lock().unwrap();
                self.renderer
                    .window_coords_to_viewport_coords(screen, mouse_state.x(), mouse_state.y())
            };

            self.px8
                .players
                .lock()
                .unwrap()
                .set_mouse_x(mouse_viewport_x);
            self.px8
                .players
                .lock()
                .unwrap()
                .set_mouse_y(mouse_viewport_y);
            self.px8
                .players
                .lock()
                .unwrap()
                .set_mouse_state(mouse_state);

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break,
                    Event::KeyDown { keycode: Some(keycode), .. } if keycode == Keycode::Escape => {
                        break
                    }
                    Event::Window { win_event: WindowEvent::SizeChanged(_, _), .. } => {
                        self.renderer
                            .update_viewport(&self.px8.screen.lock().unwrap());
                    }
                    Event::MouseButtonDown { mouse_btn, .. } => {
                        self.px8
                            .players
                            .lock()
                            .unwrap()
                            .mouse_button_down(mouse_btn, self.elapsed_time);
                    }
                    Event::MouseButtonUp { mouse_btn, .. } => {
                        self.px8
                            .players
                            .lock()
                            .unwrap()
                            .mouse_button_up(mouse_btn, self.elapsed_time);
                    }
                    Event::KeyDown {
                        keycode: Some(keycode),
                        repeat,
                        ..
                    } => {
                        self.px8
                            .players
                            .lock()
                            .unwrap()
                            .key_down(keycode, repeat, self.elapsed_time);

                        if keycode == Keycode::F2 {
                            self.px8.configuration.lock().unwrap().toggle_info_overlay();
                        } else if keycode == Keycode::F3 {
                            let dt = Local::now();
                            self.px8
                                .screenshot(&("screenshot-".to_string() +
                                              &dt.format("%Y-%m-%d-%H-%M-%S.png").to_string()));
                        } else if keycode == Keycode::F4 {
                            let record_screen = self.px8.is_recording();
                            if !record_screen {
                                let dt = Local::now();
                                self.px8
                                    .start_record(&("record-".to_string() +
                                                    &dt.format("%Y-%m-%d-%H-%M-%S.gif")
                                                         .to_string()));
                            } else {
                                self.px8.stop_record();
                            }
                        } else if keycode == Keycode::F5 {
                            self.px8.save_current_cartridge();
                        } else if keycode == Keycode::F6 || keycode == Keycode::AcBack {
                            self.px8.switch_code();
                            self.px8.init();
                        } else if keycode == Keycode::F7 {
                            self.px8.next_palette();
                        }

                        if self.px8.players.lock().unwrap().get_value_quick(0, 7) == 1 {
                            self.px8.switch_pause();
                        }
                    }
                    Event::KeyUp { keycode: Some(keycode), .. } => {
                        self.px8.players.lock().unwrap().key_up(keycode);
                    }

                    Event::ControllerButtonDown { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button(button) {
                            self.px8
                                .players
                                .lock()
                                .unwrap()
                                .key_down_direct(0, key, false, self.elapsed_time)
                        }
                    }

                    Event::ControllerButtonUp { which: id, button, .. } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button(button) {
                            self.px8.players.lock().unwrap().key_up_direct(0, key);
                        }
                    }

                    Event::ControllerAxisMotion {
                        which: id,
                        axis,
                        value,
                        ..
                    } => {
                        if !self.controllers.is_controller(id as u32) {
                            break;
                        }

                        if let Some((key, state)) = map_axis(axis, value) {
                            if axis == Axis::LeftX && value == 128 {
                                self.px8.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis == Axis::LeftY && value == -129 {
                                self.px8.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.px8
                                        .players
                                        .lock()
                                        .unwrap()
                                        .key_down_direct(0, key, false, self.elapsed_time);
                                } else {
                                    self.px8.players.lock().unwrap().key_up_direct(0, key);
                                }
                            }
                        }
                    }

                    Event::JoyAxisMotion {
                        which: id,
                        axis_idx,
                        value,
                        ..
                    } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some((key, state)) = map_axis_joystick(axis_idx, value) {
                            if axis_idx == 0 && value == 128 {
                                self.px8.players.lock().unwrap().key_direc_hor_up(0);
                            } else if axis_idx == 1 && value == -129 {
                                self.px8.players.lock().unwrap().key_direc_ver_up(0);
                            } else {
                                if state {
                                    self.px8
                                        .players
                                        .lock()
                                        .unwrap()
                                        .key_down_direct(0, key, false, self.elapsed_time);
                                } else {
                                    self.px8.players.lock().unwrap().key_up_direct(0, key);
                                }
                            }
                        }
                    }

                    Event::JoyButtonDown {
                        which: id,
                        button_idx,
                        ..
                    } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button_joystick(button_idx) {
                            self.px8
                                .players
                                .lock()
                                .unwrap()
                                .key_down_direct(0, key, false, self.elapsed_time);
                        }
                    }

                    Event::JoyButtonUp {
                        which: id,
                        button_idx,
                        ..
                    } => {
                        if !self.controllers.is_joystick(id as u32) {
                            break;
                        }

                        if let Some(key) = map_button_joystick(button_idx) {
                            self.px8.players.lock().unwrap().key_up_direct(0, key);
                        }
                    }

                    _ => (),
                }
            }

            if !self.px8.update() {
                info!("[Frontend] End of PX8 requested");
                return;
            }

            self.px8.draw();

            self.update_time();
            self.blit();
        });
    }

    pub fn blit(&mut self) {
        self.renderer.blit(&mut self.px8.screen.lock().unwrap());
        self.times.limit();
    }
}
