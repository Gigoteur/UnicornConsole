extern crate sdl2;
extern crate getopts;

extern crate nalgebra;

#[macro_use]
extern crate cpython;

#[macro_use]
extern crate glium;
//extern crate glium_sdl2;

extern crate image;
extern crate gif;

extern crate lua;
extern crate libc;

extern crate regex;
extern crate png;
extern crate byteorder;
extern crate rand;
extern crate time;
extern crate chrono;

extern crate rusttype;

extern crate chan;

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate rustc_serialize;

use std::env;
use getopts::Options;

// Local stuff

#[macro_use]
mod config;

mod frontend;
mod gfx;
mod px8;
mod pico8;
mod plugins;

use gfx::Scale;
use pico8::cartridge::Cartridge;


fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    env_logger::init().unwrap();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("c", "check", "check the cartridge");
    opts.optflag("e", "editor", "edit the cartridge");
    opts.optflag("f", "fullscreen", "display in fullscreen");
    opts.optflagopt("d", "dump", "dump the cartridge", "FILE");
    opts.optflagopt("t", "transform", "transform the PNG/PX8 cartridge in P8", "FILE");
    opts.optflagopt("s", "scale", "scale the display", "VALUE");
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
                _ => scale = Scale::Scale4x
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
  let mut frontend = match frontend::SdlFrontend::init(scale, fullscreen) {
    Err(error) => panic!("{:?}", error),
    Ok(frontend) => frontend
  };

  frontend.main(filename, editor);
}