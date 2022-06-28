use super::*;
use crate::blocks::*;

pub fn reducer(state: &State, types: Actions) -> State {
    inspect(&state.blocks);

    match types {
        Actions::Change(a, b) => {
            let mut next = state.changing.clone();
            let mut next_point = state.changing_point.clone();
            if let Some(a) = state.blocks.pick(a) {
                next.remove(&a.id.to_string());
            }

            if let Some(b) = state.blocks.pick(b) {
                next.remove(&b.id.to_string());
            }

            next_point.remove(&a);
            next_point.remove(&b);

            State {
                changing: next,
                changing_point: next_point,
                blocks: change(&state.blocks, a, b),
                ..state.clone()
            }
        }

        Actions::Changing(a, b) => {
            let mut next = state.changing.clone();
            let mut next_point = state.changing_point.clone();
            if let Some(a) = state.blocks.pick(a) {
                next.insert(a.id.to_string());
            }
            if let Some(b) = state.blocks.pick(b) {
                next.insert(b.id.to_string());
            }

            next_point.insert(a);
            next_point.insert(b);

            State {
                changing_point: next_point,
                changing: next,
                ..state.clone()
            }
        }

        Actions::Deleting(dels) => {
            let mut next = state.deleting.clone();
            let mut next_point = state.deleting_point.clone();
            for i in dels.into_iter() {
                next_point.insert(i);
                let mayblock = state.blocks.pick(i);
                if let Some(bl) = mayblock {
                    next.insert(bl.id.to_string());
                }
            }

            State {
                deleting_point: next_point,
                deleting: next,
                ..state.clone()
            }
        }

        Actions::Delete(dels) => {
            let mut next = state.deleting.clone();
            let mut next_point = state.deleting_point.clone();
            for i in dels.iter() {
                next_point.remove(i);
                let mayblock = state.blocks.pick(*i);
                if let Some(bl) = mayblock {
                    next.remove(&bl.id.to_string());
                }
            }
            State {
                deleting_point: next_point,
                deleting: next,
                blocks: delete(&state.blocks, &dels),
                ..state.clone()
            }
        }

        Actions::Falling(from, to) => {
            let mut next = state.falling.clone();
            let mut next_point = state.falling_point.clone();
            if let Some(block) = state.blocks.pick(from) {
                next.insert(block.id.to_string());
            }

            next_point.insert(from);
            next_point.insert(to);

            State {
                falling: next,
                falling_point: next_point,
                ..state.clone()
            }
        }

        Actions::Fall(from, to) => {
            let mut next = state.falling.clone();
            let mut next_point = state.falling_point.clone();
            let mayblock = state.blocks.pick(from);
            if let Some(bl) = mayblock {
                next.remove(&bl.id.to_string());
            }
            next_point.remove(&from);
            next_point.remove(&to);
            State {
                falling: next,
                falling_point: next_point,
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

        Actions::QueueTask(id) => State {
            active_queue_task: Some(id),
            ..state.clone()
        },

        Actions::DeleteQueueTask => State {
            active_queue_task: None,
            ..state.clone()
        },

        _ => state.clone(),
    }
}
