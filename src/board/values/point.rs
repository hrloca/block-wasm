use super::*;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn from(size: Size) -> Self {
        Point {
            x: size.width,
            y: size.height,
        }
    }

    pub fn of(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    pub fn equals(&self, to: Self) -> bool {
        self.x == to.x && self.y == to.y
    }

    pub fn add(&self, size: Size) -> Self {
        Point {
            x: self.x + size.width,
            y: self.y + size.height,
        }
    }

    pub fn sub(&self, size: Size) -> Self {
        Point {
            x: self.x - size.width,
            y: self.y - size.height,
        }
    }
}
