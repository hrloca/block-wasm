use super::*;
use crate::blocks::*;
use crate::board::*;
use std::collections::HashSet;

pub fn create_state() -> State {
    State {
        blocks: Board::from(vec![
            vec![Block::a(0, 1), Block::a(1, 2), Block::a(2, 5)],
            vec![Block::a(3, 2), Block::a(4, 3), Block::a(5, 4)],
            vec![Block::a(6, 3), Block::a(7, 4), Block::a(8, 2)],
            vec![Block::a(9, 4), Block::a(10, 4), Block::a(11, 1)],
            vec![Block::a(12, 2), Block::a(13, 5), Block::a(14, 1)],
        ]),
        locked: HashSet::new(),
    }
}
