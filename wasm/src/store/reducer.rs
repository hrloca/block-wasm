use super::*;
use crate::blocks::*;

pub fn reducer(state: &State, types: Actions) -> State {
    inspect(&state.blocks);

    match types {
        Actions::Change(a, b) => {
            let mut next = state.changing.clone();
            if let Some(a) = state.blocks.pick(a) {
                next.remove(&a.id.to_string());
            }

            if let Some(b) = state.blocks.pick(b) {
                next.remove(&b.id.to_string());
            }

            State {
                changing: next,
                blocks: change(&state.blocks, a, b),
                ..state.clone()
            }
        }

        Actions::Changing(a, b) => {
            let mut next = state.changing.clone();
            if let Some(a) = state.blocks.pick(a) {
                next.insert(a.id.to_string());
            }
            if let Some(b) = state.blocks.pick(b) {
                next.insert(b.id.to_string());
            }

            State {
                changing: next,
                ..state.clone()
            }
        }

        Actions::Deleting(dels) => {
            let mut next = state.deleting.clone();
            for i in dels.into_iter() {
                let mayblock = state.blocks.pick(i);
                if let Some(bl) = mayblock {
                    next.insert(bl.id.to_string());
                }
            }

            State {
                deleting: next,
                ..state.clone()
            }
        }

        Actions::Delete(dels) => {
            let mut next = state.deleting.clone();
            for i in dels.iter() {
                let mayblock = state.blocks.pick(*i);
                if let Some(bl) = mayblock {
                    next.remove(&bl.id.to_string());
                }
            }
            State {
                deleting: next,
                blocks: delete(&state.blocks, &dels),
                ..state.clone()
            }
        }

        Actions::Falling(a) => {
            let mut next = state.falling.clone();
            if let Some(a) = state.blocks.pick(a) {
                next.insert(a.id.to_string());
            }
            State {
                falling: next,
                ..state.clone()
            }
        }

        Actions::Fall(from, to) => {
            let mut next = state.falling.clone();
            let mayblock = state.blocks.pick(from);
            if let Some(bl) = mayblock {
                next.remove(&bl.id.to_string());
            }
            State {
                falling: next,
                blocks: move_to(&state.blocks, &vec![Move { from, to }]),
                ..state.clone()
            }
        }

        Actions::AddCompleteTask(id) => {
            let mut next = state.complete_tasks.clone();
            next.push(id);

            State {
                complete_tasks: next,
                ..state.clone()
            }
        }

        Actions::DeleteCompleteTask(id) => {
            let mut next = state.complete_tasks.clone();
            next.retain(|&x| x != id);
            State {
                complete_tasks: next,
                ..state.clone()
            }
        }

        _ => state.clone(),
    }
}
