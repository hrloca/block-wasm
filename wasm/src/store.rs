use crate::blocks::*;
use crate::board::*;
use crate::log;
use crate::tools::store::Store;
use std::collections::HashSet;

pub type StoreType = Store<State, Actions>;

#[derive(Debug, Clone)]
pub struct State {
    pub locked: HashSet<String>,
    pub blocks: Board<Option<Block>>,
}

pub enum Actions {
    Delete(Vec<Point>),
    Empty(Point),
    Change(Point, Point),
    Lock(Vec<Point>),
    UnLock(Vec<Point>),
    Move(Point, Point),
}

pub struct ActionDispacher<'a> {
    store: &'a mut StoreType,
}

impl<'a> ActionDispacher<'a> {
    pub fn new(store: &'a mut StoreType) -> Self {
        ActionDispacher { store }
    }

    pub fn change(&mut self, a: Point, b: Point) {
        self.unlock(vec![a, b]);
        self.store.dispatch(Actions::Change(a, b))
    }

    pub fn move_(&mut self, from: Point, to: Point) {
        self.unlock(vec![from]);
        self.store.dispatch(Actions::Move(from, to))
    }

    pub fn delete(&mut self, delete: Vec<Point>) {
        self.unlock(delete.clone());
        self.store.dispatch(Actions::Delete(delete.clone()))
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
            vec![Block::a(3, 2), Block::a(4, 3), Block::a(5, 4)],
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
        Actions::Delete(dels) => State {
            blocks: delete(&state.blocks, &dels),
            ..state.clone()
        },
        Actions::Move(from, to) => State {
            blocks: move_to(&state.blocks, &vec![Move { from, to }]),
            ..state.clone()
        },
        _ => state.clone(),
    }
}
