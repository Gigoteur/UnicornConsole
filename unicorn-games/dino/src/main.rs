extern crate unicorn_sdl;

extern crate unicorn;

extern crate getopts;

extern crate log;
extern crate fern;
extern crate time;

use std::env;
use getopts::Options;

use unicorn::gfx;
use unicorn::gfx::Scale;
use unicorn::cartridge::Cartridge;

pub struct Dino {
    pub sprite_filename: String,
}

impl Dino {
    pub fn new(sprite_filename: String) -> Dino {
        Dino {
            sprite_filename: sprite_filename,
        }
    }
}

impl RustPlugin for Dino {
    fn init(&mut self, screen: &mut gfx::Screen) -> f64 {
        match cartridge::Cartridge::parse(&self.sprite_filename, false) {
            Ok(c) => screen.set_sprites(c.gfx.sprites),
            Err(e) => panic!("Impossible to load the assets {:?}", e),
        }

        0.0
    }

    fn update(&mut self, players: &mut Players) -> f64 {
        0.0
    }

    fn draw(&mut self, screen: &mut gfx::Screen, info: &mut info::Info) -> f64 {
        screen.cls();
        0.0
    }
}


fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
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

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("v", "verbose", "Debug mode level");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
    }

    if matches.opt_present("v") {
        if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Debug) {
            panic!("Failed to initialize global logger: {}", e);
        }
    } else {
        if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Info) {
            panic!("Failed to initialize global logger: {}", e);
        }
    }

    let dino = Dino::new("./assets/dino.duc".to_string());

    let mut frontend =
        match unicorn::frontend::Frontend::init(unicorn::gfx::Scale::Scale4x, false, true, true) {
            Err(error) => panic!("{:?}", error),
            Ok(frontend) => frontend,
        };

    frontend.unicorn.register(dino);
    frontend.start("../unicorn_sdl/sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_native_cartridge();
}