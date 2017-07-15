pub mod plugin {
    use std::sync::{Arc, Mutex};

    use config::Players;

    use px8::info::Info;

    use gfx::Screen;
    use px8::Palettes;
    use sound::sound::Sound;
    use px8::noise::Noise;
    use px8::PX8Config;

    pub struct DyonPlugin {
    }

    impl DyonPlugin {
        pub fn new() -> DyonPlugin {
            DyonPlugin {
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
            panic!("[PLUGIN][DYON] plugin disabled");
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