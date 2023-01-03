use core;

use log::{debug, warn, info};

#[derive(Debug)]
pub struct CartridgeMap {
    pub map: Vec<u32>,
}

impl CartridgeMap {
    pub fn empty() -> CartridgeMap {
        CartridgeMap { map: Vec::new() }
    }

    pub fn new(lines: &[String]) -> CartridgeMap {
        info!("[CARTRIDGE] [CartridgeMap]");

        let mut map = Vec::new();
        let mut y = 0;

        for line in lines {
            debug!("[CARTRIDGE] [CartridgeMap] Line {:?} {:?}", y, line);

            let mut i = 0;

            while i < 128*2 {
                let idx_sprite = u32::from_str_radix(&line[i..i + 2], 16).unwrap();

                map.push(idx_sprite);
                
                i += 2;
            }

            y += 1;

            if y == 32 {
                break;
            }
        }

        debug!("[CARTRIDGE] [CartridgeMap] {:?}", map);

        CartridgeMap { map: map }
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for y in 0..32 {
            for x in 0..128 {
                let idx_sprite = *self.map.get(x * 128 + y).unwrap_or(&0);
                data.push_str(&format!("{:02x}", idx_sprite));
            }
            data.push('\n');
        }

        data
    }

    pub fn set_map(&mut self, map: Vec<u32>) {
        self.map = map;
    }
}