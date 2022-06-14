use super::*;
use crate::board::*;
use crate::tools::store::Store;

pub type StoreType = Store<State, Actions>;

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
