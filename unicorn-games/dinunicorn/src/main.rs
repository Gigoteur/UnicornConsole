extern crate unicorn_sdl;

extern crate unicorn;

extern crate getopts;

extern crate log;
extern crate fern;
extern crate time;

use std::env;
use getopts::Options;

use unicorn::gfx;
use unicorn::cartridge::Cartridge;
use unicorn::unicorn::info;
use unicorn::unicorn::RustPlugin;
use unicorn::config::Players;

pub struct Dino {
}

impl Dino {
    pub fn new() -> Dino {
        Dino {
        }
    }
}

impl RustPlugin for Dino {
    fn init(&mut self, screen: &mut gfx::Screen) -> f64 {
        match Cartridge::from_dunicorn_string(include_bytes!("../assets/dino.duc").to_vec()) {
            Ok(c) => screen.set_sprites(c.gfx.sprites),
            Err(e) => panic!("Impossible to load the assets {:?}", e),
        }

        0.0
    }

    fn update(&mut self, _players: &mut Players) -> f64 {
        0.0
    }

    fn draw(&mut self, screen: &mut gfx::Screen, _info: &mut info::Info) -> f64 {
        screen.rectfill(0, 0, 100, 100, 3);
        //screen.cls(1);
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

    let dino = Dino::new();

    let mut frontend =
        match unicorn_sdl::frontend::Frontend::init(unicorn::gfx::Scale::Scale4x, false, true, true) {
            Err(error) => panic!("{:?}", error),
            Ok(frontend) => frontend,
        };

    frontend.uc.register(dino);
    frontend.start();

    #[cfg(not(target_os = "emscripten"))]
    frontend.init_controllers("../../unicorn-sdl/sys/config/gamecontrollerdb.txt".to_string());
    
    frontend.run_native_cartridge();
}