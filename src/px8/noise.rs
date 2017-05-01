use num_traits;
use num_traits::{Float, NumCast};

use noise::{NoiseModule, Perlin, Fbm, Seedable};

use gfx::{SCREEN_WIDTH, SCREEN_HEIGHT};


fn cast<T: NumCast, R: NumCast>(val: T) -> R {
    num_traits::cast(val).unwrap()
}

pub struct Noise {
    perlin: Perlin,
}

impl Noise {
    pub fn new() -> Noise {
        Noise {
            perlin: Perlin::new()
        }
    }

    pub fn get(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let r : f64 = cast(self.perlin.get([x, y, z]));
        r
    }
}