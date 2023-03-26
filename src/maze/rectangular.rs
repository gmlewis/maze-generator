use crate::maze::cell::Cell;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rusttype::Point;
use std::{collections::HashMap, fmt};

pub struct Rectangular {
    width: i32,
    height: i32,
    rng: ChaCha8Rng,
    map: HashMap<Point<i32>, Cell>,
}

impl Rectangular {
    pub fn new(width: i32, height: i32, seed: u64) -> Rectangular {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let mut map = gen_grid(width, height);
        Rectangular {
            width,
            height,
            rng,
            map,
        }
    }
}

fn gen_grid(width: i32, height: i32) -> HashMap<Point<i32>, Cell> {
    let mut map = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            map.insert(Point { x, y }, Cell::new());
        }
    }
    map
}

impl fmt::Display for Rectangular {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Rectangular({} x {})", self.width, self.height)
    }
}
