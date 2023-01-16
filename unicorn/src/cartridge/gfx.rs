use log::{debug, info};

use gfx::sprite::Sprite;

#[derive(Debug)]
pub struct CartridgeGFX {
    pub sprites: Vec<Sprite>,
}


impl CartridgeGFX {
    pub fn empty() -> CartridgeGFX {
        // Fill with no sprites
        CartridgeGFX { sprites:  Vec::new() }
    }

    pub fn new(lines: &[String]) -> CartridgeGFX {
        info!("[CARTRIDGE][CartridgeGFX]");

        let mut sprites: Vec<Sprite> = Vec::new();

        if !lines.is_empty() {
            let mut v = Vec::new();

            debug!("[CARTRIDGE][CartridgeGFX] Lines {:?}", lines.len());

            for line in lines {
                //debug!("[CARTRIDGE][CartridgeGFX] LEN LINE {:?}", line.len());

                if line.len() > 128 {
                    continue;
                }

                for c in line.as_bytes() {
                    v.push((*c as char).to_digit(16).unwrap());
                }
            }

            // Fix gap if there is not enough lines ...
            for _i in lines.len()..128 {
                for _j in 0..128 {
                    v.push(0);
                }
            }

            debug!("[CARTRIDGE][CartridgeGFX] {:?}", v.len());

            let mut g_off = 0;

            debug!("[CARTRIDGE][CartridgeGFX] Finding all sprites ...");

            // Fill all sprites
            for idx in 0..(v.len()/64) {
                let mut data: [u8; 8 * 8] = [0; 8 * 8];

                let mut idx_vec = 0;

                if idx > 0 {
                    if idx % 16 == 0 {
                        g_off = idx * 8 * 8;
                    } else {
                        g_off += 8;
                    }
                }
               
                for y in 0..8 {
                    for x in 0..8 {
                        let offset = g_off + y * 128 + x;

                        data[idx_vec] = v[offset] as u8;
                        idx_vec += 1;
                    }
                }

                debug!("[CARTRIDGE][CartridgeGFX] Sprite number {:?} {:?}:{:?}", sprites.len(), data.len(), data);

                sprites.push(Sprite::new(data));
            }
            // Fill with empty sprites
            if sprites.len() == 0 {
                for _ in 0..128 {
                    sprites.push(Sprite::new([0; 8 * 8]));
                }
            }

            info!("[CARTRIDGE][CartridgeGFX] {:?}", sprites.len());
        }

        CartridgeGFX { sprites: sprites }
    }

    pub fn set_sprites(&mut self, sprites: Vec<Sprite>) {
        self.sprites = sprites;
    }

    pub fn get_data(&mut self) -> String {
        let mut data = String::new();

        let mut idx_sprites = 0;
        let mut line;

        for y in 0..128 {
            line = y % 8;

            if y > 0 && (y % 8) == 0 {
                idx_sprites += 16;
            }

            for idx in idx_sprites..idx_sprites + 16 {
                let mut gfx_sprites = self.sprites[idx].clone();

                data.push_str(&gfx_sprites.get_line(line));
            }

            data.push('\n');
        }

        data
    }
}
