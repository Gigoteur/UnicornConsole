#[cfg(feature = "cpython")]
#[allow(unused_variables)]
pub mod plugin {
    use cpython::*;

    use std::sync::{Arc, Mutex};
    use std::fs::File;
    use std::io::Read;

    use config::Players;
    use px8::info::Info;
    use px8::Palettes;
    use px8::noise::Noise;
    use px8::PX8Config;
    use gfx::Screen;
    use sound::sound::Sound;

    // Audio
    py_class!(class PX8Audio |py| {
    data sound: Arc<Mutex<Sound>>;

    // Chiptune
    
    def chiptune_play(&self, filetype: i32, channel: i32, filename: String, loops: i32, start_position: i32) -> PyResult<i32> {
        self.sound(py).lock().unwrap().chiptune_play(filetype, channel, filename, loops, start_position);
        Ok(0)
    }

    def chiptune_stop(&self, music: i32, sound: i32) -> PyResult<i32> {
        self.sound(py).lock().unwrap().chiptune_stop(music, sound);
        Ok(0)
    }
    
    def chiptune_pause(&self, music: i32, sound: i32) -> PyResult<i32> {
        self.sound(py).lock().unwrap().chiptune_pause(music, sound);
        Ok(0)
    }
    
    def chiptune_resume(&self, music: i32, sound: i32) -> PyResult<i32> {
        self.sound(py).lock().unwrap().chiptune_resume(music, sound);
        Ok(0)
    }

    def chiptune_volume(&self, volume: i32) -> PyResult<i32> {
        self.sound(py).lock().unwrap().chiptune_volume(volume);
        Ok(0)
    }

    def chiptune_position(&self) -> PyResult<i32> {
        Ok(self.sound(py).lock().unwrap().chiptune_get_position())
    }

    });

    // Palettes
    py_class!(class PX8Palette |py| {
    data palettes: Arc<Mutex<Palettes>>;

    def set_palette_color(&self, color:u32, r: u8, g: u8, b: u8) -> PyResult<i32> {
        self.palettes(py).lock().unwrap().set_color(color, r, g, b);
        Ok(0)
    }

    def reset_palette(&self) -> PyResult<i32> {
        self.palettes(py).lock().unwrap().reset();
        Ok(0)
    }

    def switch_palette(&self, name: String) -> PyResult<i32> {
        self.palettes(py).lock().unwrap().switch_to(&name);
        Ok(0)
    }

    });

    // Cart Data

    // Graphics
    py_class!(class PX8Graphic |py| {
    data screen: Arc<Mutex<Screen>>;

    def camera(&self, x: i32, y: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().camera(x, y);
        Ok(0)
    }

    def circ(&self, x: i32, y: i32, r: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().circ(x, y, r, color);
        Ok(0)
    }

    def circfill(&self, x: i32, y: i32, r: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().circfill(x, y, r, color);
        Ok(0)
    }

    def clip(&self, x: i32, y: i32, w: i32, h: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().clip(x, y, w, h);
        Ok(0)
    }

    def cls(&self) -> PyResult<i32> {
        self.screen(py).lock().unwrap().cls();
        Ok(0)
    }

    def color(&self, color:i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().color(color);
        Ok(0)
    }

    def ellipse(&self, x: i32, y: i32, rx: i32, ry: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().ellipse(x, y, rx, ry, color);
        Ok(0)
    }

    def ellipsefill(&self, x: i32, y: i32, rx: i32, ry: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().ellipsefill(x, y, rx, ry, color);
        Ok(0)
    }

    def fget(&self, idx: u32, v: u8) -> PyResult<bool> {
        Ok(self.screen(py).lock().unwrap().fget(idx, v))
    }

    def fget_all(&self, idx: u32) -> PyResult<u8> {
        Ok(self.screen(py).lock().unwrap().fget_all(idx))
    }

    def font(&self, name: String) -> PyResult<i32> {
        self.screen(py).lock().unwrap().font(&name);
        Ok(0)
    }

    def fset(&self, idx: u32, flag: u8, value: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().fset(idx, flag, value);
        Ok(0)
    }

    def fset_all(&self, idx: u32, flag: u8) -> PyResult<i32> {
        self.screen(py).lock().unwrap().fset_all(idx, flag);
        Ok(0)
    }

    def flip(&self) -> PyResult<i32> {
        Ok(0)
    }

    def line(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().line(x1, y1, x2, y2, color);
        Ok(0)
    }

    def mode(&self, width: usize, height: usize, aspect_ratio: f32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().mode(width, height, aspect_ratio);
        Ok(0)
    }

    def mode_get_width(&self) -> PyResult<usize> {
        Ok(self.screen(py).lock().unwrap().mode_width())
    }

    def mode_get_height(&self) -> PyResult<usize> {
        Ok(self.screen(py).lock().unwrap().mode_height())
    }

    def mode_get_aspect_ratio(&self) -> PyResult<f32> {
        Ok(self.screen(py).lock().unwrap().mode_aspect_ratio())
    }

    def palt(&self, c: i32, t: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().palt(c, t);
        Ok(0)
    }

    def pal(&self, c0: i32, c1: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().pal(c0, c1);
        Ok(0)
    }

    def pset(&self, x: i32, y: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().pset(x, y, color);
        Ok(0)
    }

    def print(&self, str: String, x: i32, y: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().print(str, x as i32, y as i32, color);
        Ok(0)
    }

    def pget(&self, x: i32, y: i32) -> PyResult<u32> {
        let value = self.screen(py).lock().unwrap().pget(x as u32, y as u32);
        Ok(value)
    }

    def rect(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().rect(x1, y1, x2, y2, color);
        Ok(0)
    }

    def rectfill(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().rectfill(x1, y1, x2, y2, color);
        Ok(0)
    }

    def sget(&self, x: i32, y: i32) -> PyResult<u8> {
        Ok(self.screen(py).lock().unwrap().sget(x as u32, y as u32))
    }

    def spr(&self, n: i32, x: i32, y: i32, w: i32, h: i32, flip_x: bool, flip_y: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().spr(n as u32,
                                            x as i32,
                                            y as i32,
                                            w as u32,
                                            h as u32,
                                            flip_x,
                                            flip_y);

        Ok(0)
    }

    def sset(&self, x: i32, y: i32, color: i32) -> PyResult<u8> {
        self.screen(py).lock().unwrap().sset(x as u32, y as u32, color);
        Ok(0)
    }

    def sspr(&self, sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32, dw: i32, dh: i32, flip_x: bool, flip_y: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().sspr(sx as u32,
                                             sy as u32,
                                             sw as u32,
                                             sh as u32,
                                             dx as i32,
                                             dy as i32,
                                             dw as u32,
                                             dh as u32,
                                             flip_x,
                                             flip_y);
        Ok(0)
    }


    def trigon(&self, x1: i32, y1: i32, x2: i32, y2: i32, x3: i32, y3: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().trigon(x1, y1, x2, y2, x3, y3, color);
        Ok(0)
    }


    def polygon(&self, x: PyList, y: PyList, color: i32) -> PyResult<i32> {
        if x.len(py) != y.len(py) {
            return Ok(-1);
        }

        if x.len(py) < 3 || y.len(py) < 3 {
            return Ok(-1);
        }

        let mut vx: Vec<i32> = Vec::new();
        let mut vy: Vec<i32> = Vec::new();

        for lx in x.iter(py) {
            vx.push(lx.extract::<i32>(py).unwrap());
        }

        for ly in y.iter(py) {
            vy.push(ly.extract::<i32>(py).unwrap());
        }

        self.screen(py).lock().unwrap().polygon(vx, vy, color);
        Ok(0)
    }

    });

    // Input
    py_class!(class PX8Input |py| {
    data players: Arc < Mutex < Players> >;

    def btn(&self, x: i32, p: i32) -> PyResult<bool> {
        let value = self.players(py).lock().unwrap().btn(p as u8, x as u8);
        Ok(value)
    }

    def btn2(&self, x: i32) -> PyResult<bool> {
        let value = self.players(py).lock().unwrap().btn2(x);
        Ok(value)
    }

    def btnp(&self, x: i32, p: i32) -> PyResult<bool> {
        let value = self.players(py).lock().unwrap().btnp(p as u8, x as u8);
        Ok(value)
    }

    def btnp2(&self, x: i32) -> PyResult<bool> {
        let value = self.players(py).lock().unwrap().btnp2(x);
        Ok(value)
    }

    def btn_mouse(&self, x: i32) -> PyResult<i32> {
        let value = self.players(py).lock().unwrap().mouse_coordinate(x as u8);
        Ok(value)
    }

    def btn_mouse_state(&self) -> PyResult<u32> {
        let value = self.players(py).lock().unwrap().mouse_state();
        Ok(value)
    }

    def btn_mouse_statep(&self) -> PyResult<u32> {
        let value = self.players(py).lock().unwrap().mouse_state_quick();
        Ok(value)
    }

    });

    // Map
    py_class!(class PX8Map |py| {
    data screen: Arc < Mutex < Screen > >;

    def spr_map(&self, cel_x: i32, cel_y: i32, sx: i32, sy: i32, cel_w: i32, cel_h: i32, layer: u8) -> PyResult<i32> {
        self.screen(py).lock().unwrap().map(cel_x as u32, cel_y as u32,
                                            sx, sy,
                                            cel_w as u32, cel_h as u32,
                                            layer);

        Ok(0)
    }


    def mget(&self, x: i32, y: i32) -> PyResult<u32> {
        let value = self.screen(py).lock().unwrap().mget(x, y);
        Ok(value)
    }

    def mset(&self, x: i32, y: i32, v: u32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().mset(x, y, v);
        Ok(0)
    }

    });

    // Math

    // Memory
    py_class!(class PX8Memory |py| {
    data screen: Arc < Mutex < Screen > >;

    def memcpy(&self, dest_addr: u32, source_addr: u32, len: u32) -> PyResult<u32> {
        self.screen(py).lock().unwrap().memcpy(dest_addr, source_addr, len);
        Ok(0)
    }

    });

    // Noise
    py_class!(class PX8Noise |py| {
    data _noise: Arc < Mutex < Noise > >;
        def noise(&self, x: f64, y: f64, z: f64) -> PyResult<f64> {
            Ok(self._noise(py).lock().unwrap().get(x, y, z))
        }

        def noise_set_seed(&self, seed: u32) -> PyResult<u32> {
            self._noise(py).lock().unwrap().set_seed(seed);
            Ok(0)
        }
    });


    // Others
    py_class!(class PX8Sys |py| {
    data info: Arc < Mutex <Info > >;
    data config: Arc<Mutex<PX8Config>>;

        def show_mouse(&self, value: bool) -> PyResult<u32> {
            self.config(py).lock().unwrap().toggle_mouse(value);
            Ok(0)
        }

        def time(&self) -> PyResult<i64> {
            Ok(self.info(py).lock().unwrap().time())
        }

        def time_sec(&self) -> PyResult<f64> {
            Ok(self.info(py).lock().unwrap().time_sec())
        }
    });

    pub struct PythonPlugin {
        pub mydict: PyDict,
        pub loaded_code: bool,
    }

    impl PythonPlugin {
        pub fn new() -> PythonPlugin {
            let gil = Python::acquire_gil();
            let py = gil.python();

            let d = PyDict::new(py);

            PythonPlugin {
                mydict: d,
                loaded_code: false,
            }
        }


        pub fn load(&mut self,
                    palettes: Arc<Mutex<Palettes>>,
                    players: Arc<Mutex<Players>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    sound: Arc<Mutex<Sound>>,
                    noise: Arc<Mutex<Noise>>,
                    config: Arc<Mutex<PX8Config>>) {
            info!("[PLUGIN][PYTHON] Init plugin");

            let gil = Python::acquire_gil();
            let py = gil.python();

            let px8_graphic_obj = PX8Graphic::create_instance(py, screen.clone()).unwrap();
            self.mydict
                .set_item(py, "px8_graphic", px8_graphic_obj)
                .unwrap();

            let px8_palette_obj = PX8Palette::create_instance(py, palettes.clone()).unwrap();
            self.mydict
                .set_item(py, "px8_palette", px8_palette_obj)
                .unwrap();

            let px8_audio_obj = PX8Audio::create_instance(py, sound.clone()).unwrap();
            self.mydict
                .set_item(py, "px8_audio", px8_audio_obj)
                .unwrap();

            let px8_input_obj = PX8Input::create_instance(py, players.clone()).unwrap();
            self.mydict
                .set_item(py, "px8_input", px8_input_obj)
                .unwrap();

            let px8_map_obj = PX8Map::create_instance(py, screen.clone()).unwrap();
            self.mydict.set_item(py, "px8_map", px8_map_obj).unwrap();

            let px8_sys_obj = PX8Sys::create_instance(py, info.clone(), config.clone()).unwrap();
            self.mydict.set_item(py, "px8_sys", px8_sys_obj).unwrap();

            let px8_mem_obj = PX8Memory::create_instance(py, screen.clone()).unwrap();
            self.mydict.set_item(py, "px8_mem", px8_mem_obj).unwrap();

            let px8_noise_obj = PX8Noise::create_instance(py, noise.clone()).unwrap();
            self.mydict
                .set_item(py, "px8_noise", px8_noise_obj)
                .unwrap();

            py.run(r###"globals()["px8_graphic"] = px8_graphic;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["px8_audio"] = px8_audio;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["px8_palette"] = px8_palette;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["px8_input"] = px8_input;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["px8_map"] = px8_map;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["px8_sys"] = px8_sys;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["px8_mem"] = px8_mem;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["px8_noise"] = px8_noise;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();

            let mut f = File::open("./sys/config/api.py").unwrap();
            let mut data = String::new();
            f.read_to_string(&mut data).unwrap();

            let result = py.run(&data, None, None);
            match result {
                Err(v) => {
                    panic!("[PLUGIN][PYTHON] Failed to load the plugin = {:?}", v);
                }
                Ok(v) => {
                    info!("[PLUGIN][PYTHON] Successfully loaded = {:?}", v);
                }
            }
        }


        pub fn init(&mut self) {
            info!("[PLUGIN][PYTHON] Call INIT");

            if !self.loaded_code {
                return;
            }

            let gil = Python::acquire_gil();
            let py = gil.python();

            let result = py.run(r###"_init()"###, None, Some(&self.mydict));
            info!("[PLUGIN][PYTHON] INIT -> {:?}", result);
        }

        pub fn draw(&mut self) -> bool {
            let mut return_draw_value = true;
            debug!("[PLUGIN][PYTHON] Call DRAW");

            if !self.loaded_code {
                return false;
            }

            let gil = Python::acquire_gil();
            let py = gil.python();

            let result = py.eval(r###"_draw()"###, None, Some(&self.mydict));

            match result {
                Err(v) => {
                    return_draw_value = false;
                    warn!("[PLUGIN][PYTHON] DRAW = {:?}", v);
                }
                Ok(v) => {
                    match v.extract(py) {
                        Ok(draw_value) => {
                            return_draw_value = draw_value;
                        }
                        _ => (),
                    }
                }
            }

            return return_draw_value;
        }

        pub fn update(&mut self) -> bool {
            let mut return_update_value = true;
            debug!("[PLUGIN][PYTHON] Call UPDATE");

            if !self.loaded_code {
                return false;
            }

            let gil = Python::acquire_gil();
            let py = gil.python();

            let result = py.eval(r###"_update()"###, None, Some(&self.mydict));

            match result {
                Err(v) => {
                    return_update_value = false;
                    warn!("[PLUGIN][PYTHON] UPDATE = {:?}", v);
                }
                Ok(v) => {
                    match v.extract(py) {
                        Ok(update_value) => {
                            return_update_value = update_value;
                        }
                        _ => (),
                    }
                }
            }

            return return_update_value;
        }


        pub fn load_code(&mut self, data: String) -> bool {
            info!("[PLUGIN][PYTHON] Load the code");
            let gil = Python::acquire_gil();
            let py = gil.python();


            let result = py.run(&data, None, None);

            match result {
                Ok(_) => {
                    debug!("[PLUGIN][PYTHON] Code loaded successfully");
                    self.loaded_code = true
                }
                Err(err) => {
                    error!("[PLUGIN][PYTHON] Load code error => {:?}", err);
                    self.loaded_code = false
                }
            }

            self.loaded_code
        }
    }
}

#[cfg(not(feature = "cpython"))]
pub mod plugin {
    use std::sync::{Arc, Mutex};

    use config::Players;

    use px8::info::Info;

    use gfx::Screen;
    use px8::Palettes;
    use sound::sound::Sound;
    use px8::noise::Noise;
    use px8::PX8Config;

    pub struct PythonPlugin {}

    impl PythonPlugin {
        pub fn new() -> PythonPlugin {
            PythonPlugin {}
        }


        pub fn load(&mut self,
                    _palettes: Arc<Mutex<Palettes>>,
                    _players: Arc<Mutex<Players>>,
                    _info: Arc<Mutex<Info>>,
                    _screen: Arc<Mutex<Screen>>,
                    _sound: Arc<Mutex<Sound>>,
                    _noise: Arc<Mutex<Noise>>,
                    _config: Arc<Mutex<PX8Config>>) {
            panic!("[PLUGIN][PYTHON] plugin disabled");
        }
        pub fn init(&mut self) {}
        pub fn draw(&mut self) -> bool {
            false
        }
        pub fn update(&mut self) -> bool {
            false
        }
        pub fn load_code(&mut self, _data: String) -> bool {
            false
        }
    }
}
