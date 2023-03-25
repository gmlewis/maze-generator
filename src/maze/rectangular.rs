use std::fmt;

#[derive(Debug)]
pub struct Rectangular {
    width: i32,
    height: i32,
}

impl Rectangular {
    pub fn new(width: i32, height: i32) -> Rectangular {
        Rectangular { width, height }
    }
}

impl fmt::Display for Rectangular {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Rectangular({} x {})", self.width, self.height)
    }
}
