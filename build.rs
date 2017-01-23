use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io;

fn prebuild() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("parameters.rs");

    let screen_width = match env::var_os("PX8_SCREEN_WIDTH") {
        Some(v) => v.into_string().unwrap(),
        None => "128".to_string(),
    };

    let screen_height = match env::var_os("PX8_SCREEN_HEIGHT") {
        Some(v) => v.into_string().unwrap(),
        None => "128".to_string(),
    };

    let mut f = File::create(&dest_path).unwrap();

    f.write_all(format!("pub const SCREEN_WIDTH: usize = {:?};\n", screen_width.parse::<u32>().unwrap()).as_bytes());
    f.write_all(format!("pub const SCREEN_HEIGHT: usize = {:?};\n", screen_height.parse::<u32>().unwrap()).as_bytes());

    Ok(())
}

fn main() {
    match prebuild() {
        Err(e) => panic!("Error: {}", e),
        Ok(()) => (),
    }
}
