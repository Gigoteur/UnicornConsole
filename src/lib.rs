#[cfg(feature = "lua")]
extern crate lua;

#[cfg(feature = "cpython")]
#[macro_use]
extern crate cpython;

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

extern crate rusttype;

extern crate chan;

#[macro_use]
extern crate log;
extern crate fern;

#[macro_use]
extern crate lazy_static;

extern crate rustc_serialize;

#[macro_use]
pub mod config;

pub mod frontend;
pub mod renderer;
pub mod gfx;
pub mod px8;
pub mod cartridge;
pub mod plugins;
pub mod sound;