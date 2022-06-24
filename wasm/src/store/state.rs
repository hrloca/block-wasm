use crate::blocks::*;
use crate::board::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct State {
    pub changing: HashSet<String>,
    pub deleting: HashSet<String>,
    pub falling: HashSet<String>,
    pub blocks: Board<Option<Block>>,
    pub complete_tasks: Vec<u64>,
}
