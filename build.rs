use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

fn prebuild() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("parameters.rs");

    let map_width = match env::var_os("PX8_MAP_WIDTH") {
        Some(v) => v.into_string().unwrap(),
        None => "128".to_string(),
    };

    let map_height = match env::var_os("PX8_MAP_HEIGHT") {
        Some(v) => v.into_string().unwrap(),
        None => "32".to_string(),
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
    f.write_all(format!("pub const VERSION: u32 = 0;\n").as_bytes())
        .unwrap();
    f.write_all(format!("pub const MAJOR_VERSION: u32 = 0;\n").as_bytes())
        .unwrap();
    f.write_all(format!("pub const MINOR_VERSION: u32 = 5;\n").as_bytes())
        .unwrap();

    Ok(())
}

fn main() {
    let profile = env::var("PROFILE").unwrap_or("Debug".to_string());
    let current_dir = std::env::current_dir().unwrap();
    let target;
    let target_os = env::var("TARGET").unwrap();

    if profile == "Release" {
        target = Path::new(&current_dir).join("target/release");
    } else {
        target = Path::new(&current_dir).join("target/debug");
    }

    match prebuild() {
        Err(e) => panic!("Error: {}", e),
        Ok(()) => (),
    }

    if target_os.contains("android") {
        println!("cargo:rustc-flags=-L android/sdl/build/intermediates/cmake/debug/obj/armeabi",);
        println!("cargo:rustc-flags=-L android/sdl/build/intermediates/cmake/debug/obj/armeabi-v7a",);
        println!("cargo:rustc-flags=-L android/sdl/build/intermediates/cmake/debug/obj/x86",);
        println!("cargo:rustc-flags=-L android/sdl/build/intermediates/cmake/debug/obj/arm64-v8a",);

        // We should also add the following instead of defining our toolchain in .cargo/config
        // -C link-arg=--sysroot=$NDK_ROOT/platforms/android-<api level you are targeting>/arch-arm

        let abi = if target_os.contains("armv7") {
            "armeabi-v7a"
        } else if target_os.contains("aarch64") {
            "arm64-v8a"
        } else if target_os.contains("arm") {
            "armeabi"
        } else if target_os.contains("x86") {
            "x86"
        } else if target_os.contains("i686") {
            "x86"
        } else {
            panic!("Invalid target architecture {}", target_os);
        };
/*
        let src = Path::new(&current_dir).join("target").join(target_os).join("debug").join("libpx8.so");
        let dst = Path::new(&current_dir).join("android/app/src/main/jniLibs").join(abi).join("libpx8.so");
        //panic!("{:?}", dst);
        //std::fs::remove_file(Path::new(&dst)).unwrap();
        // This won't work as it's being executed before the actual library has finished building :(
        std::fs::copy(src, dst).unwrap();*/
    }
}
