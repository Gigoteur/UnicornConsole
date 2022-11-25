#[cfg(feature = "rlua")]
extern crate rlua;

#[cfg(feature = "cpython")]
extern crate cpython;

#[cfg(feature = "duktape")]
extern crate duktape;

#[cfg(feature = "noise")]
extern crate noise;

#[cfg(feature = "image")]
extern crate image;

extern crate anyhow;

extern crate png;

extern crate gif;
extern crate ordered_float;

extern crate regex;
extern crate byteorder;
extern crate rand;
extern crate libc;
extern crate glob;

extern crate rusttype;

extern crate gapbuffer;
extern crate unicode_width;

#[macro_use]
extern crate log;
extern crate fern;

#[macro_use]
extern crate lazy_static;

extern crate serde;

extern crate num_traits;

#[macro_use]
pub mod config;
pub mod gfx;
pub mod core;
pub mod cartridge;
pub mod plugins;
pub mod editor;

include!(concat!(env!("OUT_DIR"), "/parameters.rs"));
