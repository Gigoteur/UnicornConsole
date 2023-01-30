#[cfg(feature = "rlua")]
extern crate rlua;

#[cfg(feature = "cpython")]
extern crate cpython;

#[cfg(feature = "image")]
extern crate image;

extern crate rhai;

extern crate rustpython;
extern crate rustpython_vm;

extern crate anyhow;

extern crate bytemuck;
extern crate strum;

extern crate png;

extern crate gif;
extern crate ordered_float;

extern crate regex;
extern crate byteorder;
extern crate glob;

extern crate gapbuffer;
extern crate unicode_width;

extern crate log;

extern crate num_traits;

extern crate serde;

extern crate paste;

extern crate base64;

extern crate arrayvec;

extern crate tinystr;

extern crate rtrb;

extern crate cpal;

extern crate fastrand;
extern crate rand;
extern crate rand_chacha;


pub mod gfx;
pub mod core;
pub mod input;
pub mod cartridge;
pub mod plugins;
pub mod contexts;
pub mod audio;
pub mod sound;

include!(concat!(env!("OUT_DIR"), "/parameters.rs"));
