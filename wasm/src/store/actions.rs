use crate::board::*;

pub enum Actions {
    Delete(Vec<Point>),
    Empty(Point),
    Change(Point, Point),
    Lock(Vec<Point>),
    UnLock(Vec<Point>),
    Move(Point, Point),
}
