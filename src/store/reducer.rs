use super::*;
use crate::blocks::*;

pub fn reducer(state: &State, types: Actions) -> State {
    inspect(&state.blocks);

    match types {
        Actions::Change(a, b) => {
            let mut next_point = state.changing_point.clone();
            next_point.remove(&a);
            next_point.remove(&b);

            State {
                changing_point: next_point,
                blocks: change(&state.blocks, a, b),
                ..state.clone()
            }
        }

        Actions::Changing(a, b) => {
            let mut next_point = state.changing_point.clone();
            next_point.insert(a);
            next_point.insert(b);

            State {
                changing_point: next_point,
                ..state.clone()
            }
        }

        Actions::Deleting(dels) => {
            let mut next_point = state.deleting_point.clone();
            for i in dels.into_iter() {
                next_point.insert(i);
            }

            State {
                deleting_point: next_point,
                ..state.clone()
            }
        }

        Actions::Delete(dels) => {
            let mut next_point = state.deleting_point.clone();
            for i in dels.iter() {
                next_point.remove(i);
            }
            State {
                deleting_point: next_point,
                blocks: delete(&state.blocks, &dels),
                ..state.clone()
            }
        }

        Actions::Falling(from, to) => {
            let mut next_point = state.falling_point.clone();
            next_point.insert(from);
            next_point.insert(to);

            State {
                falling_point: next_point,
                ..state.clone()
            }
        }

        Actions::Fall(from, to) => {
            let mut next_point = state.falling_point.clone();
            next_point.remove(&from);
            next_point.remove(&to);
            State {
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

        Actions::NextQueueName(name) => {
            let mut next_queue_task = state.next_queue_task.clone();
            next_queue_task.insert(name);
            State {
                next_queue_task,
                ..state.clone()
            }
        }

        Actions::DeleteNextQueueName(name) => {
            let mut next_queue_task = state.next_queue_task.clone();
            next_queue_task.remove(&name);
            State {
                next_queue_task,
                ..state.clone()
            }
        }

        Actions::DeleteActiveQueueName(name) => {
            let mut active_queue_task = state.active_queue_task.clone();
            active_queue_task.remove(&name);
            State {
                active_queue_task,
                ..state.clone()
            }
        }

        Actions::ActiveQueueName(name) => {
            let mut active_queue_task = state.active_queue_task.clone();
            active_queue_task.insert(name);
            State {
                active_queue_task,
                ..state.clone()
            }
        }

        _ => state.clone(),
    }
}
