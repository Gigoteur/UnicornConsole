#[cfg(feature = "lua")]
extern crate lua;

#[cfg(feature = "cpython")]
#[macro_use]
extern crate cpython;

#[cfg(feature = "dyon")]
extern crate dyon;

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

extern crate rustc_serialize;

#[macro_use]
pub mod config;

pub mod frontend;
pub mod renderer;
pub mod gfx;
pub mod px8;
pub mod pico8;
pub mod plugins;