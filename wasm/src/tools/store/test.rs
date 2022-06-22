#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn 同期的に状態が変更できる() {
        enum ActionTypes {
            Up,
            Down,
        }

        #[derive(Debug, Clone)]
        struct State {
            score: usize,
        }

        let store = Store::create(State { score: 0 }, |state, types| match types {
            ActionTypes::Up => State {
                score: state.score + 1,
            },
            ActionTypes::Down => State {
                score: state.score - 1,
            },
        });

        store.dispatch(ActionTypes::Up);
        assert_eq!(store.get_state().score, 1);

        store.dispatch(ActionTypes::Up);
        assert_eq!(store.get_state().score, 2);

        store.dispatch(ActionTypes::Down);
        store.dispatch(ActionTypes::Down);
        let state = store.get_state();
        assert_eq!(store.get_state().score, 0);
    }
}
