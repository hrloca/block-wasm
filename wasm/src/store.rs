use crate::blocks::*;
use crate::board::*;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct State {
    pub falling: HashSet<String>,
    pub changing: HashSet<String>,
    pub blocks: Board<Option<Block>>,
}

pub enum Actions {
    Delete,
    Empty(Point),
    Change(Point, Point),
    WillChange,
    WillFall,
    Fall,
}

pub fn create_state() -> State {
    State {
        blocks: Board::from(vec![
            vec![Block::a(0, 9), Block::a(1, 1), Block::a(2, 2)],
            vec![Block::a(3, 2), Block::a(4, 1), Block::a(5, 1)],
            vec![Block::a(6, 2), Block::a(7, 1), Block::a(8, 1)],
            vec![Block::a(9, 2), Block::a(10, 1), Block::a(11, 1)],
        ]),
        falling: HashSet::new(),
        changing: HashSet::new(),
    }
}

pub fn reducer(state: &State, types: Actions) -> State {
    inspect(&state.blocks);

    match types {
        Actions::Change(a, b) => State {
            blocks: change(&state.blocks, a, b),
            ..state.clone()
        },
        Actions::Empty(point) => State {
            blocks: blank(&state.blocks, &vec![point]),
            ..state.clone()
        },
        _ => state.clone(),
    }
}
