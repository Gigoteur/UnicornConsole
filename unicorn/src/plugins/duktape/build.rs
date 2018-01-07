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

fn build_duktape(tooling: &gcc::Tool, source: &Path, build: &Path) -> io::Result<()> {
    // extract CC and MYCFLAGS from the detected tooling
    let cc = tooling.path();
    let mut cflags = OsString::new();
    for arg in tooling.args() {
        cflags.push(arg);
        cflags.push(" ");
    }

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
        .arg("-e")
        .arg("-f")
        .arg(makefile)
        .execute()
}

fn prebuild() -> io::Result<()> {
    let duktape_dir : PathBuf = match env::var_os("DUKTAPE_LOCAL_SOURCE") {
        Some(dir) => PathBuf::from(dir),
        None => {
            let mut dir = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
            dir.push(OsStr::new("duktape-source").to_str().unwrap());
            dir
        }
    };

    let build_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut config = gcc::Config::new();

    println!("cargo:rustc-link-lib=static=duktape");

    // Check build_dir
    if duktape_dir.join("libduktape.a").exists() {
        println!("cargo:rustc-link-search=native={}", &duktape_dir.display());
    } else {
        // Build libduktape.a
        let tooling = config.get_compiler();
        try!(fs::create_dir_all(&build_dir));
        try!(build_duktape(&tooling, &duktape_dir, &build_dir));
        println!("cargo:rustc-link-search=native={}", &build_dir.display());
    }
    
    Ok(())
}

fn main() {
    match prebuild() {
        Err(e) => panic!("Error: {}", e),
        Ok(()) => (),
    }
}