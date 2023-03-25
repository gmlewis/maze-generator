use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::fmt;

#[derive(Debug)]
pub struct Rectangular {
    width: i32,
    height: i32,
    rng: ChaCha8Rng,
}

impl Rectangular {
    pub fn new(width: i32, height: i32, seed: u64) -> Rectangular {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        println!("Random f32: {}", rng.gen::<f32>());
        Rectangular { width, height, rng }
    }
}

impl fmt::Display for Rectangular {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangular({} x {})", self.width, self.height)
    }
}
