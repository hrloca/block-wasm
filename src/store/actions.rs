use crate::board::*;

pub enum Actions {
    Delete(Vec<Point>),
    Deleting(Vec<Point>),
    Change(Point, Point),
    Changing(Point, Point),
    Move(Point, Point),
    Fall(Point, Point),
    Falling(Point, Point),

    DeleteCompleteTask(u64),
    AddCompleteTask(u64),

    ActiveQueueName(String),
    DeleteActiveQueueName(String),
    NextQueueName(String),
    DeleteNextQueueName(String),
}
