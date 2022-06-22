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

    pub fn will_change(&'a mut self, a: Point, b: Point) {
        self.store.dispatch(Actions::Changing(a, b))
    }

    pub fn change(&'a mut self, a: Point, b: Point) {
        self.store.dispatch(Actions::Change(a, b));
    }

    pub fn move_(&'a mut self, from: Point, to: Point) {
        self.store.dispatch(Actions::Move(from, to))
    }

    pub fn will_fall(&'a mut self, from: Point) {
        self.store.dispatch(Actions::Falling(from))
    }

    pub fn fall(&'a mut self, from: Point, to: Point) {
        self.store.dispatch(Actions::Fall(from, to))
    }

    pub fn will_delete(&'a mut self, delete: Vec<Point>) {
        self.store.dispatch(Actions::Deleting(delete))
    }

    pub fn delete(&'a mut self, delete: Vec<Point>) {
        self.store.dispatch(Actions::Delete(delete.clone()))
    }
}
