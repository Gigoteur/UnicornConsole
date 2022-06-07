extern crate unicorn_sdl;

extern crate unicorn;

extern crate getopts;

extern crate log;
extern crate fern;
extern crate time;

use std::env;
use getopts::Options;
use fern::colors::{Color, ColoredLevelConfig};
use log::{debug};

use unicorn::gfx;
use unicorn::gfx::Scale;
use unicorn::cartridge::Cartridge;

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn set_up_logging(level: log::LevelFilter) {
    // configure colors for the whole line
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        // we actually don't need to specify the color for debug and info, they are white by default
        .info(Color::White)
        .debug(Color::White)
        // depending on the terminals color scheme, this is the same as the background color
        .trace(Color::BrightBlack);

    // configure colors for the name of the level.
    // since almost all of them are the same as the color for the whole line, we
    // just clone `colors_line` and overwrite our changes
    let colors_level = colors_line.clone().info(Color::Green);
    // here we set up our fern Dispatch
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                color_line = format_args!(
                    "\x1B[{}m",
                    colors_line.get_color(&record.level()).to_fg_str()
                ),
                date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        // set the default log level. to filter out verbose log messages from dependencies, set
        // this to Warn and overwrite the log level for your crate.
        .level(level)
        // change log levels for individual modules. Note: This looks for the record's target
        // field which defaults to the module path but can be overwritten with the `target`
        // parameter:
        // `info!(target="special_target", "This log message is about special_target");`
        .level_for("pretty_colored", log::LevelFilter::Trace)
        // output to stdout
        .chain(std::io::stdout())
        .apply()
        .unwrap();

    debug!("finished setting up logging! yay!");
}

fn main() {

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
        Err(f) => panic!("{}", f.to_string()),
    };


    if matches.opt_present("v") {
        set_up_logging(log::LevelFilter::Debug);
    } else {
        set_up_logging(log::LevelFilter::Warn);

    }
    
    if matches.opt_present("h") {
        print_usage(&program, &opts);
        return;
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
        run_cartridge_raw(scale,
                          fullscreen,
                          opengl,
                          "unicorn.uni",
                          include_bytes!("../../unicorn/sys/unicorn.uni").to_vec(),
                          matches.opt_present("e"));
        return;
    };

    if matches.opt_present("c") {
        if input.contains(".uni") {
            match Cartridge::from_unicorn_file(&input) {
                Ok(c) => {
                    println!("{:?}", c);
                }
                Err(e) => std::panic::panic_any(e),
            }
        } else if input.contains(".uc") {
            match Cartridge::from_unicorn_splitted_file(&input) {
                Ok(c) => {
                    println!("{:?}", c);
                }
                Err(e) => std::panic::panic_any(e),
            }
        } else if input.contains(".duc") {
            match Cartridge::from_dunicorn_file(&input) {
                Ok(c) => {
                    println!("{:?}", c);
                }
                Err(e) => std::panic::panic_any(e),
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
    let mut frontend = match unicorn_sdl::frontend::Frontend::init(scale, fullscreen, opengl, false) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend,
    };

    frontend.start();
    frontend.init_controllers("../unicorn-sdl/sys/config/gamecontrollerdb.txt".to_string());

    frontend.run_cartridge(filename, editor);
}

pub fn run_cartridge_raw(scale: gfx::Scale,
                         fullscreen: bool,
                         opengl: bool,
                         filename: &str,
                         data: Vec<u8>,
                         editor: bool) {
    let mut frontend = match unicorn_sdl::frontend::Frontend::init(scale, fullscreen, opengl, false) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend,
    };
    
    frontend.start();
    frontend.init_controllers("../unicorn-sdl/sys/config/gamecontrollerdb.txt".to_string());

    frontend.run_cartridge_raw(filename, data, editor);
}

pub fn run_interactive(scale: gfx::Scale, fullscreen: bool, opengl: bool) {
    let mut frontend = match unicorn_sdl::frontend::Frontend::init(scale, fullscreen, opengl, false) {
        Err(error) => panic!("{:?}", error),
        Ok(frontend) => frontend,
    };

    frontend.start();
    frontend.init_controllers("../unicorn-sdl/sys/config/gamecontrollerdb.txt".to_string());

    frontend.run_interactive();
}