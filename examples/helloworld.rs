extern crate px8;
extern crate sdl2;
extern crate time;
extern crate rand;
#[macro_use]
extern crate log;
extern crate fern;

use std::sync::{Arc, Mutex};

use px8::px8::math;
use px8::frontend;
use px8::gfx;
use px8::cartridge;
use px8::px8::RustPlugin;
use px8::config::Players;

pub struct HelloWorld {
    pub sprite_filename: String,
    pub t: i32,
}

impl HelloWorld {
    pub fn new(sprite_filename: String) -> HelloWorld {
        HelloWorld {
            sprite_filename: sprite_filename,
            t: 0,
        }
    }
}

impl RustPlugin for HelloWorld {
    fn init(&mut self, screen: &mut gfx::Screen) -> f64 {
        match cartridge::Cartridge::parse(&self.sprite_filename, false) {
            Ok(c) => screen.set_sprites(c.gfx.sprites),
            Err(e) => panic!("Impossible to load the assets {:?}", e),
        }

        0.0
    }

    fn update(&mut self, _players: &mut Players) -> f64 {
        debug!("HelloWorld update");

        self.t += 1;

        0.0
    }

    fn draw(&mut self, screen: &mut gfx::Screen) -> f64 {
        debug!("HelloWorld draw");

        screen.cls();

        for i in 1..12 {
            for j0 in 0..7 {
                let j: i32 = 7 - j0;
                let col: i32 = 7 + j - 1;
                let t1: i32 = self.t + i * 4 - j * 2;

                let x: i32 = (math::cos(j0 as f64) * 5.0).floor() as i32;
                let y: i32 = ((38 + j) as f64 + math::cos((t1 as f64 / 50.0) as f64) * 5.0)
                    .floor() as i32;

                screen.pal(7, col);
                screen.spr(16 + i as u32, 8 + i * 8 + x, y, 1, 1, false, false);
            }
        }

        0.0
    }
}


fn main() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
                             format!("[{}][{}] {}",
                                     time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(),
                                     level,
                                     msg)
                         }),
        output: vec![fern::OutputConfig::stdout(),
                     fern::OutputConfig::file("output.log")],
        level: log::LogLevelFilter::Trace,
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Info) {
        panic!("Failed to initialize global logger: {}", e);
    }


    let helloworld_example = HelloWorld::new("./examples/helloworld/helloworld.dpx8".to_string());

    let mut frontend =
        match frontend::Frontend::init(px8::gfx::Scale::Scale4x, false, true, true) {
            Err(error) => panic!("{:?}", error),
            Ok(frontend) => frontend,
        };

    frontend.px8.register(helloworld_example);
    frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_native_cartridge();
}
