use rusttype::Point;
use std::collections::HashMap;

pub struct Cell {
    links: HashMap<Point<i32>, Cell>,
}

impl Cell {
    pub fn new() -> Cell {
        Cell {
            links: HashMap::new(),
        }
    }
}
