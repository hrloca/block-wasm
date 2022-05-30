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

    pub fn equals(&self, to: Pos) -> bool {
        self.x == to.x && self.y == to.y
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
        Around {
            top: self.may_pick(Dir::Top, current),
            bottom: self.may_pick(Dir::Bottom, current),
            left: self.may_pick(Dir::Left, current),
            right: self.may_pick(Dir::Right, current),
        }
    }

    pub fn may_pick(&self, dir: Dir, at: Pos) -> Option<&T> {
        if !self.has(at)(dir) {
            return None;
        }

        match dir {
            Dir::Top => self.pick(at.sub(0, 1)),
            Dir::Bottom => self.pick(at.add(0, 1)),
            Dir::Left => self.pick(at.sub(1, 0)),
            Dir::Right => self.pick(at.add(1, 0)),
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

    pub fn pick(&self, base: Pos) -> Option<&T> {
        match base {
            pos if pos.y > self.size().1 - 1 => None,
            pos if pos.x > self.size().0 - 1 => None,
            _ => Some(&self.body[base.y][base.x]),
        }
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
