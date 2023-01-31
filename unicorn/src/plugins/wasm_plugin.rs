pub mod plugin {
    use wasmtime::*;

    use std::{sync::{Arc, Mutex}, borrow::Borrow};
    use anyhow::{Result, anyhow};
    use log::{error, info, debug};

    use contexts::Contexts;
    use core::info::Info;
    use gfx::Screen;
    use crate::core::AudioCommandBuffer;
   
    pub struct WasmPlugin {
        engine: Engine,
        store: Option<Store<Arc<Mutex<Screen>>>>,
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

            self.store = Some(Store::new(&self.engine, screen.clone()));

            Ok(())
        }

        pub fn load_code(&mut self, data: &[u8]) -> Result<()> {
            info!("[PLUGIN][WASM] Load the code {:?}", data.len());

            let mut linker = Linker::new(&self.engine);

            linker.func_wrap("env", "cls", |caller: Caller<'_, Arc<Mutex<Screen>>>, col: i32| {
                println!("Got {} from WebAssembly", col);
                caller.data().lock().unwrap().cls(col as i8);
            })?;

            linker.func_wrap("env", "circ", |caller: Caller<'_, Arc<Mutex<Screen>>>, x: i32, y:i32, r: i32, col: i32| {
                println!("Got {} {} {} {} from WebAssembly", x, y, r, col);
                caller.data().lock().unwrap().circ(x, y, r, col);
            })?;

            let mut store = self.store.as_mut().unwrap();

            let module = Module::new(&self.engine, data)?;
            let instance = linker.instantiate(&mut store, &module)?;

            self.init_fn = match instance.get_typed_func(&mut store, "init") {
                Ok(init_fn) => Some(init_fn),
                Err(e) => {
                    return Err(anyhow!("[PLUGIN][RHAI] Failed to find the init function = {:?}", e));
                }
            };

            self.draw_fn = match instance.get_typed_func(&mut store, "draw") {
                Ok(draw_fn) => Some(draw_fn),
                Err(e) => {
                    return Err(anyhow!("[PLUGIN][RHAI] Failed to find the draw function = {:?}", e));
                }
            };

            self.update_fn = match instance.get_typed_func(&mut store, "update") {
                Ok(update_fn) => Some(update_fn),
                Err(e) => {
                    return Err(anyhow!("[PLUGIN][RHAI] Failed to find the update function = {:?}", e));
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
