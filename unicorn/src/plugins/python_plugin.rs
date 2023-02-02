#[cfg(feature = "cpython")]
#[allow(unused_variables)]
pub mod plugin {
    use log::{error, info, debug};

    use cpython::*;

    use std::sync::{Arc, Mutex};
    use anyhow::{Result, anyhow};

    use contexts::Contexts;

    use core::info::Info;
    use gfx::Screen;
    use crate::core::AudioSyncCommand;
    use crate::core::AudioCommandBuffer;

    // Graphics
    py_class!(class UnicornGraphic |py| {
    data screen: Arc<Mutex<Screen>>;

    // Cart Data
    def set_color_palette(&self, color:u32, r: u8, g: u8, b: u8) -> PyResult<i32> {
        self.screen(py).lock().unwrap().set_palette_color(color, r, g, b);
        Ok(0)
    }

    def reset_palette(&self) -> PyResult<i32> {
        self.screen(py).lock().unwrap()._reset_palettes();
        Ok(0)
    }
 
    def switch_palette(&self, name: String) -> PyResult<i32> {
        self.screen(py).lock().unwrap().switch_palette(name);
        Ok(0)
    }

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

    def cls(&self, value: i8) -> PyResult<i32> {
        self.screen(py).lock().unwrap().cls(value);
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

    def fillp(&self, pat: u32, transparent: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().fillp(pat, transparent);
        Ok(0)
    }

    def line(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32) -> PyResult<i32> {
        self.screen(py).lock().unwrap().line(x1, y1, x2, y2, color);
        Ok(0)
    }

    def mode_width(&self) -> PyResult<usize> {
        Ok(self.screen(py).lock().unwrap().mode_width())
    }

    def mode_height(&self) -> PyResult<usize> {
        Ok(self.screen(py).lock().unwrap().mode_height())
    }

    def palt(&self, c: i32, t: u8) -> PyResult<i32> {
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
        Ok(self.screen(py).lock().unwrap().sget(x, y))
    }

    def spr_reg(&self, n: i64, d: PyList, width: u32, height: u32) -> PyResult<i64> {
        let mut data: Vec<u8> = Vec::new();

        for lx in d.iter(py) {
            data.push(lx.extract::<u8>(py).unwrap());
        }


        Ok(self.screen(py).lock().unwrap().spr_reg(n,
                                                   data,
                                                   width,
                                                   height))
    }

    def spr(&self, n: i32, x: i32, y: i32, w: i32, h: i32, flip_x: bool, flip_y: bool, angle: f64, zoom: f64, dynamic: bool) -> PyResult<i32> {
        self.screen(py).lock().unwrap().spr(n as u32,
                                            x,
                                            y,
                                            w,
                                            h,
                                            flip_x,
                                            flip_y,
                                            angle,
                                            zoom,
                                            dynamic);

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

    def sspr_rotazoom(&self, idx_sprite: i32, sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32, angle: f64, zoom: f64, flip_x: bool, flip_y: bool) -> PyResult<PyList> {
        let (dw, dh) = self.screen(py).lock().unwrap().sspr_rotazoom(idx_sprite,
                                                                     sx as u32,
                                                                     sy as u32,
                                                                     sw as u32,
                                                                     sh as u32,
                                                                     dx as i32,
                                                                     dy as i32,
                                                                     angle,
                                                                     zoom,
                                                                     flip_x,
                                                                     flip_y);
        
        let v = vec![dw, dh];
        let ret = v.to_py_object(py);
        Ok(ret)
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
    py_class!(class UnicornInput |py| {
    data contexts: Arc<Mutex<Contexts>>;

    def btn(&self, x: i32, p: i32) -> PyResult<bool> {
        let value = self.contexts(py).lock().unwrap().input_context.btn(p as u8, x as u8);
        Ok(value)
    }

    def btnp(&self, x: i32, p: i32) -> PyResult<bool> {
        let value = self.contexts(py).lock().unwrap().input_context.btnp(p as u8, x as u8);
        Ok(value)
    }

    def btn_mouse(&self, x: i32, p: i32) -> PyResult<i32> {
        let value = self.contexts(py).lock().unwrap().input_context.btn_mouse(p as u8, x as u8);
        Ok(value)
    }

    def btn_mouse_state(&self, p: i32) -> PyResult<u32> {
        let value = self.contexts(py).lock().unwrap().input_context.btn_mouse_state(p as u8);
        Ok(value)
    }

    def btn_mouse_statep(&self, p: i32) -> PyResult<u32> {
        let value = self.contexts(py).lock().unwrap().input_context.btn_mouse_statep(p as u8);
        Ok(value)
    }

    });

    // Map
    py_class!(class UnicornMap |py| {
    data screen: Arc < Mutex < Screen > >;

    def mapdraw(&self, cel_x: i32, cel_y: i32, sx: i32, sy: i32, cel_w: i32, cel_h: i32, layer: u8) -> PyResult<i32> {
        self.screen(py).lock().unwrap().mapdraw(cel_x as u32, cel_y as u32,
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

    // Info
    py_class!(class UnicornInfo |py| {
        data info: Arc < Mutex < Info > >;

        def time(&self) -> PyResult<u64> {
            let value = self.info(py).lock().unwrap().time();
            Ok(value)
        }

        def mtime(&self) -> PyResult<u64> {
            let value = self.info(py).lock().unwrap().mtime() as u64;
            Ok(value)
        }  

        def utime(&self) -> PyResult<u64> {
            let value = self.info(py).lock().unwrap().utime() as u64;
            Ok(value)
        }  
    });

    // Audio
    py_class!(class UnicornAudio |py| {
        data audio: Arc<Mutex<AudioCommandBuffer>>;

        def play_note(&self, note_id: u32, instrument_id: u32, channel: u32) -> PyResult<u32> {
            self.audio(py).lock().unwrap().push(AudioSyncCommand::PressedKey {note_index: note_id as usize, instrument_index: instrument_id as usize, channel: channel as usize});
            Ok(0)
        }

        
        def trigger_note(&self, note_id: u32, instrument_id: u32) -> PyResult<u32> {
            self.audio(py).lock().unwrap().push(AudioSyncCommand::TriggerNote {note_index: note_id as usize, instrument_index: instrument_id as usize});
            Ok(0)
        }

        def play_phrase(&self, phrase_index: u32, target_bpm: f32) -> PyResult<u32> {
            self.audio(py).lock().unwrap().push(AudioSyncCommand::PlayPhrase {phrase_index: phrase_index as usize, target_bpm: target_bpm});
            Ok(0)
        }

        def new_phrase(&self) -> PyResult<u32> {
            Ok(0)
        }

        def add_phrase(&self) -> PyResult<u32> {
            Ok(0)
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
                    contexts: Arc<Mutex<Contexts>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    audio: Arc<Mutex<AudioCommandBuffer>>) -> Result<()> {
            info!("[PLUGIN][PYTHON] Init plugin");

            let gil = Python::acquire_gil();
            let py = gil.python();

            let unicorn_graphic_obj = UnicornGraphic::create_instance(py, screen.clone()).unwrap();
            self.mydict.set_item(py, "unicorn_graphic", unicorn_graphic_obj).unwrap();

            let unicorn_input_obj = UnicornInput::create_instance(py, contexts.clone()).unwrap();
            self.mydict.set_item(py, "unicorn_input", unicorn_input_obj).unwrap();

            let unicorn_map_obj = UnicornMap::create_instance(py, screen.clone()).unwrap();
            self.mydict.set_item(py, "unicorn_map", unicorn_map_obj).unwrap();

            let unicorn_info_obj = UnicornInfo::create_instance(py, info.clone()).unwrap();
            self.mydict.set_item(py, "unicorn_info", unicorn_info_obj).unwrap();  

            let unicorn_audio_obj = UnicornAudio::create_instance(py, audio.clone()).unwrap();
            self.mydict.set_item(py, "unicorn_audio", unicorn_audio_obj).unwrap();  


            py.run(r###"globals()["unicorn_graphic"] = unicorn_graphic;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["unicorn_input"] = unicorn_input;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["unicorn_map"] = unicorn_map;"###,
                     None,
                     Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["unicorn_info"] = unicorn_info;"###,
                None,
                Some(&self.mydict))
                .unwrap();
            py.run(r###"globals()["unicorn_audio"] = unicorn_audio;"###,
                None,
                Some(&self.mydict))
                .unwrap();

            let data = include_str!("python/api.py").to_string();

            let result = py.run(&data, None, None);
            match result {
                Err(v) => {
                    return Err(anyhow!("[PLUGIN][PYTHON] Failed to load the plugin = {:?}", v));
                }
                Ok(v) => {
                    info!("[PLUGIN][PYTHON] Successfully loaded = {:?}", v);
                    self.loaded_code = true;
                }
            }

            Ok(())
        }


        pub fn init(&mut self) -> Result<()> {
            info!("[PLUGIN][PYTHON] Call INIT");

            if !self.loaded_code {
                return Err(anyhow!("[PLUGIN][Python] [draw] code is not loaded"));
            }

            let gil = Python::acquire_gil();
            let py = gil.python();

            let result = py.run(r###"_init()"###, None, Some(&self.mydict));
            info!("[PLUGIN][PYTHON] INIT -> {:?}", result);

            Ok(())
        }

        pub fn draw(&mut self) -> Result<()> {
            if self.loaded_code {
                let gil = Python::acquire_gil();
                let py = gil.python();

                match py.eval(r###"_draw()"###, None, Some(&self.mydict)) {
                    Ok(v) => return Ok(()),
                    Err(v) => return Err(anyhow!("[PLUGIN][Python] [draw] impossible to call _draw {:?}", v)),
                }
            }

            Err(anyhow!("[PLUGIN][Python] [draw] impossible to load the code"))
        }

        pub fn update(&mut self) -> Result<()> {
            if self.loaded_code {
                let gil = Python::acquire_gil();
                let py = gil.python();

                match py.eval(r###"_update()"###, None, Some(&self.mydict)) {
                    Ok(v) => return Ok(()),
                    Err(v) => return Err(anyhow!("[PLUGIN][Python] [update] impossible to call _update {:?}", v)),
                }
            }

            Err(anyhow!("[PLUGIN][Python] [update] impossible to load the code"))
        }


        pub fn load_code(&mut self, data: String)  -> Result<()> {
            info!("[PLUGIN][PYTHON] Load the code");
            let gil = Python::acquire_gil();
            let py = gil.python();

            let result = py.run(&data, None, None);

            match result {
                Ok(_) => {
                    info!("[PLUGIN][PYTHON] Code loaded successfully");
                    Ok(())
                }
                Err(err) => {
                    Err(anyhow!("[PLUGIN][Python] [update] impossible to load the code"))
                }
            }
        }
    }
}

#[cfg(not(feature = "cpython"))]
pub mod plugin {
    use log::{error};

    use std::sync::{Arc, Mutex};
    use anyhow::{Result, anyhow};

    use contexts::Contexts;

    use core::info::Info;

    use gfx::Screen;

    use crate::core::AudioCommandBuffer;

    #[derive(Debug)]
    pub struct PythonPlugin {}

    impl PythonPlugin {
        pub fn new() -> PythonPlugin {
            PythonPlugin {}
        }


        pub fn load(&mut self,
                    _contexts: Arc<Mutex<Contexts>>,
                    _info: Arc<Mutex<Info>>,
                    _screen: Arc<Mutex<Screen>>,
                    _audio: Arc<Mutex<AudioCommandBuffer>>) -> Result<()> {
                        Err(anyhow!("[PLUGIN][PYTHON] plugin disabled"))
        }
        pub fn load_code(&mut self, _data: String) -> Result<()> {
            Err(anyhow!("[PLUGIN][PYTHON] plugin disabled"))
        }
        pub fn init(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][PYTHON] plugin disabled"))
        }
        pub fn draw(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][PYTHON] plugin disabled"))
        }
        pub fn update(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][PYTHON] plugin disabled"))
        }
    }
}
