use crate::blocks::*;
use crate::board::*;
use crate::tools::store::Store;
use std::collections::HashSet;

pub type StoreType = Store<State, Actions>;

#[derive(Debug, Clone)]
pub struct State {
    pub locked: HashSet<String>,
    pub blocks: Board<Option<Block>>,
}

pub enum Actions {
    Delete,
    Empty(Point),
    Change(Point, Point),
    Lock(Vec<Point>),
    UnLock(Vec<Point>),
    Fall,
}

pub struct ActionDispacher<'a> {
    store: &'a mut StoreType,
}

impl<'a> ActionDispacher<'a> {
    pub fn new(store: &'a mut StoreType) -> Self {
        ActionDispacher { store }
    }

    pub fn change(&mut self, from: Point, to: Point, dry: bool) {
        self.store.dispatch(Actions::Change(from, to))
    }

    pub fn unlock(&mut self, points: Vec<Point>) {
        self.store.dispatch(Actions::UnLock(points))
    }

    pub fn lock(&mut self, points: Vec<Point>) {
        self.store.dispatch(Actions::Lock(points))
    }
}

pub fn create_state() -> State {
    State {
        blocks: Board::from(vec![
            vec![Block::a(0, 1), Block::a(1, 2), Block::a(2, 5)],
            vec![Block::a(3, 2), Block::a(4, 4), Block::a(5, 3)],
            vec![Block::a(6, 3), Block::a(7, 4), Block::a(8, 2)],
            vec![Block::a(9, 4), Block::a(10, 4), Block::a(11, 1)],
            vec![Block::a(12, 2), Block::a(13, 5), Block::a(14, 1)],
        ]),
        locked: HashSet::new(),
    }
}

pub fn reducer(state: &State, types: Actions) -> State {
    inspect(&state.blocks);

    match types {
        Actions::Lock(points) => {
            let mut next = state.locked.clone();
            for i in points.into_iter() {
                let mayblock = state.blocks.pick(i);
                if let Some(bl) = mayblock {
                    next.insert(bl.id.to_string());
                }
            }
            State {
                locked: next,
                ..state.clone()
            }
        }
        Actions::UnLock(points) => {
            let mut next = state.locked.clone();
            for i in points.into_iter() {
                let mayblock = state.blocks.pick(i);
                if let Some(bl) = mayblock {
                    next.remove(&bl.id.to_string());
                }
            }
            State {
                locked: next,
                ..state.clone()
            }
        }
        Actions::Change(a, b) => State {
            blocks: change(&state.blocks, a, b),
            ..state.clone()
        },
        _ => state.clone(),
    }
}
