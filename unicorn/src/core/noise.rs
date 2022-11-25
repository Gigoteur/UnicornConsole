use num_traits;
use num_traits::NumCast;

#[cfg(feature = "noise")]
use noise::{NoiseModule, Perlin, Seedable};

#[cfg(feature = "noise")]
fn cast<T: NumCast, R: NumCast>(val: T) -> R {
    num_traits::cast(val).unwrap()
}


#[cfg(feature = "noise")]
pub struct Noise {
    perlin: Perlin,
}

#[cfg(feature = "noise")]
impl Noise {
    pub fn new() -> Noise {
        info!("[Unicorn][Noise] new");
        Noise { perlin: Perlin::new() }
    }

    pub fn get(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let r: f64 = cast(self.perlin.get([x, y, z]));
        r
    }

    pub fn set_seed(&mut self, seed: u32) {
        debug!("Change seed to {:?}", seed);
        self.perlin.set_seed(seed as usize);
    }
}


#[cfg(not(feature = "noise"))]
pub struct Noise {
}

#[cfg(not(feature = "noise"))]
impl Noise {
    pub fn new() -> Noise {
        info!("[Unicorn][Noise] new");
        Noise { }
    }

    pub fn get(&mut self, _x: f64, _y: f64, _z: f64) -> f64 {
        0.
    }

    pub fn set_seed(&mut self, _seed: u32) {
    }
}
