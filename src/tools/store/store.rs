use std::cell::RefCell;

pub type Reducer<T, A> = fn(state: &T, types: A) -> T;

pub struct Store<T: Clone, A> {
    pub state: RefCell<T>,
    reducer: Reducer<T, A>,
    f: RefCell<Option<Box<dyn Fn(&T) -> ()>>>,
}

impl<T: Clone, A> Store<T, A> {
    pub fn create(state: T, reducer: Reducer<T, A>) -> Store<T, A> {
        Store {
            reducer,
            state: RefCell::new(state),
            f: RefCell::new(None),
        }
    }

    pub fn initialize_state(&self, state: T) {
        *self.state.borrow_mut() = state;
    }

    pub fn subscribe(&self, cb: Box<dyn Fn(&T) -> ()>) {
        *self.f.borrow_mut() = Some(cb);
    }

    pub fn get_state(&self) -> T {
        let rf = self.state.borrow();
        rf.clone()
    }

    pub fn dispatch(&self, action: A) {
        let new_state = (self.reducer)(&*self.state.borrow(), action);

        if let Some(f) = &*self.f.borrow() {
            f(&new_state)
        }

        *self.state.borrow_mut() = new_state;
    }
}
