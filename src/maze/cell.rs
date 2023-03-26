use rusttype::Point;
use std::collections::HashMap;
use std::fmt;

pub struct GridCell<'a> {
    key: Point<i32>,
    links: HashMap<Point<i32>, &'a GridCell<'a>>,
}

impl<'a> GridCell<'a> {
    pub fn new(key: Point<i32>) -> GridCell<'a> {
        let links = HashMap::new();
        GridCell { key, links }
    }

    pub fn key(&self) -> Point<i32> {
        self.key
    }

    pub fn north(&self) -> Option<&GridCell<'a>> {
        let k = Point {
            x: self.key.x,
            y: self.key.y - 1,
        };
        self.links.get(&k).copied()
    }

    pub fn south(&self) -> Option<&GridCell> {
        let k = Point {
            x: self.key.x,
            y: self.key.y + 1,
        };
        self.links.get(&k).copied()
    }

    pub fn east(&self) -> Option<&GridCell> {
        let k = Point {
            x: self.key.x + 1,
            y: self.key.y,
        };
        self.links.get(&k).copied()
    }

    pub fn west(&self) -> Option<&GridCell> {
        let k = Point {
            x: self.key.x - 1,
            y: self.key.y,
        };
        self.links.get(&k).copied()
    }

    pub fn neighbors(&self) -> Vec<&GridCell> {
        let dirs = vec![self.north(), self.south(), self.east(), self.west()];
        dirs.into_iter().flatten().collect()
    }

    pub fn link(&mut self, other: &'a GridCell<'a>) {
        self.links.insert(other.key(), other);
    }
}

impl<'a> fmt::Display for GridCell<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GridCell({}, {})", self.key.x, self.key.y)
    }
}
