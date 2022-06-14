use crate::blocks::*;
use crate::board::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct State {
    pub locked: HashSet<String>,
    pub blocks: Board<Option<Block>>,
}
