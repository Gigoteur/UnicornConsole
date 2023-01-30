pub mod plugin {
    use std::sync::{Arc, Mutex};
    use anyhow::{Result, anyhow};

    use contexts::Contexts;
    use core::info::Info;
    use gfx::Screen;
    use crate::core::AudioCommandBuffer;
   
    pub struct RPythonPlugin {
    }

    impl RPythonPlugin {
        pub fn new() -> RPythonPlugin {
            RPythonPlugin {

            }
        }


        pub fn load(&mut self,
                    contexts: Arc<Mutex<Contexts>>,
                    info: Arc<Mutex<Info>>,
                    screen: Arc<Mutex<Screen>>,
                    audio: Arc<Mutex<AudioCommandBuffer>>) -> Result<()> {
            Ok(())
        }

        pub fn load_code(&mut self, _data: String) -> Result<()> {
            Ok(())
        }

        pub fn init(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][RPYTHON] [init] python is not compiled"))
        }

        pub fn draw(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][RPYTHON] [draw] python is not compiled"))
        }

        pub fn update(&mut self) -> Result<()> {
            Err(anyhow!("[PLUGIN][RPYTHON] [update] python is not compiled"))
        }

    }
}
