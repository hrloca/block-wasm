use super::*;
use crate::blocks::*;
use crate::board::*;
use std::collections::HashSet;

pub fn create_state() -> State {
    State {
        blocks: create(),
        locked: HashSet::new(),
    }
}
