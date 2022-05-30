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
}

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn from(x: usize, y: usize) -> Pos {
        Pos { x, y }
    }

    pub fn add(&self, x: usize, y: usize) -> Pos {
        Pos {
            x: self.x + x,
            y: self.y + y,
        }
    }

    pub fn sub(&self, x: usize, y: usize) -> Pos {
        Pos {
            x: self.x - x,
            y: self.y - y,
        }
    }
}

pub type TableBody<T> = Vec<Vec<T>>;

#[derive(Debug, Clone)]
pub struct Table<T> {
    body: TableBody<T>,
}

impl<T> Table<T> {
    pub fn create(body: TableBody<T>) -> Table<T> {
        Table { body }
    }

    pub fn body(&self) -> &TableBody<T> {
        &self.body
    }

    pub fn around(&self, current: Pos) -> Around<Option<&T>> {
        let has = self.has(current);
        Around {
            top: self.pick(has(Dir::Top), || self.cell(current.sub(0, 1))),
            bottom: self.pick(has(Dir::Bottom), || self.cell(current.add(0, 1))),
            left: self.pick(has(Dir::Left), || self.cell(current.sub(1, 0))),
            right: self.pick(has(Dir::Right), || self.cell(current.add(1, 0))),
        }
    }

    pub fn pick<U>(&self, has: bool, delay: impl Fn() -> U) -> Option<U> {
        if has {
            Some(delay())
        } else {
            None
        }
    }

    pub fn has(&self, base: Pos) -> impl Fn(Dir) -> bool + '_ {
        move |dir| match dir {
            Dir::Top => base.y != 0,
            Dir::Bottom => base.y < self.size().1 - 1,
            Dir::Left => base.x != 0,
            Dir::Right => base.x < self.size().0 - 1,
        }
    }

    pub fn cell(&self, current: Pos) -> &T {
        &self.body[current.y][current.x]
    }

    pub fn size(&self) -> (usize, usize) {
        (self.body[0].len(), self.body.len())
    }

    pub fn read(&self, cb: impl Fn(Pos, &T) -> ()) {
        self.body.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, col)| {
                cb(Pos::from(x, y), col);
            })
        });
    }

    pub fn rebuild(&self, cb: impl Fn(Pos, &T) -> T) -> Self {
        Table {
            body: self
                .body
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, col)| cb(Pos::from(x, y), col))
                        .collect()
                })
                .collect(),
        }
    }
}
