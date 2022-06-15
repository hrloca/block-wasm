use super::*;
use std::fmt;

pub type Body<T> = Vec<Vec<T>>;

#[derive(Debug, Clone)]
pub struct Board<T> {
    body: Body<T>,
}

impl<T> Board<T>
where
    T: Clone + fmt::Debug,
{
    pub fn init(size: Size, init: impl Fn((usize, usize)) -> T) -> Self {
        let y = size.height;
        let x = size.width;
        let mut body = Vec::with_capacity(y);
        for y in 0..y {
            body.push(Vec::with_capacity(x));
            for x in 0..x {
                body[y].push(init((x, y)));
            }
        }

        Board { body }
    }

    pub fn from(body: Body<T>) -> Self {
        Board { body }
    }

    pub fn size(&self) -> Size {
        Size::of(self.body[0].len(), self.body.len())
    }

    pub fn insert(mut self, point: Point, element: T) -> Self {
        self.body[point.y][point.x] = element;
        self
    }

    pub fn has(&self, base: Point) -> bool {
        let size = self.size();
        base.y <= size.height - 1 && base.x <= size.width - 1
    }

    pub fn top(&self, point: Point) -> Option<(Point, &T)> {
        if point.y == 0 {
            None
        } else {
            let point = Point::of(point.x, point.y - 1);
            let element = self.pick(point);
            Some((point, element))
        }
    }

    pub fn bottom(&self, point: Point) -> Option<(Point, &T)> {
        let point = Point::of(point.x, point.y + 1);
        if self.has(point) {
            let element = self.pick(point);
            Some((point, element))
        } else {
            None
        }
    }

    pub fn left(&self, point: Point) -> Option<(Point, &T)> {
        if point.x == 0 {
            None
        } else {
            let point = Point::of(point.x - 1, point.y);
            let element = self.pick(point);
            Some((point, element))
        }
    }

    pub fn right(&self, point: Point) -> Option<(Point, &T)> {
        let point = Point::of(point.x + 1, point.y);
        if self.has(point) {
            let element = self.pick(point);
            Some((point, element))
        } else {
            None
        }
    }

    // TODO: Should Result Type
    pub fn change(self, a: Point, b: Point) -> Self {
        let cell_a = self.body[a.y][a.x].clone();
        let cell_b = self.body[b.y][b.x].clone();
        self.insert(a, cell_b).insert(b, cell_a)
    }

    pub fn pick(&self, base: Point) -> &T {
        &self.body[base.y][base.x]
    }

    pub fn try_pick(&self, base: Point) -> Result<&T, ()> {
        if self.has(base) {
            Ok(&self.body[base.y][base.x])
        } else {
            Err(())
        }
    }

    pub fn update<F>(&self, f: F) -> Self
    where
        F: Fn(Self, (Point, &T)) -> Board<T>,
    {
        self.fold(self.clone(), f)
    }

    pub fn fold<U, F>(&self, init: U, f: F) -> U
    where
        F: Fn(U, (Point, &T)) -> U,
    {
        self.body.iter().enumerate().fold(init, |acc, (y, row)| {
            row.iter()
                .enumerate()
                .fold(acc, |nested_acc, (x, element)| {
                    f(nested_acc, (Point::of(x, y), element))
                })
        })
    }

    pub fn map<F>(&self, f: F) -> Board<T>
    where
        F: Fn((Point, &T)) -> T,
    {
        Board {
            body: self.map2d(f),
        }
    }

    pub fn each<F>(&self, f: F) -> ()
    where
        F: Fn((Point, &T)) -> (),
    {
        self.body.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, col)| {
                f((Point::of(x, y), col));
            })
        });
    }

    pub fn inspect<F>(&self, f: F)
    where
        F: Fn((Point, &T)) -> (),
    {
        let size = self.size();
        println!("");
        for y in 0..size.height {
            for x in 0..size.width {
                print!("|");
                let point = Point::of(x, y);
                f((point, self.pick(point)));
            }
            println!("|");
        }
        println!("");
    }

    fn map2d<F>(&self, f: F) -> Body<T>
    where
        F: Fn((Point, &T)) -> T,
    {
        self.body
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, col)| f((Point::of(x, y), col)))
                    .collect()
            })
            .collect()
    }
}
