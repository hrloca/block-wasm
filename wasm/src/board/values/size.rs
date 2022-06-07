#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Size {
    pub fn of(width: usize, height: usize) -> Self {
        Size { width, height }
    }

    pub fn equals(&self, to: Self) -> bool {
        self.width == to.width && self.height == to.height
    }

    pub fn add(&self, size: Size) -> Self {
        Size {
            width: self.width + size.width,
            height: self.height + size.height,
        }
    }

    pub fn sub(&self, size: Size) -> Self {
        Size {
            width: self.width - size.width,
            height: self.height + size.height,
        }
    }
}
