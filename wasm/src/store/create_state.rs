use super::*;
use crate::blocks::*;
use std::collections::HashSet;

pub fn create_state() -> State {
    State {
        blocks: create(),
        changing: HashSet::new(),
        deleting: HashSet::new(),
        falling: HashSet::new(),
        complete_tasks: Vec::new(),
    }
}
