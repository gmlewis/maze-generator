use crate::maze::cell::GridCell;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rusttype::Point;
use std::{collections::HashMap, fmt};

pub struct Rectangular {
    cols: i32,
    rows: i32,
    rng: ChaCha8Rng,
    map: HashMap<Point<i32>, GridCell>,
}

impl Rectangular {
    pub fn new(cols: i32, rows: i32, seed: u64) -> Rectangular {
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let map = gen_grid(cols, rows);
        Rectangular {
            cols,
            rows,
            rng,
            map,
        }
    }
}

fn gen_grid(cols: i32, rows: i32) -> HashMap<Point<i32>, GridCell> {
    let mut map = HashMap::new();
    for y in 0..rows {
        for x in 0..cols {
            let k = Point { x, y };
            map.insert(k, GridCell::new(k));
        }
    }
    map
}

impl fmt::Display for Rectangular {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Rectangular({} x {})", self.cols, self.rows)?;
        write!(f, "+")?;
        writeln!(f, "{}", "---+".repeat(self.cols.try_into().unwrap()))?;
        for y in 0..self.rows {
            let mut row1 = vec!["|"];
            let mut row2 = vec!["+"];
            for x in 0..self.cols {
                let k = Point { x, y };
                let cell = self.map.get(&k).unwrap();
                let east_boundary = match cell.east() {
                    Some(_) => " ",
                    _ => "|",
                };
                row1.push("   ");
                row1.push(east_boundary);
                let south_boundary = match cell.south() {
                    Some(_) => "   ",
                    _ => "---",
                };
                row2.push(south_boundary);
                row2.push("+");
            }
            writeln!(f, "{}", row1.join(""))?;
            writeln!(f, "{}", row2.join(""))?;
        }
        Ok(())
    }
}
