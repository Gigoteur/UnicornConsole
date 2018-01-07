extern crate px8;
extern crate sdl2;
extern crate time;
extern crate rand;
extern crate log;
extern crate fern;

use rand::Rng;

use px8::frontend;
use px8::gfx;
use px8::px8::info;
use px8::cartridge;
use px8::px8::RustPlugin;
use px8::config::Players;


pub struct Ghost {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub color: u32,
}

impl Ghost {
    pub fn new() -> Ghost {
        Ghost {
            x: rand::thread_rng().gen_range(10., 128.),
            y: rand::thread_rng().gen_range(10., 128.),
            vx: rand::thread_rng().gen_range(-10., 10.) / 10.0,
            vy: rand::thread_rng().gen_range(-10., 10.) / 10.0,
            color: rand::thread_rng().gen_range(0, 16),
        }
    }

    pub fn update(&mut self, screen: &mut gfx::Screen) {
        self.x = self.x + self.vx;
        self.y = self.y + self.vy;

        if self.x < 0. || self.x > 128. {
            self.vx = -self.vx;
        }

        if self.y > 128. {
            self.y = 128.;
            self.vy = -(rand::thread_rng().gen_range(0., 100.) / 25.0);
        }


        screen.spr(self.color, self.x as i32, self.y as i32, 1, 1, false, false);
        self.vy = self.vy + 0.05;
    }
}

pub struct GhostMark {
    pub sprite_filename: String,
    pub sprites: Vec<Ghost>,
}

impl GhostMark {
    pub fn new(sprite_filename: String) -> GhostMark {
        GhostMark {
            sprite_filename: sprite_filename,
            sprites: Vec::new(),
        }
    }
}

impl RustPlugin for GhostMark {
    fn init(&mut self, screen: &mut gfx::Screen) -> f64 {
        match cartridge::Cartridge::parse(&self.sprite_filename, false) {
            Ok(c) => screen.set_sprites(c.gfx.sprites),
            Err(e) => panic!("Impossible to load the assets {:?}", e),
        }

        for _ in 0..500 {
            self.sprites.push(Ghost::new());
        }

        0.0
    }

    fn update(&mut self, players: &mut Players) -> f64 {
        if players.btnp(0, 5) {
            for _ in 0..500 {
                self.sprites.push(Ghost::new());
            }
        }

        0.0
    }

    fn draw(&mut self, screen: &mut gfx::Screen, info: &mut info::Info) -> f64 {
        screen.cls();

        let stime = info.time();

        for ghost in &mut self.sprites {
            ghost.update(screen);
        }

        let etime = info.time();

        screen.print(format!("dots {:?}", self.sprites.len()), 8, 0, 7);
        screen.print(format!("Time : {:?}", etime - stime), 8, 8, 7);
        screen.print(format!("Time : {:?}", (etime - stime) as f64 / 16.67),
                     8,
                     16,
                     7);

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
        output: vec![fern::OutputConfig::stdout()],
        level: log::LogLevelFilter::Trace,
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Info) {
        panic!("Failed to initialize global logger: {}", e);
    }


    let ghostmark_example = GhostMark::new("./examples/ghostmark.dpx8".to_string());

    let mut frontend =
        match frontend::Frontend::init(px8::gfx::Scale::Scale4x, false, true, true) {
            Err(error) => panic!("{:?}", error),
            Ok(frontend) => frontend,
        };

    frontend.px8.register(ghostmark_example);
    frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_native_cartridge();
}
