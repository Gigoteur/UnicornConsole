use log::{warn, info};

pub use super::utils::read_u8;

use gfx::sprite::Sprite;

#[derive(Debug)]
pub struct CartridgeGFF {
    pub flags: Vec<u8>,
}

impl CartridgeGFF {
    pub fn empty() -> CartridgeGFF {
        CartridgeGFF { flags: Vec::new() }
    }

    pub fn new(lines: &[String]) -> CartridgeGFF {
        info!("[CARTRIDGE] CartridgeGFF");

        let mut v = Vec::new();

        for line in lines {
            for c in line.as_bytes() {
                v.push((*c as char).to_digit(16).unwrap() as u8);
            }
        }

        let mut v_order = Vec::new();
        let mut idx = 0;
        while idx < v.len() {
            v_order.push(v[idx + 1]);
            v_order.push(v[idx]);

            idx += 2;
        }

        CartridgeGFF::new_from_bytes(&v_order)
    }

    pub fn new_from_bytes(v: &[u8]) -> CartridgeGFF {
        let mut flags: Vec<u8> = Vec::new();

        let mut v_copy = v.to_vec();

        let len_v = v_copy.len();
        let mut idx: usize = 0;

        while idx < len_v {
            let flag = read_u8(&mut v_copy);
            flags.push(flag as u8);
            idx += 2;
        }

        CartridgeGFF { flags: flags.clone() }
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        for (i, flag) in self.flags.iter().enumerate() {
            data.push_str(&format!("{:x}{:x}", (flag & 0xf0) >> 4, flag & 0x0f));

            if (i + 1) % 128 == 0 {
                data.push('\n');
            }
        }

        data
    }

    pub fn set_flags(&mut self, sprites: Vec<Sprite>) {
        if self.flags.len() != sprites.len() {
            warn!("Wrong number of flags {:?}, {:?}", self.flags.len(), sprites.len());
            return;
        }

        let mut idx = 0;
        for s in &sprites {
            if idx <= self.flags.len() {
                self.flags[idx] = s.flags;
            } else {
                self.flags.push(s.flags);
            }

            idx += 1;
        }
    }
}
