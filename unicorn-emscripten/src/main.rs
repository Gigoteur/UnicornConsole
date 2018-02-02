pub mod fps;
pub mod frametimes;
pub mod frontend;
pub mod renderer;
pub mod controllers;
pub mod input;

extern crate unicorn;

extern crate sdl2;
extern crate getopts;
extern crate chrono;


#[macro_use]
extern crate log;
extern crate fern;
extern crate time;

use std::env;
use getopts::Options;

use unicorn::gfx;
use unicorn::gfx::Scale;
use unicorn::cartridge::Cartridge;

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
    opts.optflag("c", "check", "check the cartridge");
    opts.optflag("e", "editor", "edit the cartridge");
    opts.optflag("o", "opengl", "enable opengl with SDL");
    opts.optflag("f", "fullscreen", "display in fullscreen");
    opts.optflagopt("d", "dump", "dump the cartridge", "FILE");
    opts.optflagopt("t",
                    "transform",
                    "transform the PNG/Unicorn cartridge in P8",
                    "FILE");
    opts.optflagopt("s", "scale", "scale the display", "VALUE");
    opts.optflagopt("b", "bind", "bind a server on a specific address", "ADDR");
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

    let mut scale = Scale::Scale1x;
    if matches.opt_present("s") {
        let value = matches.opt_str("s").unwrap().parse::<i32>().unwrap();
        match value {
            2 => scale = Scale::Scale2x,
            3 => scale = Scale::Scale3x,
            4 => scale = Scale::Scale4x,
            5 => scale = Scale::Scale5x,
            6 => scale = Scale::Scale6x,
            8 => scale = Scale::Scale8x,
            10 => scale = Scale::Scale10x,
            _ => scale = Scale::Scale1x,
        }
    }

    let fullscreen = matches.opt_present("f");
    let opengl = matches.opt_present("o");

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        let data = include_bytes!("../../unicorn/sys/unicorn.uni");
        let data_final: Vec<u8> = unicorn::unicorn::array_to_vec(data);

        run_cartridge_raw(scale,
                          fullscreen,
                          opengl,
                          "unicorn.uni",
                          data_final,
                          matches.opt_present("e"));
        return;
    };

    if matches.opt_present("c") {
        if input.contains(".uni") {
            match Cartridge::from_unicorn_file(&input) {
                Ok(c) => {
                    println!("{:?}", c);
                }
                Err(e) => panic!(e),
            }
        } else if input.contains(".uc") {
            match Cartridge::from_unicorn_splitted_file(&input) {
                Ok(c) => {
                    println!("{:?}", c);
                }
                Err(e) => panic!(e),
            }
        } else if input.contains(".duc") {
            match Cartridge::from_dunicorn_file(&input) {
                Ok(c) => {
                    println!("{:?}", c);
                }
                Err(e) => panic!(e),
            }
        }
    } else {
        run_cartridge(scale, fullscreen, opengl, &input, matches.opt_present("e"));
    }
}

pub fn run_cartridge(scale: gfx::Scale,
                     fullscreen: bool,
                     opengl: bool,
                     filename: &str,
                     editor: bool) {
    let mut frontend = match frontend::Frontend::init(scale, fullscreen, opengl, false) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend,
    };

    frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_cartridge(filename, editor);
}

pub fn run_cartridge_raw(scale: gfx::Scale,
                         fullscreen: bool,
                         opengl: bool,
                         filename: &str,
                         data: Vec<u8>,
                         editor: bool) {
    let mut frontend = match frontend::Frontend::init(scale, fullscreen, opengl, false) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend,
    };

    frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_cartridge_raw(filename, data, editor);
}

pub fn run_interactive(scale: gfx::Scale, fullscreen: bool, opengl: bool) {
    let mut frontend = match frontend::Frontend::init(scale, fullscreen, opengl, false) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend,
    };

    frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_interactive();
}