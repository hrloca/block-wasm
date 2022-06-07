mod point;
mod size;

pub use point::*;
pub use size::*;

#[derive(Debug, Clone, Copy)]
pub struct Around<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T,
}

impl<T> Around<T> {
    pub fn map<U>(&self, cb: impl Fn(&T) -> U) -> Around<U> {
        Around {
            top: cb(&self.top),
            bottom: cb(&self.bottom),
            left: cb(&self.left),
            right: cb(&self.right),
        }
    }

    pub fn each<U>(&self, cb: impl Fn(&T) -> U) {
        cb(&self.top);
        cb(&self.bottom);
        cb(&self.left);
        cb(&self.right);
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Top,
    Bottom,
    Left,
    Right,
}
