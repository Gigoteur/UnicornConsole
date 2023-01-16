use core;

use log::{debug, warn, info};

#[derive(Debug)]
pub struct CartridgeMap {
    pub map: Vec<u32>,
    pub width: u32,
    pub height: u32,
}

impl CartridgeMap {
    pub fn empty() -> CartridgeMap {
        CartridgeMap { map: Vec::new(), width: 0, height: 0 }
    }

    pub fn new(lines: &[String], width: u32, height: u32) -> CartridgeMap {
        info!("[CARTRIDGE] [CartridgeMap] {:?}", lines.len());

        let mut map = Vec::new();

        for line in lines {
            debug!("[CARTRIDGE] [CartridgeMap] Line {:?}", line);

            let mut i = 0;

            while i < width as usize*2 {
                let idx_sprite = u32::from_str_radix(&line[i..i + 2], 16).unwrap();

                map.push(idx_sprite);
                
                i += 2;
            }

        }

        info!("[CARTRIDGE] [CartridgeMap] {:?}", map.len());

        CartridgeMap { map: map, width: width, height: height }
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for y in 0..32 {
            for x in 0..self.width {
                let idx_sprite = *self.map.get((x + self.width * y) as usize).unwrap_or(&0);
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