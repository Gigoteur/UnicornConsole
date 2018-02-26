extern crate unicorn;

extern crate getopts;

extern crate log;
extern crate fern;
extern crate time;

extern crate glutin_window;
extern crate piston;
extern crate graphics;
extern crate gfx_graphics;
extern crate gfx;
extern crate gfx_device_gl;

use gfx::traits::*;
use gfx::format::{DepthStencil, Formatted, Srgba8};
use gfx::memory::Typed;
use glutin_window::{GlutinWindow, OpenGL};
use piston::window::{OpenGLWindow, Window, WindowSettings};
use piston::event_loop::{Events, EventSettings, EventLoop};
use graphics::draw_state::Blend;
use graphics::*;
use piston::input::*;
use gfx_graphics::{Flip, Gfx2d, Texture, TextureSettings};

use unicorn::gfx::Scale;
use unicorn::cartridge::Cartridge;


use std::env;
use getopts::Options;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
	let uc = unicorn::unicorn::Unicorn::new();

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

        return;
    };

    let opengl = OpenGL::V3_2;
    let samples = 4;
    let mut window: GlutinWindow = WindowSettings::new(
            "Unicorn: Piston",
            [600, 600]
        )
        .exit_on_esc(true)
        .samples(samples)
        .opengl(opengl)
        .build()
        .unwrap();

    let (mut device, mut factory) = gfx_device_gl::create(|s|
        window.get_proc_address(s) as *const std::os::raw::c_void);

}