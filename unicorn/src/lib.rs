#[cfg(feature = "rlua")]
extern crate rlua;

#[cfg(feature = "cpython")]
extern crate cpython;

#[cfg(feature = "duktape")]
extern crate duktape;

#[cfg(feature = "image")]
extern crate image;

extern crate anyhow;

extern crate strum;

extern crate png;

extern crate gif;
extern crate ordered_float;

extern crate regex;
extern crate byteorder;
extern crate libc;
extern crate glob;

extern crate rusttype;

extern crate gapbuffer;
extern crate unicode_width;

extern crate log;

extern crate num_traits;

extern crate serde;

#[macro_use]
pub mod config;
pub mod gfx;
pub mod core;
pub mod input;
pub mod cartridge;
pub mod plugins;

include!(concat!(env!("OUT_DIR"), "/parameters.rs"));
