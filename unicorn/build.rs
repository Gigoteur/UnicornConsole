use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

fn prebuild() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("parameters.rs");

    let map_width = match env::var_os("UNICORN_MAP_WIDTH") {
        Some(v) => v.into_string().unwrap(),
        None => "400".to_string(),
    };

    let map_height = match env::var_os("UNICORN_MAP_HEIGHT") {
        Some(v) => v.into_string().unwrap(),
        None => "60".to_string(),
    };

    let mut f = File::create(&dest_path).unwrap();

    f.write_all(format!("pub const MAP_WIDTH: usize = {:?};\n",
                           map_width.parse::<u32>().unwrap())
                           .as_bytes())
        .unwrap();
    f.write_all(format!("pub const MAP_HEIGHT: usize = {:?};\n",
                           map_height.parse::<u32>().unwrap())
                           .as_bytes())
        .unwrap();
    f.write_all(format!("pub const VERSION: u32 = 1;\n").as_bytes())
        .unwrap();
    f.write_all(format!("pub const MAJOR_VERSION: u32 = 0;\n").as_bytes())
        .unwrap();
    f.write_all(format!("pub const MINOR_VERSION: u32 = 0;\n").as_bytes())
        .unwrap();

    Ok(())
}

fn main() {
    let target_os = env::var("TARGET").unwrap();

    match prebuild() {
        Err(e) => panic!("Error: {}", e),
        Ok(()) => (),
    }

    if target_os.contains("android") {
        println!("rustc-link-lib=static=chiptune");
        println!("rustc-link-lib=static=duktape");

        println!("cargo:rustc-flags=-L android/sdl/build/intermediates/cmake/debug/obj/armeabi",);
        println!("cargo:rustc-flags=-L android/sdl/build/intermediates/cmake/debug/obj/armeabi-v7a",);
        println!("cargo:rustc-flags=-L android/sdl/build/intermediates/cmake/debug/obj/x86",);
        println!("cargo:rustc-flags=-L android/sdl/build/intermediates/cmake/debug/obj/arm64-v8a",);

        // We should also add the following instead of defining our toolchain in .cargo/config
        // -C link-arg=--sysroot=$NDK_ROOT/platforms/android-<api level you are targeting>/arch-arm

        let _abi = if target_os.contains("armv7") {
            "armeabi-v7a"
        } else if target_os.contains("arm") {
            "armeabi"
        } else if target_os.contains("aarch64") {
            "arm64-v8a"
        } else if target_os.contains("x86") {
            "x86"
        } else if target_os.contains("i686") {
            "x86"
        } else {
            panic!("Invalid target architecture {}", target_os);
        };
    }
}
