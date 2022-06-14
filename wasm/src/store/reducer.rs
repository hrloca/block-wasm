use super::*;
use crate::blocks::*;

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
