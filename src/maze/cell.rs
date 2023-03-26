use rusttype::Point;
use std::collections::HashMap;

pub struct GridCell {
    key: Point<i32>,
    links: HashMap<Point<i32>, GridCell>,
}

impl GridCell {
    pub fn new(key: Point<i32>) -> GridCell {
        let links = HashMap::new();
        GridCell { key, links }
    }

    pub fn north(&self) -> Option<&GridCell> {
        let k = Point {
            x: self.key.x,
            y: self.key.y - 1,
        };
        self.links.get(&k)
    }

    pub fn south(&self) -> Option<&GridCell> {
        let k = Point {
            x: self.key.x,
            y: self.key.y + 1,
        };
        self.links.get(&k)
    }

    pub fn east(&self) -> Option<&GridCell> {
        let k = Point {
            x: self.key.x + 1,
            y: self.key.y,
        };
        self.links.get(&k)
    }

    pub fn west(&self) -> Option<&GridCell> {
        let k = Point {
            x: self.key.x - 1,
            y: self.key.y,
        };
        self.links.get(&k)
    }

    pub fn neighbors(&self) -> Vec<&GridCell> {
        let dirs = vec![self.north(), self.south(), self.east(), self.west()];
        dirs.into_iter().flatten().collect()
    }
}
