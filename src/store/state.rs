use crate::blocks::*;
use crate::board::*;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone)]
pub struct State {
    pub changing: HashSet<String>,
    pub deleting: HashSet<String>,
    pub falling: HashSet<String>,
    pub changing_point: HashSet<Point>,
    pub deleting_point: HashSet<Point>,
    pub falling_point: HashSet<Point>,
    pub blocks: Board<Option<Block>>,
    pub complete_tasks: Vec<u64>,
    pub queue_tasks: VecDeque<u64>,
    pub active_queue_task: Option<u64>,
}

impl State {
    pub fn create() -> Self {
        State {
            blocks: create(),
            changing: HashSet::new(),
            deleting: HashSet::new(),
            falling: HashSet::new(),
            changing_point: HashSet::new(),
            deleting_point: HashSet::new(),
            falling_point: HashSet::new(),
            complete_tasks: Vec::new(),
            queue_tasks: VecDeque::new(),
            active_queue_task: None,
        }
    }
}
