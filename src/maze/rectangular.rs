use crate::maze::cell::GridCell;
use rand::Rng;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use rusttype::Point;
use std::{collections::HashMap, fmt};

pub struct Rectangular<'a> {
    cols: i32,
    rows: i32,
    rng: ChaCha8Rng,
    map: HashMap<Point<i32>, GridCell<'a>>,
}

impl<'a> Rectangular<'a> {
    fn new(cols: i32, rows: i32, seed: u64) -> Rectangular<'a> {
        let rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let map = gen_grid(cols, rows);
        Rectangular {
            cols,
            rows,
            rng,
            map,
        }
    }

    pub fn build(cols: i32, rows: i32, seed: u64) -> Rectangular<'a> {
        let mut r = Self::new(cols, rows, seed);
        let cell = r.random_cell();
        println!("random cell={cell}");
        r
    }

    fn random_cell(&mut self) -> &GridCell {
        let y = self.rng.gen_range(0..self.rows);
        let x = self.rng.gen_range(0..self.cols);
        let key = Point { x, y };
        self.map.get(&key).unwrap()
    }
}

fn gen_grid(cols: i32, rows: i32) -> HashMap<Point<i32>, GridCell<'static>> {
    let mut map = HashMap::new();
    for y in 0..rows {
        for x in 0..cols {
            let k = Point { x, y };
            map.insert(k, GridCell::new(k));
        }
    }
    map
}

impl<'a> fmt::Display for Rectangular<'a> {
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
