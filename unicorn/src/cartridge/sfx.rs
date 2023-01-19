use log::{info};

#[derive(Debug)]
pub struct CartridgeSFX {}

impl CartridgeSFX {
    pub fn new(_lines: &[String]) -> CartridgeSFX {
        info!("[CARTRIDGE] CartridgeSFX");
        CartridgeSFX {}
    }

    pub fn new_from_bytes(_v: &[u8]) -> CartridgeSFX {
        CartridgeSFX {}
    }

    pub fn empty() -> CartridgeSFX {
        CartridgeSFX {}
    }
}