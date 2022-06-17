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

    pub fn will_change(&mut self, a: Point, b: Point) {
        self.store.dispatch(Actions::Changing(a, b))
    }

    pub fn change(&mut self, a: Point, b: Point) {
        self.store.dispatch(Actions::Change(a, b))
    }

    pub fn move_(&mut self, from: Point, to: Point) {
        self.store.dispatch(Actions::Move(from, to))
    }

    pub fn will_fall(&mut self, from: Point) {
        self.store.dispatch(Actions::Falling(from))
    }

    pub fn fall(&mut self, from: Point, to: Point) {
        self.store.dispatch(Actions::Fall(from, to))
    }

    pub fn will_delete(&mut self, delete: Vec<Point>) {
        self.store.dispatch(Actions::Deleting(delete))
    }

    pub fn delete(&mut self, delete: Vec<Point>) {
        self.store.dispatch(Actions::Delete(delete.clone()))
    }
}
