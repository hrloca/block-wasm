use super::*;
use std::fmt;

pub type Body<T> = Vec<Vec<T>>;

#[derive(Debug, Clone)]
pub struct Board<T> {
    body: Body<T>,
}

impl<T> Board<T>
where
    T: Clone + Copy + fmt::Debug,
{
    pub fn from(body: Body<T>) -> Self {
        Board { body }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.body[0].len(), self.body.len())
    }

    pub fn has(&self, base: Pos) -> bool {
        base.y <= self.size().1 - 1 && base.x <= self.size().0 - 1
    }

    // TODO: Should Result Type
    pub fn insert(mut self, pos: Pos, content: T) -> Self {
        self.body[pos.y][pos.x] = content;
        self
    }

    // TODO: Should Result Type
    pub fn change(self, a: Pos, b: Pos) -> Self {
        let cell_a = self.body[a.y][a.x];
        let cell_b = self.body[b.y][b.x];
        self.insert(a, cell_b).insert(b, cell_a)
    }

    pub fn pick(&self, base: Pos) -> &T {
        &self.body[base.y][base.x]
    }

    pub fn safe_pick(&self, base: Pos) -> Option<&T> {
        match base {
            pos if pos.y > self.size().1 - 1 => None,
            pos if pos.x > self.size().0 - 1 => None,
            _ => Some(self.pick(base)),
        }
    }

    pub fn around_the(&self, base: Pos) -> impl Fn(Dir) -> bool + '_ {
        if !self.has(base) {
            panic!("テーブルの範囲外が指定されています");
        }

        move |dir| match dir {
            Dir::Top => base.y != 0,
            Dir::Bottom => base.y < self.size().1 - 1,
            Dir::Left => base.x != 0,
            Dir::Right => base.x < self.size().0 - 1,
        }
    }

    pub fn may_next(&self, dir: Dir, pos: Pos) -> (Option<&T>, Pos) {
        if !self.around_the(pos)(dir) {
            return (None, pos);
        }

        match dir {
            Dir::Top => {
                let top = pos.offset(0, -1);
                (self.safe_pick(top), top)
            }
            Dir::Bottom => {
                let bottom = pos.offset(0, 1);
                (self.safe_pick(bottom), bottom)
            }
            Dir::Left => {
                let left = pos.offset(-1, 0);
                (self.safe_pick(left), left)
            }
            Dir::Right => {
                let right = pos.offset(1, 0);
                (self.safe_pick(right), right)
            }
        }
    }

    pub fn around(&self, current: Pos) -> Around<(Option<&T>, Pos)> {
        Around {
            top: self.may_next(Dir::Top, current),
            bottom: self.may_next(Dir::Bottom, current),
            left: self.may_next(Dir::Left, current),
            right: self.may_next(Dir::Right, current),
        }
    }

    pub fn rebuild(&self, cb: impl Fn(Self, &T, Pos) -> Self) -> Self {
        let new = self.clone();
        self.body.iter().enumerate().fold(new, |acc, (y, row)| {
            row.iter()
                .enumerate()
                .fold(acc, |h, (x, content)| cb(h, content, Pos::from(x, y)))
        })
    }

    pub fn map(&self, cb: impl Fn(&T, Pos) -> T) -> Self {
        Board {
            body: self
                .body
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(x, col)| cb(col, Pos::from(x, y)))
                        .collect()
                })
                .collect(),
        }
    }

    pub fn each(&self, cb: impl Fn(&T, Pos) -> ()) {
        self.body.iter().enumerate().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, col)| {
                cb(col, Pos::from(x, y));
            })
        });
    }

    pub fn inspect(&self, cb: impl Fn(&T) -> ()) {
        let (x, y) = self.size();
        println!("---------------------------------------");
        for y in 0..y {
            for x in 0..x {
                print!("|");
                cb(self.pick(Pos::from(x, y)));
            }
            println!("|");
        }
        println!("---------------------------------------");
    }
}

// #[test]
// fn shoud_table_change() {
//     #[rustfmt::skip]
//     let table = Table::create(vec![
//         vec![1, 2],
//         vec![3, 4],
//     ]);

//     let start = Pos::from(0, 0);
//     let end = Pos::from(1, 1);
//     let changed = table.change(start, end);

//     assert_eq!(*changed.pick(start), 4);
//     assert_eq!(*changed.pick(end), 1);
// }

// #[test]
// fn shoud_table_pos() {
//     #[rustfmt::skip]
//     let table = Table::create(vec![
//         vec![1, 1, 2],
//         vec![2, 2, 1],
//         vec![1, 3, 1],
//     ]);

//     assert!(table.has(Pos::from(0, 0)));
//     assert!(table.has(Pos::from(2, 2)));
//     assert!(!table.has(Pos::from(3, 2)));
// }

// #[test]
// #[should_panic]
// fn shoud_pos_panic() {
//     let pos = Pos::from(5, 5).offset(-10, -5);
//     assert_eq!(pos.x, 0);
// }

// #[test]
// fn shoud_pos_equal() {
//     assert!(Pos::from(5, 5).equals(Pos::from(5, 5)));
// }

// #[test]
// fn shoud_pos_offset() {
//     let pos = Pos::from(5, 5);
//     let offset = pos.offset(-2, 5);
//     assert_eq!(offset.x, 3);
//     assert_eq!(offset.y, 10);
// }

// #[test]
// fn should_do_table() {
//     let tabel = Table::create(vec![
//         vec![1, 1, 2, 2],
//         vec![2, 2, 1, 1],
//         vec![1, 3, 1, 1],
//         vec![1, 3, 1, 1],
//     ]);

//     let startpoint_has = tabel.around_the(Pos::from(0, 0));
//     assert_eq!(startpoint_has(Dir::Top), false);
//     assert_eq!(startpoint_has(Dir::Bottom), true);
//     assert_eq!(startpoint_has(Dir::Left), false);
//     assert_eq!(startpoint_has(Dir::Right), true);

//     let endpoint_has = tabel.around_the(Pos::from(3, 3));
//     assert_eq!(endpoint_has(Dir::Top), true);
//     assert_eq!(endpoint_has(Dir::Bottom), false);
//     assert_eq!(endpoint_has(Dir::Left), true);
//     assert_eq!(endpoint_has(Dir::Right), false);
// }
