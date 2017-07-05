extern crate gcc;

use std::fs;
use std::io;
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::ffi::OsString;
use std::ffi::OsStr;

trait CommandExt {
    fn execute(&mut self) -> io::Result<()>;
}

impl CommandExt for Command {
    /// Execute the command and return an error if it exited with a failure status.
    fn execute(&mut self) -> io::Result<()> {
        let status = try!(self.status());
        if status.success() {
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::Other, format!("The command\n\
            \t{:?}\n\
            did not run successfully.", self)))
        }
    }
}

/// The command to build lua, with switches for different *nix targets.
fn build_lua(tooling: &gcc::Tool, source: &Path, build: &Path) -> io::Result<()> {
    // calculate the Lua platform name
    let platform = match env::var("TARGET").unwrap().split('-').nth(2).unwrap() {
        "windows" => "mingw",
        "darwin" => "macosx",
        "linux" => "linux",
        "freebsd" => "freebsd",
        "dragonfly" => "bsd",
        // fall back to the "generic" system
        _ => "generic",
    };

    // extract CC and MYCFLAGS from the detected tooling
    let cc = tooling.path();
    let mut cflags = OsString::new();
    for arg in tooling.args() {
        cflags.push(arg);
        cflags.push(" ");
    }

    // VPATH is used to invoke "make" from the directory where we want Lua to
    // be built into, but read the sources from the provided source directory.
    // Setting MAKE to match the command we invoke means that the VPATH and
    // Makefile path will be carried over when the Makefile invokes itself.
    let makefile = source.join("Makefile");
    let make = OsString::from(format!("make -e -f {:?}", makefile.to_string_lossy().replace("\\", "/")));

    // call the makefile
    let mut command = Command::new("make");
    for &(ref key, ref val) in tooling.env() {
        command.env(key, val);
    }
    command.current_dir(build)
        .env("VPATH", source.to_string_lossy().replace("\\", "/"))
        .env("MAKE", make)
        .env("CC", cc)
        .env("MYCFLAGS", cflags)
        .arg("-e")
        .arg("-f").arg(makefile)
        .arg(platform)
        .execute()
}


/// Ensure we have cl.exe and lib.exe at our disposal.
fn verify_msvc_environment() {
    let found_cl_exe = Command::new("cl.exe").arg("/help").output().is_ok();
    let found_lib_exe = Command::new("lib.exe").arg("/help").output().is_ok();

    if !found_cl_exe || !found_lib_exe {
        panic!("cl.exe and lib.exe must be on your %PATH% to compile Lua for MSVC.\n\
        Please install this crate through the Visual Studio Native Tools Command Line.");
    }
}

/// Compile liblua.lib for use with MSVC flavored Rust.
fn build_lua_msvc(source: &Path, build: &Path) -> io::Result<()>{
    verify_msvc_environment();
    let build_str = build.as_os_str().to_str().unwrap();
    // Compile our .obj files
    let mut compile_cmd = Command::new("cl.exe");
    compile_cmd.current_dir(&source);
    // Give cl.exe our .c files.
    for file_res in fs::read_dir(source).unwrap() {
        let dir_entry = file_res.unwrap();
        let file_name = dir_entry.file_name().into_string().unwrap();
        if file_name.ends_with(".c") && file_name != "luac.c" {
            compile_cmd.arg(file_name);
        }
    }
    compile_cmd.arg("/c") // Don't link. Just generate .obj files.
        .arg("/MP") // Builds multiple source files concurrently.
        .arg(format!("/Fo{}\\", &build_str)) // Output to the build folder
        .arg("/nologo"); // Prevent stdout pollution
        //.arg("/LD") // Not sure if I need this or not.
    compile_cmd.execute().unwrap(); // Block until compilation is complete.
    // Link our .obj files into liblua.lib.
    let mut lib_cmd = Command::new("lib.exe");
    lib_cmd.current_dir(&build);
    for file_res in fs::read_dir(build).unwrap() {
        let dir_entry = file_res.unwrap();
        let file_name = dir_entry.file_name().into_string().unwrap();
        if file_name.ends_with(".obj") {
            lib_cmd.arg(file_name);
        }
    }
    lib_cmd.arg(format!("/out:{}\\lua.lib", &build_str)) // Output file
        .arg("/NOLOGO");
    lib_cmd.execute()
}

/// If a static Lua is not yet available from a prior run of this script, this
/// will download Lua and build it. The cargo configuration text to link
/// statically against liblua.a/liblua.lib is then printed to stdout.
fn prebuild() -> io::Result<()> {
    let lua_dir : PathBuf = match env::var_os("LUA_LOCAL_SOURCE") {
        // If LUA_LOCAL_SOURCE is set, use it
        Some(dir) => PathBuf::from(dir),
        // Otherwise, pull from lua-source/src in the crate root
        None => {
            let mut dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
            dir.push(OsStr::new("lua-source/src").to_str().unwrap());
            dir
        }
    };
    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut config = gcc::Config::new();
    let msvc = env::var("TARGET").unwrap().split('-').last().unwrap() == "msvc";
    println!("cargo:rustc-link-lib=static=lua");
    if !msvc && lua_dir.join("liblua.a").exists() {
        // If liblua.a is already in lua_dir, use it
        println!("cargo:rustc-link-search=native={}", &lua_dir.display());
    } else if msvc {
        if !build_dir.join("lua.lib").exists() {
            try!(build_lua_msvc(&lua_dir, &build_dir));
        }
        println!("cargo:rustc-link-search=native={}", &build_dir.display());
    } else {
        // Check build_dir
        if !build_dir.join("liblua.a").exists() {
            // Build liblua.a
            let tooling = config.get_compiler();
            try!(fs::create_dir_all(&build_dir));
            try!(build_lua(&tooling, &lua_dir, &build_dir));
        }
        println!("cargo:rustc-link-search=native={}", &build_dir.display());
    }
    
    // Ensure the presence of glue.rs
    if !build_dir.join("glue.rs").exists() {
        // Compile and run glue.c
        let glue = build_dir.join("glue");
        try!(config.include(&lua_dir).get_compiler().to_command()
            .arg("-I").arg(&lua_dir)
            .arg("src/glue/glue.c")
            .arg("-o").arg(&glue)
            .execute());
        try!(Command::new(glue)
            .arg(build_dir.join("glue.rs"))
            .execute());
    }
    Ok(())
}

fn main() {
    match prebuild() {
        Err(e) => panic!("Error: {}", e),
        Ok(()) => (),
    }
}
