use log::{info};

#[derive(Debug)]
pub struct CartridgeMusic {}

impl CartridgeMusic {
    pub fn new(_lines: &[String]) -> CartridgeMusic {
        info!("[CARTRIDGE] CartridgeMusic");
        CartridgeMusic {}
    }

    pub fn new_from_bytes(_v: &[u8]) -> CartridgeMusic {
        CartridgeMusic {}
    }

    pub fn empty() -> CartridgeMusic {
        CartridgeMusic {}
    }
}