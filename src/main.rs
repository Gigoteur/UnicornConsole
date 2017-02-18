#[cfg(feature = "lua")]
extern crate lua;

#[cfg(feature = "cpython")]
#[macro_use]
extern crate cpython;

extern crate sdl2;
extern crate getopts;

extern crate nalgebra;

#[macro_use]
extern crate glium;

extern crate image;
extern crate gif;

extern crate regex;
extern crate png;
extern crate byteorder;
extern crate rand;
extern crate time;
extern crate chrono;
extern crate libc;

extern crate rusttype;

extern crate chan;

#[macro_use]
extern crate log;
extern crate fern;

#[macro_use]
extern crate lazy_static;

extern crate rustc_serialize;

extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

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

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            format!("[{}][{}] {}", time::now().strftime("%Y-%m-%d][%H:%M:%S").unwrap(), level, msg)
        }),
        output: vec![fern::OutputConfig::stdout(), fern::OutputConfig::file("output.log")],
        level: log::LogLevelFilter::Trace,
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Info) {
        panic!("Failed to initialize global logger: {}", e);
    }

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("c", "check", "check the cartridge");
    opts.optflag("e", "editor", "edit the cartridge");
    opts.optflag("f", "fullscreen", "display in fullscreen");
    opts.optflagopt("d", "dump", "dump the cartridge", "FILE");
    opts.optflagopt("t", "transform", "transform the PNG/PX8 cartridge in P8", "FILE");
    opts.optflagopt("s", "scale", "scale the display", "VALUE");
    opts.optflagopt("b", "bind", "bind a server on a specific address", "ADDR");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let input = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    if matches.opt_present("c") {
        if input.contains(".png") {
            match Cartridge::from_png_file(input) {
                Ok(mut c) => {
                    println!("{:?}", c);

                    if matches.opt_present("d") {
                        c.dump(matches.opt_str("d").unwrap());
                    }
                },
                Err(e) => panic!(e),
            }
        } else if input.contains(".p8") {
            match Cartridge::from_p8_file(input) {
                Ok(mut c) => {
                    println!("{:?}", c);

                    if matches.opt_present("d") {
                        c.dump(matches.opt_str("d").unwrap());
                    }
                },
                Err(e) => panic!(e),
            }
        } else if input.contains(".px8") {
            match Cartridge::from_px8_file(input) {
                Ok(c) => {
                    println!("{:?}", c);
                },
                Err(e) => panic!(e),
            }
        }
    }
    else if matches.opt_present("t") {
        if input.contains(".png") {
            match Cartridge::from_png_file(input) {
                Ok(mut c) => {
                    println!("{:?}", c);

                    c.save_in_p8(matches.opt_str("t").unwrap());
                },
                Err(e) => panic!(e),
            }
        }
    } else {
        let mut scale = Scale::Scale4x;
        if matches.opt_present("s") {
            let value = matches.opt_str("s").unwrap().parse::<i32>().unwrap();
            match value {
                1 => scale = Scale::Scale1x,
                2 => scale = Scale::Scale2x,
                3 => scale = Scale::Scale3x,
                4 => scale = Scale::Scale4x,
                5 => scale = Scale::Scale5x,
                6 => scale = Scale::Scale6x,
                8 => scale = Scale::Scale8x,
                10 => scale = Scale::Scale10x,
                _ => scale = Scale::Scale1x
            }
        }

        let mut fullscreen = false;
        if matches.opt_present("f") {
            fullscreen = true;
        }

        start_px8(scale, fullscreen, input, matches.opt_present("e"));
    }
}

pub fn start_px8(scale: gfx::Scale, fullscreen: bool, filename: String, editor: bool) {
    let mut frontend = match frontend::Frontend::init(scale, fullscreen) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend
    };

    frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
    frontend.run_cartridge(filename, editor);
}