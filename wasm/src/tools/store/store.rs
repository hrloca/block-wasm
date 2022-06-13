pub type Reducer<T, A> = fn(state: &T, types: A) -> T;

pub struct Store<T: Clone, A> {
    pub state: T,
    reducer: Reducer<T, A>,
    cb: Option<Box<dyn Fn(&T) -> ()>>,
}

impl<T: Clone, A> Store<T, A> {
    pub fn create(state: T, reducer: Reducer<T, A>) -> Store<T, A> {
        Store {
            cb: None,
            reducer,
            state,
        }
    }

    pub fn initialize_state(&mut self, state: T) {
        self.state = state;
    }

    pub fn subscribe(&mut self, cb: Box<dyn Fn(&T) -> ()>) {
        self.cb = Some(cb);
    }

    pub fn get_state(&self) -> T {
        self.state.clone()
    }

    pub fn dispatch(&mut self, action: A) {
        let new_state = (self.reducer)(&self.state, action);

        match &self.cb {
            None => (),
            Some(x) => x(&new_state),
        }

        self.state = new_state;
    }
}
