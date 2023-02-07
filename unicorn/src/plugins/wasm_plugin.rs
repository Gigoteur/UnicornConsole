pub mod plugin {
    use wasmtime::*;

    use std::{sync::{Arc, Mutex}};
    use std::str;

    use anyhow::{Result, anyhow};
    use log::{info};

    use rand;
    use rand::Rng;

    use contexts::Contexts;
    use core::info::Info;
    use gfx::Screen;
    use crate::core::AudioCommandBuffer;
   
    pub struct WasmContext {
        pub screen_context: Arc<Mutex<Screen>>,
        pub input_context: Arc<Mutex<Contexts>>,
    }

        
    impl WasmContext {
        pub fn new(
            screen_context: Arc<Mutex<Screen>>,
            input_context: Arc<Mutex<Contexts>>,
        ) -> Self {
            Self {
                screen_context:screen_context.clone(),
                input_context: input_context.clone(),
            }
        }
    }

    pub(crate) fn bool_to_u32(val: bool) -> u32 {
        match val {
            true => 1,
            false => 0,
        }
    }
    
    pub struct WasmPlugin {
        engine: Engine,
        store: Option<Store<WasmContext>>,
        init_fn: Option<TypedFunc<(), ()>>,
        draw_fn: Option<TypedFunc<(), ()>>,
        update_fn: Option<TypedFunc<(), ()>>,
    }

    impl WasmPlugin {
        pub fn new() -> WasmPlugin {
            WasmPlugin {
                engine: Engine::default(),
                store: None,
                init_fn: None,
                draw_fn: None,
                update_fn: None,
            }
        }


        pub fn load(&mut self,
                    contexts: Arc<Mutex<Contexts>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    audio: Arc<Mutex<AudioCommandBuffer>>) -> Result<()> {
            info!("[PLUGIN][WASM] Init plugin");

            self.store = Some(Store::new(&self.engine, WasmContext::new(screen, contexts)));
            
            Ok(())
        }

        pub fn bind_all_apis(&mut self, linker: &mut wasmtime::Linker<WasmContext>)  {
            linker.func_wrap("env", "mode_height", |caller: Caller<'_, WasmContext>| {
                return caller.data().screen_context.lock().unwrap().mode_height() as u32;
            }).unwrap();

            linker.func_wrap("env", "mode_width", |caller: Caller<'_, WasmContext>| {
                return caller.data().screen_context.lock().unwrap().mode_width() as u32;
            }).unwrap();

            linker.func_wrap("env", "btnp", |caller: Caller<'_, WasmContext>, x: i32, p:i32| {
                return bool_to_u32(caller.data().input_context.lock().unwrap().input_context.btnp(p as u8, x as u8));
            }).unwrap();

            linker.func_wrap("env", "mouse_x", |caller: Caller<'_, WasmContext>| {
                return caller.data().input_context.lock().unwrap().input_context.btn_mouse(0, 0);
            }).unwrap();

            linker.func_wrap("env", "mouse_y", |caller: Caller<'_, WasmContext>| {
                return caller.data().input_context.lock().unwrap().input_context.btn_mouse(0, 1);
            }).unwrap();

            linker.func_wrap("env", "mouse_left_state", |caller: Caller<'_, WasmContext>, p:i32| {
                return caller.data().input_context.lock().unwrap().input_context.btn_mouse_state(p as u8) & 0x000000FF;
            }).unwrap();
            

            linker.func_wrap("env", "mouse_left_statep", |caller: Caller<'_, WasmContext>, p:i32| {
                return caller.data().input_context.lock().unwrap().input_context.btn_mouse_statep(p as u8) & 0x000000FF;
            }).unwrap();
            
            linker.func_wrap("env", "cls", |caller: Caller<'_, WasmContext>, col: i32| {
                caller.data().screen_context.lock().unwrap().cls(col as i8);
            }).unwrap();

            linker.func_wrap("env", "pset", |caller: Caller<'_, WasmContext>, x: i32, y:i32, col: i32| {
                caller.data().screen_context.lock().unwrap().pset(x, y, col);
            }).unwrap();

            linker.func_wrap("env", "pset_rgba", |caller: Caller<'_, WasmContext>, x: i32, y:i32, r: i32, g: i32, b: i32, a: i32| {
                caller.data().screen_context.lock().unwrap().pset_rgba(x, y, r as u8, g as u8, b as u8, a as u8);
            }).unwrap();

            linker.func_wrap("env", "circ", |caller: Caller<'_, WasmContext>, x: i32, y:i32, r: i32, col: i32| {
                caller.data().screen_context.lock().unwrap().circ(x, y, r, col);
            }).unwrap();

            linker.func_wrap("env", "circfill", |caller: Caller<'_, WasmContext>, x: i32, y:i32, r: i32, col: i32| {
                caller.data().screen_context.lock().unwrap().circfill(x, y, r, col);
            }).unwrap();

            linker.func_wrap("env", "debug_print", |mut caller: Caller<'_, WasmContext>, text_ptr: i32, len: i32| {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => anyhow::bail!("failed to find host memory"),
                };
                let data = mem.data(&caller)
                .get(text_ptr as u32 as usize..)
                .and_then(|arr| arr.get(..len as u32 as usize));

                let string = match data {
                    Some(data) => match str::from_utf8(data) {
                        Ok(s) => s,
                        Err(_) => anyhow::bail!("invalid utf-8"),
                    },
                    None => anyhow::bail!("pointer/length out of bounds"),
                };

                info!("[CART] {}", string.to_string());
                Ok(())

            }).unwrap();

            linker.func_wrap("env", "print", |mut caller: Caller<'_, WasmContext>, text_ptr: i32, len: i32, x: i32, y:i32, col: i32| {
                let mem = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => anyhow::bail!("failed to find host memory"),
                };
                let data = mem.data(&caller)
                .get(text_ptr as u32 as usize..)
                .and_then(|arr| arr.get(..len as u32 as usize));

                let string = match data {
                    Some(data) => match str::from_utf8(data) {
                        Ok(s) => s,
                        Err(_) => anyhow::bail!("invalid utf-8"),
                    },
                    None => anyhow::bail!("pointer/length out of bounds"),
                };

                caller.data().screen_context.lock().unwrap().print(string.to_string(), x, y, col);
                Ok(())

            }).unwrap();
            
            linker.func_wrap("env", "rnd_range", |_caller: Caller<'_, WasmContext>, x: i32, y:i32| {
                return rand::thread_rng().gen_range(x as f64..y as f64) as i32;
            }).unwrap();

            linker.func_wrap("env", "frnd", |_caller: Caller<'_, WasmContext>| {
                return rand::thread_rng().gen::<f32>();
            }).unwrap();


            linker.func_wrap("env", "spr", |caller: Caller<'_, WasmContext>, n: u32, x: i32, y: i32, w: i32, h: i32, flip_x: i32, flip_y: i32, angle: f32, zoom: f32, dynamic: i32| {
                caller.data().screen_context.lock().unwrap().spr(n, x, y, w, h, flip_x == 1, flip_y == 1, angle.into(), zoom.into(), dynamic == 1);
            }).unwrap();
        }

        pub fn load_code(&mut self, data: &[u8]) -> Result<()> {
            info!("[PLUGIN][WASM] Load the code {:?}", data.len());

            let mut linker = Linker::new(&self.engine);
            self.bind_all_apis(&mut linker);



            let mut store = self.store.as_mut().unwrap();

            let add = Func::wrap(&mut store, |a: i32, b: i32| -> i32 { a + b });

            let module = Module::new(&self.engine, data)?;
            let instance = linker.instantiate(&mut store, &module)?;

            self.init_fn = match instance.get_typed_func(&mut store, "init") {
                Ok(init_fn) => Some(init_fn),
                Err(e) => {
                    return Err(anyhow!("[PLUGIN][WASM] Failed to find the init function = {:?}", e));
                }
            };

            self.draw_fn = match instance.get_typed_func(&mut store, "draw") {
                Ok(draw_fn) => Some(draw_fn),
                Err(e) => {
                    return Err(anyhow!("[PLUGIN][WASM] Failed to find the draw function = {:?}", e));
                }
            };

            self.update_fn = match instance.get_typed_func(&mut store, "update") {
                Ok(update_fn) => Some(update_fn),
                Err(e) => {
                    return Err(anyhow!("[PLUGIN][WASM] Failed to find the update function = {:?}", e));
                }
            };


            Ok(())
        }

        pub fn init(&mut self) -> Result<()> {
            let mut store = self.store.as_mut().unwrap();

            self.init_fn.unwrap().call(&mut store, ());
            Ok(())
        }

        pub fn draw(&mut self) -> Result<()> {
            let mut store = self.store.as_mut().unwrap();

            self.draw_fn.unwrap().call(&mut store, ());
            Ok(())
        }

        pub fn update(&mut self) -> Result<()> {
            let mut store = self.store.as_mut().unwrap();

            self.update_fn.unwrap().call(&mut store, ());
            Ok(())
        }

    }
}
