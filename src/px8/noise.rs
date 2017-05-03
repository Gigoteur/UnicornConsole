use num_traits;
use num_traits::{NumCast};

use noise::{NoiseModule, Perlin, Seedable};

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

    pub fn set_seed(&mut self, seed: u32) {
        debug!("Change seed to {:?}", seed);
        self.perlin.set_seed(seed as usize);
    }
}