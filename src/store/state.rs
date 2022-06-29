use crate::blocks::*;
use crate::board::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct State {
    pub changing_point: HashSet<Point>,
    pub deleting_point: HashSet<Point>,
    pub falling_point: HashSet<Point>,
    pub blocks: Board<Option<Block>>,
    pub complete_tasks: Vec<u64>,

    pub next_queue_task: HashSet<String>,
    pub active_queue_task: HashSet<String>,
}

impl State {
    pub fn create() -> Self {
        State {
            blocks: create(),
            changing_point: HashSet::new(),
            deleting_point: HashSet::new(),
            falling_point: HashSet::new(),

            complete_tasks: Vec::new(),

            next_queue_task: HashSet::new(),
            active_queue_task: HashSet::new(),
        }
    }
}
