pub type Reducer<T, A> = fn(state: &T, types: A) -> T;

pub struct Store<T, A> {
    pub state: T,
    reducer: Reducer<T, A>,
    cb: Option<fn(state: &T) -> ()>,
}

impl<T, A> Store<T, A> {
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

    pub fn subscribe(&mut self, cb: fn(state: &T) -> ()) {
        self.cb = Some(cb);
    }

    pub fn dispatch(&mut self, action: A) {
        let new_state = (self.reducer)(&self.state, action);

        if let None = self.cb {
            ()
        } else {
            self.cb.unwrap()(&new_state);
        }

        self.state = new_state;
    }
}
