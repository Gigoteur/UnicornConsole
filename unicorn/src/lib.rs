#[cfg(feature = "unicorn_plugin_lua")]
extern crate unicorn_plugin_lua;

#[cfg(feature = "cpython")]
#[macro_use]
extern crate cpython;

#[cfg(feature = "duktape")]
extern crate duktape;

#[cfg(feature = "syntect")]
extern crate syntect;

extern crate chiptune;

extern crate image;
extern crate gif;

extern crate regex;
extern crate png;
extern crate byteorder;
extern crate rand;
extern crate time;
extern crate chrono;
extern crate libc;
extern crate glob;

extern crate rusttype;

extern crate gapbuffer;
extern crate tempdir;
extern crate unicode_width;

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

#[macro_use]
pub mod config;
pub mod gfx;
pub mod unicorn;
pub mod cartridge;
pub mod plugins;
pub mod sound;

include!(concat!(env!("OUT_DIR"), "/parameters.rs"));

// #[cfg(target_os="android")]
// extern crate jni;
//
// #[cfg(target_os="android")]
// use jni::objects::JObject;
// #[cfg(target_os="android")]
// use jni::objects::JClass;
// #[cfg(target_os="android")]
// use jni::JNIEnv;
// #[cfg(target_os="android")]
// use jni::sys::jint;
// #[cfg(target_os="android")]
// use sdl2::libc::c_char;
//
// #[no_mangle]
// #[allow(non_snake_case)]
// pub extern "C" fn SDL_main() -> i32 {
// let mut frontend = match frontend::Frontend::init(gfx::Scale::Scale1x, true, false, false) {
// Err(error) => panic!("{:?}", error),
// Ok(frontend) => frontend,
// };
//
// let data = include_bytes!("../sys/unicorn.uni");
// let data_final: Vec<u8> = unicorn::array_to_vec(data);
//
// frontend.start("./sys/config/gamecontrollerdb.txt".to_string());
// frontend.run_cartridge_raw("unicorn.p8",
// data_final,
// false,
// unicorn::UnicornMode::Unicorn);
//
// 0
// }
//
// #[cfg(target_os="android")]
// extern "C" {
// fn SDL_Android_Init(env: JNIEnv, cls: JClass);
// fn SDL_SetMainReady();
// }
//
// #[cfg(target_os="android")]
// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn Java_org_libsdl_app_SDLActivity_nativeInit(env: JNIEnv,
// cls: JClass,
// array: JObject)
// -> jint {
// let mut i: i32;
// let mut argc: i32;
// let mut status: i32;
// let mut len: i32;
// let mut argv: *const *const c_char;
//
// This interface could expand with ABI negotiation, callbacks, etc.
// SDL_Android_Init(env, cls);
//
// SDL_SetMainReady();
//
// Run the application.
//
// status = SDL_main(/*argc, argv*/);
//
// return status;
// }