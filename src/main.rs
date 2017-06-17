#[cfg(feature = "lua")]
extern crate lua;

#[cfg(feature = "cpython")]
#[macro_use]
extern crate cpython;

#[cfg(feature = "portaudio")]
#[macro_use]
extern crate portaudio;

extern crate sdl2;
extern crate getopts;

extern crate nalgebra;

extern crate image;
extern crate gif;

extern crate regex;
extern crate png;
extern crate byteorder;
extern crate rand;
extern crate time;
extern crate chrono;
extern crate libc;
extern crate num;
extern crate glob;

extern crate rusttype;

#[macro_use]
extern crate log;
extern crate fern;

#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

extern crate num_traits;
extern crate noise;

use std::env;
use getopts::Options;

// Local stuff

#[macro_use]
mod config;

mod frontend;
mod renderer;
mod gfx;
mod px8;
mod plugins;
mod cartridge;
mod sound;

use gfx::Scale;
use cartridge::Cartridge;

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
                    "transform the PNG/PX8 cartridge in P8",
                    "FILE");
    opts.optflagopt("s", "scale", "scale the display", "VALUE");
    opts.optflagopt("b", "bind", "bind a server on a specific address", "ADDR");
    opts.optflagopt("m", "mode", "Switch the compatibility mode", "MODE");
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

    let mut mode = px8::PX8Mode::PX8;
    if matches.opt_present("m") {
        let mode_str = matches.opt_str("m").unwrap();
        if mode_str == "pico8" {
            mode = px8::PX8Mode::PICO8;
        }
    }

    let mut scale = Scale::Scale4x;
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
        run_interactive(scale, fullscreen, opengl);
        return;
    };

    if matches.opt_present("c") {
        if input.contains(".png") {
            match Cartridge::from_png_file(&input) {
                Ok(mut c) => {
                    println!("{:?}", c);

                    if matches.opt_present("d") {
                        c.dump(&matches.opt_str("d").unwrap());
                    }
                }
                Err(e) => panic!(e),
            }
        } else if input.contains(".p8") {
            match Cartridge::from_p8_file(&input) {
                Ok(mut c) => {
                    println!("{:?}", c);

                    if matches.opt_present("d") {
                        c.set_mode(mode == px8::PX8Mode::PICO8);
                        c.dump(&matches.opt_str("d").unwrap());
                    }
                }
                Err(e) => panic!(e),
            }
        } else if input.contains(".px8") {
            match Cartridge::from_px8_file(&input) {
                Ok(c) => {
                    println!("{:?}", c);
                }
                Err(e) => panic!(e),
            }
        }
    } else if matches.opt_present("t") {
        if input.contains(".png") {
            match Cartridge::from_png_file(&input) {
                Ok(mut c) => {
                    println!("{:?}", c);

                    c.save_in_p8(&matches.opt_str("t").unwrap());
                }
                Err(e) => panic!(e),
            }
        }
    } else {
        run_cartridge(scale,
                      fullscreen,
                      opengl,
                      &input,
                      matches.opt_present("e"),
                      mode);
    }
}

pub fn run_cartridge(scale: gfx::Scale,
                     fullscreen: bool,
                     opengl: bool,
                     filename: &str,
                     editor: bool,
                     mode: px8::PX8Mode) {
    let mut frontend = match frontend::Frontend::init(scale, fullscreen, opengl, false) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend,
    };

    frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_cartridge(filename, editor, mode);
}


pub fn run_interactive(scale: gfx::Scale, fullscreen: bool, opengl: bool) {
    let mut frontend = match frontend::Frontend::init(scale, fullscreen, opengl, false) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend,
    };

    frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_interactive();
}
