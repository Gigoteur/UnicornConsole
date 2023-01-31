pub mod plugin {
    use rhai::{Engine, EvalAltResult, Scope, AST};

    use std::{sync::{Arc, Mutex}, borrow::Borrow};
    use anyhow::{Result, anyhow};
    use log::{error, info, debug};

    use contexts::Contexts;
    use core::info::Info;
    use gfx::Screen;
    use crate::core::AudioCommandBuffer;
   
    pub struct RhaiPlugin {
        engine: Engine,
        ast: Option<AST>,
        scope: Scope<'static>,
    }

    impl RhaiPlugin {
        pub fn new() -> RhaiPlugin {
            RhaiPlugin {
                engine: Engine::new(),
                ast: None,
                scope: Scope::new(),
            }
        }


        pub fn load(&mut self,
                    contexts: Arc<Mutex<Contexts>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    audio: Arc<Mutex<AudioCommandBuffer>>) -> Result<()> {
            info!("[PLUGIN][RHAI] Init plugin");

            let s = screen.clone();

            self.engine.register_fn("cls", move |value: i64| {
                s.lock().unwrap().cls(value as i8);
            });
            
            let s = screen.clone();
            self.engine.register_fn("circfill", move |x: i64, y: i64, r: i64, col: i64| -> Result<_, Box<EvalAltResult>> {
                println!("CIRCFILL FROM RHAI");
                s.lock().unwrap().circfill(x as i32, y as i32, r as i32, col as i32);
                
                Ok(())
            });

            Ok(())
        }

        pub fn load_code(&mut self, data: String) -> Result<()> {
            info!("[PLUGIN][RHAI] Load the code {:?}", data);
            self.ast = Some(self.engine.compile_with_scope(&mut self.scope, &data).unwrap());

            match self.engine.run_ast_with_scope(&mut self.scope, &self.ast.as_mut().unwrap()) {
                Ok(_) => return Ok(()),
                Err(v) => return Err(anyhow!("[PLUGIN][RHAI] Failed to load the code = {:?}", v)),
            }
        }

        pub fn init(&mut self) -> Result<()> {
            let result = self.engine.call_fn::<bool>(&mut self.scope, &self.ast.as_mut().unwrap(), "_init", ());

            Ok(())
        }

        pub fn draw(&mut self) -> Result<()> {
            let result = self.engine.call_fn::<bool>(&mut self.scope, &self.ast.as_mut().unwrap(), "_draw", ());

            Ok(())
        }

        pub fn update(&mut self) -> Result<()> {
            let result = self.engine.call_fn::<bool>(&mut self.scope, &self.ast.as_mut().unwrap(), "_update", ());

            Ok(())
        }

    }
}
