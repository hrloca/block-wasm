use super::*;
use crate::board::*;
use crate::tools::store::Store;

pub type StoreType = Store<State, Actions>;

pub struct ActionDispacher<'a> {
    store: &'a StoreType,
}

impl<'a> ActionDispacher<'a> {
    pub fn new(store: &'a StoreType) -> Self {
        ActionDispacher { store }
    }

    pub fn will_change(&'a self, a: Point, b: Point) {
        self.store.dispatch(Actions::Changing(a, b))
    }

    pub fn change(&'a self, a: Point, b: Point) {
        self.store.dispatch(Actions::Change(a, b));
    }

    pub fn move_(&'a self, from: Point, to: Point) {
        self.store.dispatch(Actions::Move(from, to))
    }

    pub fn will_fall(&'a self, from: Point) {
        self.store.dispatch(Actions::Falling(from))
    }

    pub fn fall(&'a self, from: Point, to: Point) {
        self.store.dispatch(Actions::Fall(from, to))
    }

    pub fn will_delete(&'a self, delete: Vec<Point>) {
        self.store.dispatch(Actions::Deleting(delete))
    }

    pub fn delete(&'a self, delete: Vec<Point>) {
        self.store.dispatch(Actions::Delete(delete.clone()))
    }

    pub fn delete_complete(&'a self, id: u64) {
        self.store.dispatch(Actions::DeleteCompleteTask(id))
    }

    pub fn add_complete(&'a self, id: u64) {
        self.store.dispatch(Actions::AddCompleteTask(id))
    }
}
