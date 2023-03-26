use rusttype::Point;
use std::collections::HashMap;
use std::fmt;

pub struct GridCell {
    key: Point<i32>,
    links: HashMap<Point<i32>, GridCell>,
}

impl GridCell {
    pub fn new(key: Point<i32>) -> GridCell {
        let links = HashMap::new();
        GridCell { key, links }
    }

    pub fn key(&self) -> Point<i32> {
        self.key
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

    pub fn link(&mut self, other: GridCell) {
        self.links.insert(other.key(), other);
    }
}

impl fmt::Display for GridCell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GridCell({}, {})", self.key.x, self.key.y)
    }
}
