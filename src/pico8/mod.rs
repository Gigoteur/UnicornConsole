pub mod cartridge;

pub mod pico8 {
  use gfx::Sprite;

  use px8;

  pub struct Memory {
      sprites: Vec<Sprite>,
      map: [[u32; 32]; px8::SCREEN_WIDTH],
  }

  impl Memory {
      pub fn new(sprites: Vec<Sprite>, map: [[u32; 32]; px8::SCREEN_WIDTH]) -> Memory {
          Memory {
              sprites: sprites.clone(),
              map: map,
          }
      }

      pub fn sget(&mut self, x: u32, y: u32) -> u8 {
        let idx_sprite = (x/8) + 16 * (y/8);
        let sprite = &self.sprites[idx_sprite as usize];
        return *sprite.data.get(((x%8) + (y % 8) * 8) as usize).unwrap();
      }
  }
}